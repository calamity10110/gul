// Dynamic Library Loading for Rust
use libloading::{Library, Symbol};
use std::path::Path;

/// Rust dynamic library loader
pub struct RustLoader {
    libraries: Vec<Library>,
}

impl RustLoader {
    pub fn new() -> Self {
        Self {
            libraries: Vec::new(),
        }
    }

    /// Load a dynamic library
    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        unsafe {
            let lib = Library::new(path.as_ref())
                .map_err(|e| format!("Failed to load library: {}", e))?;
            self.libraries.push(lib);
            Ok(())
        }
    }

    /// Call a function from loaded library
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    /// - It loads and calls arbitrary code from dynamic libraries
    /// - The caller must ensure the library at `lib_index` exists
    /// - The caller must ensure the function signature `T` matches the actual function
    /// - The function being called must be safe to call with the given arguments
    pub unsafe fn call_function<T>(
        &self,
        lib_index: usize,
        func_name: &str,
    ) -> Result<Symbol<'_, T>, String> {
        let lib = self
            .libraries
            .get(lib_index)
            .ok_or_else(|| "Library not found".to_string())?;

        lib.get(func_name.as_bytes())
            .map_err(|e| format!("Failed to get function: {}", e))
    }

    /// Get number of loaded libraries
    pub fn count(&self) -> usize {
        self.libraries.len()
    }
}

impl Default for RustLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loader_creation() {
        let loader = RustLoader::new();
        assert_eq!(loader.count(), 0);
    }
}
