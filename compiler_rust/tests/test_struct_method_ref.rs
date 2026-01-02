// Test struct method that uses ref self and returns struct
struct Result {
    value: String,

}
struct Parser {
    data: String,

    fn create(d: String)  ->  Parser {
        return Parser{data: d}

    }
    fn simple_method(&mut self)  ->  Result {
        println!("{}", "DEBUG: Inside simple_method");
        println!("{}", "DEBUG: self.data is:");
        println!("{}", self.data);
        return Result{value: "success"}

    }
}
fn use_parser() {
    let mut p = Parser.create("test data");
    println!("{}", "DEBUG: Calling simple_method...");
    let mut r = p.simple_method();
    println!("{}", "DEBUG: simple_method returned");
    println!("{}", "DEBUG: result.value:");
    println!("{}", r.value);

}
use_parser();

fn main() {
    println!("{}", "Done!");
}