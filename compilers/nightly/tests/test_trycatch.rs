//  Test try/catch
println!("{}", "Testing try/catch...");

if true {
    println!("{}", "In try block");
    let result = read_file("nonexistent_file.txt");
    println!("{}", "Should ! reach here");
}
else if false {
    println!("{}", "In catch block - error handled!");

}
println!("{}", "After try/catch block");
println!("{}", "Test complete!");