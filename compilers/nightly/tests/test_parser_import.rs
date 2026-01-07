// Minimal test file that imports parser and tests method execution
use crate::compiler::lexer::lexer;
use crate::compiler::parser::parser;

fn test_it() {
    let source = "let x = 5";
    let tokens = tokenize(source);
    let mut p = Parser.create(tokens);
    println!("{}", "DEBUG: p created");
    println!("{}", "DEBUG: p type check");
    println!("{}", p);
    println!("{}", "Before calling test_method_works");
    p.test_method_works();
    println!("{}", "After calling test_method_works");

}
test_it();

fn main() {
    println!("{}", "Done");
}