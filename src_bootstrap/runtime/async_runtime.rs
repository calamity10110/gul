#![allow(dead_code)]
// Async runtime - executes async functions

use std::future::Future;
use tokio::runtime::Runtime;

pub struct AsyncRuntime {
    runtime: Runtime,
}

impl AsyncRuntime {
    pub fn new() -> Self {
        AsyncRuntime {
            runtime: Runtime::new().expect("Failed to create Tokio runtime"),
        }
    }

    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: Future,
    {
        self.runtime.block_on(future)
    }

    pub fn spawn<F>(&self, future: F) -> tokio::task::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.runtime.spawn(future)
    }

    pub fn spawn_blocking<F, R>(&self, f: F) -> tokio::task::JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        self.runtime.spawn_blocking(f)
    }
}

impl Default for AsyncRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_async_runtime_creation() {
        let runtime = AsyncRuntime::new();
        let result = runtime.block_on(async { 42 });
        assert_eq!(result, 42);
    }

    #[test]
    fn test_async_spawn() {
        let runtime = AsyncRuntime::new();
        let handle = runtime.spawn(async {
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            "done"
        });
        let result = runtime.block_on(handle).unwrap();
        assert_eq!(result, "done");
    }

    #[test]
    fn test_spawn_blocking() {
        let runtime = AsyncRuntime::new();
        let handle = runtime.spawn_blocking(|| {
            std::thread::sleep(std::time::Duration::from_millis(10));
            100
        });
        let result = runtime.block_on(handle).unwrap();
        assert_eq!(result, 100);
    }
}
