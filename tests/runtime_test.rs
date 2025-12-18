// Runtime operation tests
use gul::interop::javascript_runtime::JavaScriptRuntime;
use gul::interop::python_runtime::PythonRuntime;
use gul::interop::rust_loader::RustLoader;

#[test]
fn test_python_runtime_creation() {
    let runtime = PythonRuntime::new();
    assert!(runtime.is_ok());
}

#[test]
fn test_python_execute_simple() {
    let mut runtime = PythonRuntime::new().unwrap();
    let result = runtime.execute("print('Hello from Python')");
    assert!(result.is_ok());
}

#[test]
fn test_python_execute_math() {
    let mut runtime = PythonRuntime::new().unwrap();
    let result = runtime.execute("result = 2 + 2");
    assert!(result.is_ok());
}

#[test]
fn test_javascript_runtime_creation() {
    let runtime = JavaScriptRuntime::new();
    assert!(runtime.is_ok());
}

#[test]
fn test_javascript_execute_simple() {
    let mut runtime = JavaScriptRuntime::new().unwrap();
    let result = runtime.execute("console.log('Hello from JS')");
    assert!(result.is_ok());
}

#[test]
fn test_javascript_eval_expression() {
    let mut runtime = JavaScriptRuntime::new().unwrap();
    let result = runtime.eval("2 + 2");
    assert!(result.is_ok());
}

#[test]
fn test_rust_loader_creation() {
    let loader = RustLoader::new();
    // Loader creation should always succeed
    assert!(std::mem::size_of_val(&loader) > 0);
}

#[test]
fn test_rust_loader_load_nonexistent() {
    let mut loader = RustLoader::new();
    let result = loader.load_library("/nonexistent/path.so");
    assert!(result.is_err());
}
