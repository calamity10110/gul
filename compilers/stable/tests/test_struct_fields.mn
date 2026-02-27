// Minimal test of struct field storage
use crate::compiler::lexer::lexer;

fn test_parser_fields() {
    let source = "let x = 5";
    let tokens = tokenize(source);
    println!("{}", "DEBUG: tokens count before create:");
    println!("{}", (tokens).len());

    struct TestParser {
        data: Vec<Token>,
        pos: i64,

        fn create(toks: Vec<Token>)  ->  TestParser {
            return TestParser{data: toks, pos: i64(0)}

        }
        fn show(&self) {
            println!("{}", "Inside show method");
            println!("{}", self.pos);
            println!("{}", (self.data).len());

        }
    }
    let mut p = TestParser.create(tokens);
    println!("{}", "Parser created");
    p.show();

}
test_parser_fields();

fn main() {
    println!("{}", "Done");
}