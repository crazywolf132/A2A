#[cfg(test)]
mod tests {
    use crate::types::{
        Message, Part, Task, TaskState, TaskStatus, TextPart,
    };
    use chrono::Utc;


    #[test]
    fn test_task_state_serialization() {
        let states = vec![
            TaskState::Submitted,
            TaskState::Working,
            TaskState::InputRequired,
            TaskState::Completed,
            TaskState::Canceled,
            TaskState::Failed,
            TaskState::Rejected,
            TaskState::Unknown,
        ];

        for state in states {
            let serialized = serde_json::to_string(&state).unwrap();
            let deserialized: TaskState = serde_json::from_str(&serialized).unwrap();
            assert_eq!(state, deserialized);
        }
    }

    #[test]
    fn test_message_serialization() {
        let message = Message {
            role: "user".to_string(),
            parts: vec![Part::Text(TextPart {
                part_type: "text".to_string(),
                text: "Hello, world!".to_string(),
                metadata: None,
            })],
            metadata: None,
        };

        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: Message = serde_json::from_str(&serialized).unwrap();

        assert_eq!(message.role, deserialized.role);
        assert_eq!(message.parts.len(), deserialized.parts.len());

        if let Part::Text(text_part) = &message.parts[0] {
            if let Part::Text(deserialized_text_part) = &deserialized.parts[0] {
                assert_eq!(text_part.text, deserialized_text_part.text);
            } else {
                panic!("Deserialized part is not a TextPart");
            }
        } else {
            panic!("Original part is not a TextPart");
        }
    }

    #[test]
    fn test_task_serialization() {
        let task = Task {
            id: "task-123".to_string(),
            session_id: Some("session-456".to_string()),
            status: TaskStatus {
                state: TaskState::Completed,
                message: Some(Message {
                    role: "agent".to_string(),
                    parts: vec![Part::Text(TextPart {
                        part_type: "text".to_string(),
                        text: "Task completed successfully".to_string(),
                        metadata: None,
                    })],
                    metadata: None,
                }),
                timestamp: Utc::now(),
            },
            artifacts: None,
            metadata: None,
        };

        let serialized = serde_json::to_string(&task).unwrap();
        let deserialized: Task = serde_json::from_str(&serialized).unwrap();

        assert_eq!(task.id, deserialized.id);
        assert_eq!(task.session_id, deserialized.session_id);
        assert_eq!(task.status.state, deserialized.status.state);

        if let Some(message) = &task.status.message {
            if let Some(deserialized_message) = &deserialized.status.message {
                assert_eq!(message.role, deserialized_message.role);
            } else {
                panic!("Deserialized message is None");
            }
        } else {
            panic!("Original message is None");
        }
    }
}

#[cfg(test)]
mod store_tests {
    use crate::store::TaskStore;
    use crate::types::{Message, Part, Task, TaskState, TaskStatus, TextPart};
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_task_store_create_get() {
        let store = TaskStore::new();
        let task_id = Uuid::new_v4().to_string();

        let task = Task {
            id: task_id.clone(),
            session_id: Some(Uuid::new_v4().to_string()),
            status: TaskStatus {
                state: TaskState::Submitted,
                message: None,
                timestamp: Utc::now(),
            },
            artifacts: None,
            metadata: None,
        };

        // Create task
        let created_task = store.create_task(task.clone()).unwrap();
        assert_eq!(created_task.id, task_id);

        // Get task
        let retrieved_task = store.get_task(&task_id).unwrap();
        assert_eq!(retrieved_task.id, task_id);
        assert_eq!(retrieved_task.status.state, TaskState::Submitted);
    }

    #[test]
    fn test_task_store_update_status() {
        let store = TaskStore::new();
        let task_id = Uuid::new_v4().to_string();

        let task = Task {
            id: task_id.clone(),
            session_id: Some(Uuid::new_v4().to_string()),
            status: TaskStatus {
                state: TaskState::Submitted,
                message: None,
                timestamp: Utc::now(),
            },
            artifacts: None,
            metadata: None,
        };

        // Create task
        store.create_task(task.clone()).unwrap();

        // Update status
        let new_status = TaskStatus {
            state: TaskState::Working,
            message: Some(Message {
                role: "agent".to_string(),
                parts: vec![Part::Text(TextPart {
                    part_type: "text".to_string(),
                    text: "Working on it...".to_string(),
                    metadata: None,
                })],
                metadata: None,
            }),
            timestamp: Utc::now(),
        };

        let updated_task = store.update_task_status(&task_id, new_status).unwrap();
        assert_eq!(updated_task.id, task_id);
        assert_eq!(updated_task.status.state, TaskState::Working);

        // Verify the update
        let retrieved_task = store.get_task(&task_id).unwrap();
        assert_eq!(retrieved_task.status.state, TaskState::Working);

        if let Some(message) = retrieved_task.status.message {
            if let Part::Text(text_part) = &message.parts[0] {
                assert_eq!(text_part.text, "Working on it...");
            } else {
                panic!("Retrieved part is not a TextPart");
            }
        } else {
            panic!("Retrieved message is None");
        }
    }

    #[test]
    fn test_task_store_cancel() {
        let store = TaskStore::new();
        let task_id = Uuid::new_v4().to_string();

        let task = Task {
            id: task_id.clone(),
            session_id: Some(Uuid::new_v4().to_string()),
            status: TaskStatus {
                state: TaskState::Working,
                message: None,
                timestamp: Utc::now(),
            },
            artifacts: None,
            metadata: None,
        };

        // Create task
        store.create_task(task.clone()).unwrap();

        // Cancel task
        let canceled_task = store.cancel_task(&task_id).unwrap();
        assert_eq!(canceled_task.id, task_id);
        assert_eq!(canceled_task.status.state, TaskState::Canceled);

        // Verify the cancellation
        let retrieved_task = store.get_task(&task_id).unwrap();
        assert_eq!(retrieved_task.status.state, TaskState::Canceled);
    }
}

// We'll implement integration tests separately in a more appropriate way
// For now, we'll focus on unit tests for the core types and store
