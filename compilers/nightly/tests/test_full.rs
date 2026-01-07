// Test program for full interpreter features

// Variables
let x = 42;
let mut y = 10;

// Functions
fn add(a, b) {
    return a + b;

}
fn greet(name) {
    let message = "Hello, " + name;
    return message;

// Control flow
}
if x > 30 {
    println!("{}", "x is large:", x);
}
else {
    println!("{}", "x is small");

// Loops
}
println!("{}", "Counting:");
for i in range(5) {
    println!("{}", "  ", i);

}
let total = 0;
while total < 10 {
    total = total + 1;

}
println!("{}", "Total:", total);

// Function calls
let sum = add(5, 3);
println!("{}", "5 + 3 =", sum);

let greeting = greet("World");
println!("{}", greeting);

// Lists
let numbers = [1, 2, 3, 4, 5];
println!("{}", "Numbers:", numbers);
println!("{}", "Length:", (numbers).len());

// Struct (simplified)
// struct Point:
//     x: int
//     y: int

println!("{}", "✨ All features tested!");