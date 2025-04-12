use crate::agents::openai_agent::OpenAIAgent;
use crate::error::{A2AError, A2AResult};
use crate::store::TaskStore;
use crate::types::{
    CancelTaskRequest, GetTaskRequest, JsonRpcResponse, SendTaskRequest, Task,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

/// Application state for the OpenAI agent server
#[derive(Clone)]
pub struct AppState {
    pub task_store: TaskStore,
    pub agent: Arc<Mutex<OpenAIAgent>>,
}

/// Error response
impl IntoResponse for A2AError {
    fn into_response(self) -> Response {
        let (status, error_code, error_message) = match &self {
            A2AError::TaskNotFound(id) => (
                StatusCode::NOT_FOUND,
                -32001,
                format!("Task not found: {}", id),
            ),
            A2AError::InvalidRequest(msg) => (
                StatusCode::BAD_REQUEST,
                -32600,
                format!("Invalid request: {}", msg),
            ),
            A2AError::MethodNotFound(method) => (
                StatusCode::NOT_FOUND,
                -32601,
                format!("Method not found: {}", method),
            ),
            A2AError::TaskNotCancelable(msg) => (
                StatusCode::BAD_REQUEST,
                -32002,
                format!("Task cannot be canceled: {}", msg),
            ),
            A2AError::UnsupportedOperation(msg) => (
                StatusCode::BAD_REQUEST,
                -32004,
                format!("Unsupported operation: {}", msg),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                -32603,
                format!("Internal server error: {}", self),
            ),
        };

        let error = crate::types::JsonRpcError {
            code: error_code,
            message: error_message,
            data: None,
        };

        let response = JsonRpcResponse::<()> {
            jsonrpc: "2.0".to_string(),
            id: None,
            result: None,
            error: Some(error),
        };

        (status, Json(response)).into_response()
    }
}

/// Handle JSON-RPC requests
async fn handle_request(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, A2AError> {
    let method = payload["method"]
        .as_str()
        .ok_or_else(|| A2AError::InvalidRequest("Missing method".to_string()))?;

    match method {
        "tasks/send" => {
            let request: SendTaskRequest = serde_json::from_value(payload.clone())?;
            let request_id = request.id.clone();
            
            // Create a new task
            let task_id = request.params.id.clone();
            let session_id = request.params.session_id.clone();
            let message = request.params.message.clone();
            
            let task = Task {
                id: task_id,
                session_id,
                status: crate::types::TaskStatus {
                    state: crate::types::TaskState::Submitted,
                    message: Some(message),
                    timestamp: chrono::Utc::now(),
                },
                artifacts: None,
                metadata: None,
            };
            
            // Store the task
            let created_task = state.task_store.create_task(task.clone())?;
            
            // Process the task with the OpenAI agent
            let agent = state.agent.lock().await;
            let processed_task = agent.handle_task(created_task).await?;
            
            // Update the task in the store
            let updated_task = state.task_store.update_task_status(
                &processed_task.id,
                processed_task.status.clone(),
            )?;
            
            // Add artifacts if any
            if let Some(artifacts) = processed_task.artifacts {
                for artifact in artifacts {
                    state.task_store.add_artifact(&updated_task.id, artifact)?;
                }
            }
            
            // Get the final task
            let final_task = state.task_store.get_task(&updated_task.id)?;
            
            let response = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request_id,
                result: Some(final_task),
                error: None,
            };
            
            Ok(Json(serde_json::to_value(response)?))
        }
        "tasks/get" => {
            let request: GetTaskRequest = serde_json::from_value(payload.clone())?;
            let request_id = request.id.clone();
            let task = state.task_store.get_task(&request.params.id)?;
            
            let response = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request_id,
                result: Some(task),
                error: None,
            };
            
            Ok(Json(serde_json::to_value(response)?))
        }
        "tasks/cancel" => {
            let request: CancelTaskRequest = serde_json::from_value(payload.clone())?;
            let request_id = request.id.clone();
            let task = state.task_store.cancel_task(&request.params.id)?;
            
            let response = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request_id,
                result: Some(task),
                error: None,
            };
            
            Ok(Json(serde_json::to_value(response)?))
        }
        _ => Err(A2AError::MethodNotFound(method.to_string())),
    }
}

/// Create the Axum router for the OpenAI agent server
pub fn create_router(agent: OpenAIAgent) -> Router {
    let task_store = TaskStore::new();
    let app_state = Arc::new(AppState {
        task_store,
        agent: Arc::new(Mutex::new(agent)),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/", post(handle_request))
        .layer(cors)
        .with_state(app_state)
}
