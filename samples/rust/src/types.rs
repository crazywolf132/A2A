use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents the state of a task within the A2A protocol.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum TaskState {
    Submitted,
    Working,
    InputRequired,
    Completed,
    Canceled,
    Failed,
    Rejected,
    Unknown,
}

/// Represents a part of a message containing text content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPart {
    #[serde(rename = "type")]
    pub part_type: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl Default for TextPart {
    fn default() -> Self {
        Self {
            part_type: "text".to_string(),
            text: String::new(),
            metadata: None,
        }
    }
}

/// Represents the content of a file, either as base64 encoded bytes or a URI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// Represents a part of a message containing file content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePart {
    #[serde(rename = "type")]
    pub part_type: String,
    pub file: FileContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl Default for FilePart {
    fn default() -> Self {
        Self {
            part_type: "file".to_string(),
            file: FileContent {
                name: None,
                mime_type: None,
                bytes: None,
                uri: None,
            },
            metadata: None,
        }
    }
}

/// Represents a part of a message containing structured data (JSON).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPart {
    #[serde(rename = "type")]
    pub part_type: String,
    pub data: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl Default for DataPart {
    fn default() -> Self {
        Self {
            part_type: "data".to_string(),
            data: HashMap::new(),
            metadata: None,
        }
    }
}

/// Represents a single part of a multi-part message. Can be text, file, or data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Part {
    Text(TextPart),
    File(FilePart),
    Data(DataPart),
}

/// Represents a message exchanged between a user and an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub parts: Vec<Part>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Represents the status of a task at a specific point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatus {
    pub state: TaskState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    pub timestamp: DateTime<Utc>,
}

/// Represents an artifact generated or used by a task, potentially composed of multiple parts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub parts: Vec<Part>,
    #[serde(default)]
    pub index: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_chunk: Option<bool>,
}

/// Represents a task being processed by an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    pub status: TaskStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<Artifact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Represents a task status update event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatusUpdateEvent {
    pub id: String,
    pub status: TaskStatus,
    #[serde(default)]
    pub final_status: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Represents a task artifact update event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskArtifactUpdateEvent {
    pub id: String,
    pub artifact: Artifact,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Base JSON-RPC message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcMessage {
    pub jsonrpc: String,
    pub id: Option<String>,
}

impl Default for JsonRpcMessage {
    fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Some(Uuid::new_v4().to_string()),
        }
    }
}

/// JSON-RPC error structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// JSON-RPC response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse<T> {
    pub jsonrpc: String,
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

impl<T> Default for JsonRpcResponse<T> {
    fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Some(Uuid::new_v4().to_string()),
            result: None,
            error: None,
        }
    }
}

/// Parameters for task operations that require only a task ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskIdParams {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for querying a task, extending TaskIdParams
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskQueryParams {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for sending a message to a task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskSendParams {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    pub message: Message,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Request to send a message to a task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTaskRequest {
    pub jsonrpc: String,
    pub id: Option<String>,
    pub method: String,
    pub params: TaskSendParams,
}

impl Default for SendTaskRequest {
    fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Some(Uuid::new_v4().to_string()),
            method: "tasks/send".to_string(),
            params: TaskSendParams {
                id: Uuid::new_v4().to_string(),
                session_id: Some(Uuid::new_v4().to_string()),
                message: Message {
                    role: "user".to_string(),
                    parts: vec![],
                    metadata: None,
                },
                history_length: None,
                metadata: None,
            },
        }
    }
}

/// Request to get a task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTaskRequest {
    pub jsonrpc: String,
    pub id: Option<String>,
    pub method: String,
    pub params: TaskQueryParams,
}

impl Default for GetTaskRequest {
    fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Some(Uuid::new_v4().to_string()),
            method: "tasks/get".to_string(),
            params: TaskQueryParams {
                id: Uuid::new_v4().to_string(),
                history_length: None,
                metadata: None,
            },
        }
    }
}

/// Request to cancel a task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelTaskRequest {
    pub jsonrpc: String,
    pub id: Option<String>,
    pub method: String,
    pub params: TaskIdParams,
}

impl Default for CancelTaskRequest {
    fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Some(Uuid::new_v4().to_string()),
            method: "tasks/cancel".to_string(),
            params: TaskIdParams {
                id: Uuid::new_v4().to_string(),
                metadata: None,
            },
        }
    }
}
