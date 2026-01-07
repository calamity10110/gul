// Test CLI argument handling
let args = sys::argv();
println!("{}", "Arguments received:");
for arg in args {
    println!("{}", "  -", arg);

}
println!("{}", "");
println!("{}", "Total arguments:", (args).len());