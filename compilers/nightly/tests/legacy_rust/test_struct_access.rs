// Test struct field access
struct TestStruct {
    name: String,
    value: i64,

}
let test = TestStruct{name: "hello", value: 42};
println!("{}", "Testing struct field access:");
println!("{}", test.name);
println!("{}", test.value);

fn main() {
    println!("{}", "Done!");
}