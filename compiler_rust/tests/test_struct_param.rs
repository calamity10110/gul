// Test struct field access via function parameter
struct TestStruct {
    name: String,
    value: i64,

}
fn test_param_access(ts: TestStruct) {
    println!("{}", "Inside function:");
    println!("{}", ts.name);
    println!("{}", ts.value);

}
let test = TestStruct{name: "hello", value: 42};
test_param_access(test);

fn main() {
    println!("{}", "Done!");
}