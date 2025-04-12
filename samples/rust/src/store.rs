use crate::error::{A2AError, A2AResult};
use crate::types::{Artifact, Task, TaskState, TaskStatus};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

/// A simple in-memory store for tasks
#[derive(Clone)]
pub struct TaskStore {
    tasks: Arc<Mutex<HashMap<String, Task>>>,
    task_updates: broadcast::Sender<Task>,
}

impl TaskStore {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
            task_updates: tx,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Task> {
        self.task_updates.subscribe()
    }

    pub fn get_task(&self, id: &str) -> A2AResult<Task> {
        let tasks = self.tasks.lock().unwrap();
        tasks
            .get(id)
            .cloned()
            .ok_or_else(|| A2AError::TaskNotFound(id.to_string()))
    }

    pub fn create_task(&self, task: Task) -> A2AResult<Task> {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.insert(task.id.clone(), task.clone());
        let _ = self.task_updates.send(task.clone());
        Ok(task)
    }

    pub fn update_task_status(&self, id: &str, status: TaskStatus) -> A2AResult<Task> {
        let mut tasks = self.tasks.lock().unwrap();
        let task = tasks
            .get_mut(id)
            .ok_or_else(|| A2AError::TaskNotFound(id.to_string()))?;
        
        task.status = status;
        let updated_task = task.clone();
        let _ = self.task_updates.send(updated_task.clone());
        Ok(updated_task)
    }

    pub fn add_artifact(&self, id: &str, artifact: Artifact) -> A2AResult<Task> {
        let mut tasks = self.tasks.lock().unwrap();
        let task = tasks
            .get_mut(id)
            .ok_or_else(|| A2AError::TaskNotFound(id.to_string()))?;
        
        if task.artifacts.is_none() {
            task.artifacts = Some(vec![]);
        }
        
        if let Some(artifacts) = &mut task.artifacts {
            artifacts.push(artifact);
        }
        
        let updated_task = task.clone();
        let _ = self.task_updates.send(updated_task.clone());
        Ok(updated_task)
    }

    pub fn cancel_task(&self, id: &str) -> A2AResult<Task> {
        let mut tasks = self.tasks.lock().unwrap();
        let task = tasks
            .get_mut(id)
            .ok_or_else(|| A2AError::TaskNotFound(id.to_string()))?;
        
        // Only tasks in certain states can be canceled
        match task.status.state {
            TaskState::Submitted | TaskState::Working | TaskState::InputRequired => {
                task.status = TaskStatus {
                    state: TaskState::Canceled,
                    message: task.status.message.clone(),
                    timestamp: Utc::now(),
                };
                let updated_task = task.clone();
                let _ = self.task_updates.send(updated_task.clone());
                Ok(updated_task)
            }
            _ => Err(A2AError::TaskNotCancelable(format!(
                "Task {} cannot be canceled in state {:?}",
                id, task.status.state
            ))),
        }
    }
}
