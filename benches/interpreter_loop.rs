use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gul_lang::backend::interpreter::Interpreter;
use gul_lang::frontend::lexer::Lexer;
use gul_lang::frontend::parser::Parser;

fn bench_interpreter_loop(c: &mut Criterion) {
    c.bench_function("interpreter_loop", |b| {
        let source = r#"
            let sum = 0
            let items = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
            for item in items:
                sum = sum + item
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        b.iter(|| {
            let mut interpreter = Interpreter::new();
            interpreter.run(black_box(&program))
        })
    });
}

criterion_group!(benches, bench_interpreter_loop);
criterion_main!(benches);
