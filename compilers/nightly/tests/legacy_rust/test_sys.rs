// Better test for sys.argv
println!("{}", "Testing sys module...");

// Access sys
println!("{}", "sys object:", sys);

// Access sys.argv
println!("{}", "sys::argv():", sys::argv());

// Test iteration
println!("{}", "Arguments:");
for arg in sys::argv() {
    println!("{}", "  ", arg);

}
println!("{}", "Done!");