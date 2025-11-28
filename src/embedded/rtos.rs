// RTOS Integration - FreeRTOS and Zephyr support

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq)]
pub enum RtosType {
    FreeRTOS,
    Zephyr,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum TaskPriority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

#[derive(Debug, Clone)]
pub struct TaskConfig {
    pub name: String,
    pub priority: TaskPriority,
    pub stack_size: usize,
    pub deadline_ms: Option<u64>, // Real-time deadline in milliseconds
}

pub struct Task {
    config: TaskConfig,
    state: TaskState,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Suspended,
}

impl Task {
    pub fn new(config: TaskConfig) -> Self {
        Task {
            config,
            state: TaskState::Ready,
        }
    }

    pub fn get_priority(&self) -> &TaskPriority {
        &self.config.priority
    }

    pub fn get_state(&self) -> &TaskState {
        &self.state
    }

    pub fn suspend(&mut self) {
        self.state = TaskState::Suspended;
    }

    pub fn resume(&mut self) {
        self.state = TaskState::Ready;
    }
}

pub struct RtosScheduler {
    rtos_type: RtosType,
    tasks: Arc<Mutex<HashMap<String, Task>>>,
    current_task: Arc<Mutex<Option<String>>>,
}

impl RtosScheduler {
    pub fn new(rtos_type: RtosType) -> Self {
        RtosScheduler {
            rtos_type,
            tasks: Arc::new(Mutex::new(HashMap::new())),
            current_task: Arc::new(Mutex::new(None)),
        }
    }

    pub fn create_task(&self, config: TaskConfig) -> Result<(), String> {
        let task_name = config.name.clone();
        let task = Task::new(config);

        let mut tasks = self.tasks.lock().unwrap();
        if tasks.contains_key(&task_name) {
            return Err(format!("Task '{}' already exists", task_name));
        }

        tasks.insert(task_name, task);
        Ok(())
    }

    pub fn delete_task(&self, task_name: &str) -> Result<(), String> {
        let mut tasks = self.tasks.lock().unwrap();
        if tasks.remove(task_name).is_some() {
            Ok(())
        } else {
            Err(format!("Task '{}' not found", task_name))
        }
    }

    pub fn suspend_task(&self, task_name: &str) -> Result<(), String> {
        let mut tasks = self.tasks.lock().unwrap();
        if let Some(task) = tasks.get_mut(task_name) {
            task.suspend();
            Ok(())
        } else {
            Err(format!("Task '{}' not found", task_name))
        }
    }

    pub fn resume_task(&self, task_name: &str) -> Result<(), String> {
        let mut tasks = self.tasks.lock().unwrap();
        if let Some(task) = tasks.get_mut(task_name) {
            task.resume();
            Ok(())
        } else {
            Err(format!("Task '{}' not found", task_name))
        }
    }

    pub fn schedule(&self) -> Option<String> {
        let tasks = self.tasks.lock().unwrap();

        // Priority-based scheduling with deadline consideration
        let mut selected_task: Option<String> = None;
        let mut highest_priority = &TaskPriority::Idle;
        let mut earliest_deadline: Option<u64> = None;

        for (name, task) in tasks.iter() {
            if task.state == TaskState::Ready {
                let task_priority = task.get_priority();
                let should_select = if task_priority > highest_priority {
                    true
                } else if task_priority == highest_priority {
                    // If same priority, check deadline
                    if let Some(deadline) = task.config.deadline_ms {
                        if let Some(current_earliest) = earliest_deadline {
                            deadline < current_earliest
                        } else {
                            true
                        }
                    } else {
                        false
                    }
                } else {
                    false
                };

                if should_select {
                    highest_priority = task_priority;
                    selected_task = Some(name.clone());
                    earliest_deadline = task.config.deadline_ms;
                }
            }
        }

        selected_task
    }

    pub fn get_task_count(&self) -> usize {
        self.tasks.lock().unwrap().len()
    }

    pub fn get_rtos_type(&self) -> &RtosType {
        &self.rtos_type
    }
}

// Inter-task communication
pub struct MessageQueue {
    messages: Arc<Mutex<Vec<String>>>,
    max_size: usize,
}

impl MessageQueue {
    pub fn new(max_size: usize) -> Self {
        MessageQueue {
            messages: Arc::new(Mutex::new(Vec::new())),
            max_size,
        }
    }

