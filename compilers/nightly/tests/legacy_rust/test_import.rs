// Test importing a module
println!("{}", "Testing import system...");

use crate::compiler::tests::test_module;

println!("{}", "Calling imported function...");
let result = helper_function(21);
println!("{}", "Result:", result);

let greeting = greet("World");
println!("{}", greeting);

println!("{}", "Module constant:", MODULE_CONSTANT);

println!("{}", "Import test complete!");