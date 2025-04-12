use crate::error::{A2AError, A2AResult};
use crate::store::TaskStore;
use crate::types::{
    Artifact, CancelTaskRequest, GetTaskRequest, JsonRpcError, JsonRpcResponse, Message, Part,
    SendTaskRequest, Task, TaskState, TaskStatus, TextPart,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use chrono::Utc;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};


/// Application state
#[derive(Clone)]
pub struct AppState {
    pub task_store: TaskStore,
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

        let error = JsonRpcError {
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
            let task = handle_send_task(state, request).await?;
            let response = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request_id,
                result: Some(task),
                error: None,
            };
            Ok(Json(serde_json::to_value(response)?))
        }
        "tasks/get" => {
            let request: GetTaskRequest = serde_json::from_value(payload.clone())?;
            let request_id = request.id.clone();
            let task = handle_get_task(state, request).await?;
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
            let task = handle_cancel_task(state, request).await?;
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

/// Handle send task request
async fn handle_send_task(
    state: Arc<AppState>,
    request: SendTaskRequest,
) -> A2AResult<Task> {
    let task_id = request.params.id.clone();
    let session_id = request.params.session_id.clone();
    let message = request.params.message.clone();

    // Check if task exists
    let existing_task = state.task_store.get_task(&task_id);

    match existing_task {
        Ok(task) => {
            // Task exists, update it
            if task.status.state == TaskState::InputRequired {
                // Process the new message
                let response_message = process_message(&message)?;

                // Update task status
                let new_status = TaskStatus {
                    state: TaskState::Completed,
                    message: Some(response_message),
                    timestamp: Utc::now(),
                };

                state.task_store.update_task_status(&task_id, new_status)
            } else {
                Err(A2AError::InvalidRequest(format!(
                    "Task {} is not in input-required state",
                    task_id
                )))
            }
        }
        Err(_) => {
            // Task doesn't exist, create a new one
            let response_message = process_message(&message)?;

            // Create a new task
            let task = Task {
                id: task_id,
                session_id,
                status: TaskStatus {
                    state: TaskState::Completed,
                    message: Some(response_message.clone()),
                    timestamp: Utc::now(),
                },
                artifacts: Some(vec![Artifact {
                    name: Some("result".to_string()),
                    description: Some("Task result".to_string()),
                    parts: vec![Part::Text(TextPart {
                        part_type: "text".to_string(),
                        text: "This is a sample artifact from the Rust A2A server.".to_string(),
                        metadata: None,
                    })],
                    index: 0,
                    append: None,
                    metadata: None,
                    last_chunk: Some(true),
                }]),
                metadata: None,
            };

            state.task_store.create_task(task)
        }
    }
}

/// Handle get task request
async fn handle_get_task(
    state: Arc<AppState>,
    request: GetTaskRequest,
) -> A2AResult<Task> {
    let task_id = request.params.id.clone();
    state.task_store.get_task(&task_id)
}

/// Handle cancel task request
async fn handle_cancel_task(
    state: Arc<AppState>,
    request: CancelTaskRequest,
) -> A2AResult<Task> {
    let task_id = request.params.id.clone();
    state.task_store.cancel_task(&task_id)
}

/// Process a message and generate a response
fn process_message(message: &Message) -> A2AResult<Message> {
    // Extract text from the message
    let text = message
        .parts
        .iter()
        .filter_map(|part| match part {
            Part::Text(text_part) => Some(text_part.text.clone()),
            _ => None,
        })
        .collect::<Vec<String>>()
        .join(" ");

    // Generate a simple response
    let response_text = format!("Rust A2A server received: {}", text);

    Ok(Message {
        role: "agent".to_string(),
        parts: vec![Part::Text(TextPart {
            part_type: "text".to_string(),
            text: response_text,
            metadata: None,
        })],
        metadata: None,
    })
}

/// Create the Axum router
pub fn create_router() -> Router {
    let task_store = TaskStore::new();
    let app_state = Arc::new(AppState { task_store });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/", post(handle_request))
        .layer(cors)
        .with_state(app_state)
}
