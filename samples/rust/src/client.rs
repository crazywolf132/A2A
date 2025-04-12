use crate::error::{A2AError, A2AResult};
use crate::types::{
    CancelTaskRequest, GetTaskRequest, JsonRpcResponse, Message, Part, SendTaskRequest, Task,
    TaskIdParams, TaskQueryParams, TaskSendParams, TextPart,
};
use reqwest::Client as HttpClient;
use serde::de::DeserializeOwned;
use uuid::Uuid;

/// A client for interacting with an A2A server
pub struct A2AClient {
    http_client: HttpClient,
    base_url: String,
}

impl A2AClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            http_client: HttpClient::new(),
            base_url: base_url.to_string(),
        }
    }

    async fn send_request<T, R>(&self, request: T) -> A2AResult<R>
    where
        T: serde::Serialize,
        R: DeserializeOwned,
    {
        let response = self
            .http_client
            .post(&self.base_url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(A2AError::InvalidRequest(format!(
                "HTTP error: {}", response.status()
            )));
        }

        let json_response: JsonRpcResponse<R> = response.json().await?;

        if let Some(error) = json_response.error {
            return Err(A2AError::InvalidRequest(error.message));
        }

        json_response
            .result
            .ok_or_else(|| A2AError::InvalidRequest("No result in response".to_string()))
    }

    /// Send a message to a task
    pub async fn send_task(&self, message: &str) -> A2AResult<Task> {
        let task_id = Uuid::new_v4().to_string();
        let session_id = Uuid::new_v4().to_string();

        let text_part = TextPart {
            part_type: "text".to_string(),
            text: message.to_string(),
            metadata: None,
        };

        let request = SendTaskRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(Uuid::new_v4().to_string()),
            method: "tasks/send".to_string(),
            params: TaskSendParams {
                id: task_id,
                session_id: Some(session_id),
                message: Message {
                    role: "user".to_string(),
                    parts: vec![Part::Text(text_part)],
                    metadata: None,
                },
                history_length: None,
                metadata: None,
            },
        };

        self.send_request::<SendTaskRequest, Task>(request).await
    }

    /// Get a task by ID
    pub async fn get_task(&self, task_id: &str) -> A2AResult<Task> {
        let request = GetTaskRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(Uuid::new_v4().to_string()),
            method: "tasks/get".to_string(),
            params: TaskQueryParams {
                id: task_id.to_string(),
                history_length: None,
                metadata: None,
            },
        };

        self.send_request::<GetTaskRequest, Task>(request).await
    }

    /// Cancel a task
    pub async fn cancel_task(&self, task_id: &str) -> A2AResult<Task> {
        let request = CancelTaskRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(Uuid::new_v4().to_string()),
            method: "tasks/cancel".to_string(),
            params: TaskIdParams {
                id: task_id.to_string(),
                metadata: None,
            },
        };

        self.send_request::<CancelTaskRequest, Task>(request).await
    }
}
