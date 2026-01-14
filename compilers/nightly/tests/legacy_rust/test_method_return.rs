// Test struct field access from method return
struct Token {
    type: String,
    value: String,

}
struct Parser {
    position: i64,

    fn current(&self)  ->  Token {
        return Token{type: "Fn", value: "fn"}

    }
}
fn test_method_return_access(p: Parser) {
    let tok = p.current();
    println!("{}", "Inside function after method call:");
    println!("{}", tok.type);
    println!("{}", tok.value);

}
let parser = Parser{position: 0};
test_method_return_access(parser);

fn main() {
    println!("{}", "Done!");
}