    pub fn send(&self, message: String) -> Result<(), String> {
        let mut messages = self.messages.lock().unwrap();
        if messages.len() >= self.max_size {
            return Err("Queue is full".to_string());
        }
        messages.push(message);
        Ok(())
    }

    pub fn receive(&self) -> Option<String> {
        let mut messages = self.messages.lock().unwrap();
        if !messages.is_empty() {
            Some(messages.remove(0))
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.messages.lock().unwrap().is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.messages.lock().unwrap().len() >= self.max_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtos_scheduler_creation() {
        let scheduler = RtosScheduler::new(RtosType::FreeRTOS);
        assert_eq!(scheduler.get_task_count(), 0);
        assert_eq!(*scheduler.get_rtos_type(), RtosType::FreeRTOS);
    }

    #[test]
    fn test_create_task() {
        let scheduler = RtosScheduler::new(RtosType::FreeRTOS);
        let config = TaskConfig {
            name: "test_task".to_string(),
            priority: TaskPriority::Normal,
            stack_size: 1024,
            deadline_ms: None,
        };
        assert!(scheduler.create_task(config).is_ok());
        assert_eq!(scheduler.get_task_count(), 1);
    }

    #[test]
    fn test_delete_task() {
        let scheduler = RtosScheduler::new(RtosType::Zephyr);
        let config = TaskConfig {
            name: "test_task".to_string(),
            priority: TaskPriority::Normal,
            stack_size: 1024,
            deadline_ms: None,
        };
        scheduler.create_task(config).unwrap();
        assert!(scheduler.delete_task("test_task").is_ok());
        assert_eq!(scheduler.get_task_count(), 0);
    }

    #[test]
    fn test_suspend_resume_task() {
        let scheduler = RtosScheduler::new(RtosType::FreeRTOS);
        let config = TaskConfig {
            name: "test_task".to_string(),
            priority: TaskPriority::Normal,
            stack_size: 1024,
            deadline_ms: None,
        };
        scheduler.create_task(config).unwrap();
        assert!(scheduler.suspend_task("test_task").is_ok());
        assert!(scheduler.resume_task("test_task").is_ok());
    }

    #[test]
    fn test_task_scheduling() {
        let scheduler = RtosScheduler::new(RtosType::FreeRTOS);

        let high_priority = TaskConfig {
            name: "high".to_string(),
            priority: TaskPriority::High,
            stack_size: 1024,
            deadline_ms: None,
        };
        let low_priority = TaskConfig {
            name: "low".to_string(),
            priority: TaskPriority::Low,
            stack_size: 1024,
            deadline_ms: None,
        };

        scheduler.create_task(low_priority).unwrap();
        scheduler.create_task(high_priority).unwrap();

        let next_task = scheduler.schedule();
        assert_eq!(next_task, Some("high".to_string()));
    }

    #[test]
    fn test_message_queue() {
        let queue = MessageQueue::new(5);
        assert!(queue.is_empty());
        assert!(queue.send("Hello".to_string()).is_ok());
        assert!(!queue.is_empty());
        assert_eq!(queue.receive(), Some("Hello".to_string()));
        assert!(queue.is_empty());
    }

    #[test]
    fn test_message_queue_full() {
        let queue = MessageQueue::new(2);
        assert!(queue.send("msg1".to_string()).is_ok());
        assert!(queue.send("msg2".to_string()).is_ok());
        assert!(queue.is_full());
        assert!(queue.send("msg3".to_string()).is_err());
    }

    #[test]
    fn test_deadline_scheduling() {
        let scheduler = RtosScheduler::new(RtosType::FreeRTOS);

        let urgent_task = TaskConfig {
            name: "urgent".to_string(),
            priority: TaskPriority::Normal,
            stack_size: 1024,
            deadline_ms: Some(100),
        };
        let normal_task = TaskConfig {
            name: "normal".to_string(),
            priority: TaskPriority::Normal,
            stack_size: 1024,
            deadline_ms: Some(200),
        };

        scheduler.create_task(normal_task).unwrap();
        scheduler.create_task(urgent_task).unwrap();

        let next_task = scheduler.schedule();
        assert_eq!(next_task, Some("urgent".to_string()));
    }
}
