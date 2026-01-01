// GUL MCP Scheduler - Automated task scheduling for AI operations
// Handles scheduled code generation, testing, linting, and maintenance

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Task schedule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub name: String,
    pub task: ScheduledTask,
    pub interval: ScheduleInterval,
    pub enabled: bool,
}

/// Types of scheduled tasks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduledTask {
    Lint,
    Format,
    Test,
    Audit,
    Build,
    Deploy,
    AiOptimize,
    DependencyUpdate,
    Documentation,
    Custom(String),
}

/// Schedule interval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduleInterval {
    Hourly,
    Daily,
    Weekly,
    OnCommit,
    OnPush,
    Cron(String),
    Duration(Duration),
}

/// Task scheduler
pub struct TaskScheduler {
    schedules: HashMap<String, Schedule>,
    running: bool,
}

impl TaskScheduler {
    pub fn new() -> Self {
        Self {
            schedules: HashMap::new(),
            running: false,
        }
    }

    /// Add a scheduled task
    pub fn add_schedule(&mut self, schedule: Schedule) {
        self.schedules.insert(schedule.name.clone(), schedule);
    }

    /// Remove a schedule
    pub fn remove_schedule(&mut self, name: &str) {
        self.schedules.remove(name);
    }

    /// Enable/disable a schedule
    pub fn set_enabled(&mut self, name: &str, enabled: bool) {
        if let Some(schedule) = self.schedules.get_mut(name) {
            schedule.enabled = enabled;
        }
    }

    /// Start scheduler
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop scheduler
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Get all schedules
    pub fn list_schedules(&self) -> Vec<&Schedule> {
        self.schedules.values().collect()
    }
}

impl Default for TaskScheduler {
    fn default() -> Self {
        let mut scheduler = Self::new();

        // Add default schedules
        scheduler.add_schedule(Schedule {
            name: "auto_lint".to_string(),
            task: ScheduledTask::Lint,
            interval: ScheduleInterval::OnCommit,
            enabled: true,
        });

        scheduler.add_schedule(Schedule {
            name: "auto_format".to_string(),
            task: ScheduledTask::Format,
            interval: ScheduleInterval::OnCommit,
            enabled: true,
        });

        scheduler.add_schedule(Schedule {
            name: "auto_test".to_string(),
            task: ScheduledTask::Test,
            interval: ScheduleInterval::OnPush,
            enabled: true,
        });

        scheduler.add_schedule(Schedule {
            name: "daily_audit".to_string(),
            task: ScheduledTask::Audit,
            interval: ScheduleInterval::Daily,
            enabled: true,
        });

        scheduler.add_schedule(Schedule {
            name: "weekly_deps".to_string(),
            task: ScheduledTask::DependencyUpdate,
            interval: ScheduleInterval::Weekly,
            enabled: false,
        });

        scheduler
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_creation() {
        let scheduler = TaskScheduler::default();
        assert_eq!(scheduler.list_schedules().len(), 5);
    }

    #[test]
    fn test_add_schedule() {
        let mut scheduler = TaskScheduler::new();
        scheduler.add_schedule(Schedule {
            name: "test".to_string(),
            task: ScheduledTask::Build,
            interval: ScheduleInterval::Hourly,
            enabled: true,
        });
        assert_eq!(scheduler.list_schedules().len(), 1);
    }

    #[test]
    fn test_enable_disable() {
        let mut scheduler = TaskScheduler::default();
        scheduler.set_enabled("auto_lint", false);
        let schedule = scheduler.schedules.get("auto_lint").unwrap();
        assert!(!schedule.enabled);
    }
}
