// Profiler module - performance profiling and analysis
#![allow(dead_code)]
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct ProfileEntry {
    pub name: String,
    pub call_count: usize,
    pub total_time: Duration,
    pub self_time: Duration,
    pub memory_allocated: usize,
    pub memory_freed: usize,
}

#[derive(Debug, Clone)]
pub struct ProfileSample {
    pub timestamp: Instant,
    pub function: String,
    pub line: usize,
    pub memory_usage: usize,
}

pub struct Profiler {
    entries: HashMap<String, ProfileEntry>,
    samples: Vec<ProfileSample>,
    active_timers: HashMap<String, Instant>,
    start_time: Option<Instant>,
    total_memory: usize,
    enabled: bool,
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Profiler {
    pub fn new() -> Self {
        Profiler {
            entries: HashMap::new(),
            samples: Vec::new(),
            active_timers: HashMap::new(),
            start_time: None,
            total_memory: 0,
            enabled: false,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
        self.start_time = Some(Instant::now());
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn reset(&mut self) {
        self.entries.clear();
        self.samples.clear();
        self.active_timers.clear();
        self.start_time = None;
        self.total_memory = 0;
    }

    // Timing profiling
    pub fn start_function(&mut self, name: String) {
        if !self.enabled {
            return;
        }

        self.active_timers.insert(name.clone(), Instant::now());

        // Initialize entry if it doesn't exist
        self.entries.entry(name.clone()).or_insert(ProfileEntry {
            name,
            call_count: 0,
            total_time: Duration::ZERO,
            self_time: Duration::ZERO,
            memory_allocated: 0,
            memory_freed: 0,
        });
    }

    pub fn end_function(&mut self, name: &str) {
        if !self.enabled {
            return;
        }

        if let Some(start_time) = self.active_timers.remove(name) {
            let elapsed = start_time.elapsed();

            if let Some(entry) = self.entries.get_mut(name) {
                entry.call_count += 1;
                entry.total_time += elapsed;
                entry.self_time += elapsed;
            }
        }
    }

    pub fn record_sample(&mut self, function: String, line: usize) {
        if !self.enabled {
            return;
        }

        self.samples.push(ProfileSample {
            timestamp: Instant::now(),
            function,
            line,
            memory_usage: self.total_memory,
        });
    }

    // Memory profiling
    pub fn allocate_memory(&mut self, function: &str, size: usize) {
        if !self.enabled {
            return;
        }

        self.total_memory += size;

        if let Some(entry) = self.entries.get_mut(function) {
            entry.memory_allocated += size;
        }
    }

    pub fn free_memory(&mut self, function: &str, size: usize) {
        if !self.enabled {
            return;
        }

        self.total_memory = self.total_memory.saturating_sub(size);

        if let Some(entry) = self.entries.get_mut(function) {
            entry.memory_freed += size;
        }
    }

    pub fn get_total_memory(&self) -> usize {
        self.total_memory
    }

    // Reporting
    pub fn get_profile_data(&self) -> Vec<&ProfileEntry> {
        let mut entries: Vec<&ProfileEntry> = self.entries.values().collect();
        entries.sort_by(|a, b| b.total_time.cmp(&a.total_time));
        entries
    }

    pub fn get_hotspots(&self, top_n: usize) -> Vec<&ProfileEntry> {
        let mut entries: Vec<&ProfileEntry> = self.entries.values().collect();
        entries.sort_by(|a, b| b.total_time.cmp(&a.total_time));
        entries.into_iter().take(top_n).collect()
    }

    pub fn get_memory_hotspots(&self, top_n: usize) -> Vec<&ProfileEntry> {
        let mut entries: Vec<&ProfileEntry> = self.entries.values().collect();
        entries.sort_by(|a, b| b.memory_allocated.cmp(&a.memory_allocated));
        entries.into_iter().take(top_n).collect()
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Profiling Report ===\n\n");

        if let Some(start) = self.start_time {
            let total_time = start.elapsed();
            report.push_str(&format!("Total execution time: {:?}\n", total_time));
            report.push_str(&format!(
                "Total memory usage: {} bytes\n\n",
                self.total_memory
            ));
        }

        report.push_str("Function Performance:\n");
        report.push_str(&format!(
            "{:<30} {:>10} {:>15} {:>15}\n",
            "Function", "Calls", "Total Time", "Avg Time"
        ));
        report.push_str(&"-".repeat(70));
        report.push('\n');

        for entry in self.get_profile_data() {
            let avg_time = if entry.call_count > 0 {
                entry.total_time / entry.call_count as u32
            } else {
                Duration::ZERO
            };

            report.push_str(&format!(
                "{:<30} {:>10} {:>15?} {:>15?}\n",
                entry.name, entry.call_count, entry.total_time, avg_time
            ));
        }

        report.push('\n');
        report.push_str("Memory Usage:\n");
        report.push_str(&format!(
            "{:<30} {:>15} {:>15}\n",
            "Function", "Allocated", "Freed"
        ));
        report.push_str(&"-".repeat(60));
        report.push('\n');

        for entry in self.get_memory_hotspots(10) {
            report.push_str(&format!(
                "{:<30} {:>15} {:>15}\n",
                entry.name, entry.memory_allocated, entry.memory_freed
            ));
        }

        report
    }

    pub fn generate_flame_graph_data(&self) -> Vec<(String, Duration)> {
        self.entries
            .values()
            .map(|entry| (entry.name.clone(), entry.total_time))
            .collect()
    }

    pub fn get_samples(&self) -> &[ProfileSample] {
        &self.samples
    }

    pub fn get_entry(&self, name: &str) -> Option<&ProfileEntry> {
        self.entries.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_profiler_creation() {
        let profiler = Profiler::new();
        assert!(!profiler.is_enabled());
        assert_eq!(profiler.get_total_memory(), 0);
    }

    #[test]
    fn test_enable_disable() {
        let mut profiler = Profiler::new();

        profiler.enable();
        assert!(profiler.is_enabled());

        profiler.disable();
        assert!(!profiler.is_enabled());
    }

    #[test]
    fn test_function_timing() {
        let mut profiler = Profiler::new();
        profiler.enable();

        profiler.start_function("test_func".to_string());
        thread::sleep(Duration::from_millis(10));
        profiler.end_function("test_func");

        let entry = profiler.get_entry("test_func").unwrap();
        assert_eq!(entry.call_count, 1);
        assert!(entry.total_time >= Duration::from_millis(10));
    }

    #[test]
    fn test_memory_tracking() {
        let mut profiler = Profiler::new();
        profiler.enable();

        profiler.start_function("test_func".to_string());
        profiler.allocate_memory("test_func", 1024);
        profiler.free_memory("test_func", 512);
        profiler.end_function("test_func");

        let entry = profiler.get_entry("test_func").unwrap();
        assert_eq!(entry.memory_allocated, 1024);
        assert_eq!(entry.memory_freed, 512);
        assert_eq!(profiler.get_total_memory(), 512);
    }

    #[test]
    fn test_hotspots() {
        let mut profiler = Profiler::new();
        profiler.enable();

        profiler.start_function("slow_func".to_string());
        thread::sleep(Duration::from_millis(20));
        profiler.end_function("slow_func");

        profiler.start_function("fast_func".to_string());
        thread::sleep(Duration::from_millis(5));
        profiler.end_function("fast_func");

        let hotspots = profiler.get_hotspots(1);
        assert_eq!(hotspots.len(), 1);
        assert_eq!(hotspots[0].name, "slow_func");
    }

    #[test]
    fn test_reset() {
        let mut profiler = Profiler::new();
        profiler.enable();

        profiler.start_function("test".to_string());
        profiler.end_function("test");

        profiler.reset();
        assert_eq!(profiler.entries.len(), 0);
        assert_eq!(profiler.samples.len(), 0);
    }

    #[test]
    fn test_generate_report() {
        let mut profiler = Profiler::new();
        profiler.enable();

        profiler.start_function("test".to_string());
        profiler.end_function("test");

        let report = profiler.generate_report();
        assert!(report.contains("Profiling Report"));
        assert!(report.contains("test"));
    }

    #[test]
    fn test_flame_graph_data() {
        let mut profiler = Profiler::new();
        profiler.enable();

        profiler.start_function("func1".to_string());
        profiler.end_function("func1");

        profiler.start_function("func2".to_string());
        profiler.end_function("func2");

        let data = profiler.generate_flame_graph_data();
        assert_eq!(data.len(), 2);
    }
}
