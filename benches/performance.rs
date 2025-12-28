use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gul_lang::lexer::Lexer;
use gul_lang::parser::Parser;

fn bench_lexer_simple(c: &mut Criterion) {
    c.bench_function("lexer_simple", |b| {
        b.iter(|| {
            let source = black_box("let x = 42\nlet y = x + 10\nprint(y)");
            let mut lexer = Lexer::new(source);
            lexer.tokenize()
        })
    });
}

fn bench_lexer_complex(c: &mut Criterion) {
    c.bench_function("lexer_complex", |b| {
        b.iter(|| {
            let source = black_box(
                r#"
                fn factorial(n: @int): @int
                    if n <= 1:
                        return 1
                    else:
                        return n * factorial(n - 1)
                
                async fn fetch_data(url: @str): @dict
                    let response = await http.get(url)
                    return response.json()
                
                struct User:
                    name: @str
                    age: @int
                    email: @str
                    
                    fn greet(self):
                        print("Hello, " + self.name)
                "#,
            );
            let mut lexer = Lexer::new(source);
            lexer.tokenize()
        })
    });
}

fn bench_parser_simple(c: &mut Criterion) {
    c.bench_function("parser_simple", |b| {
        b.iter(|| {
            let source = black_box("let x = 42\nlet y = x + 10\nprint(y)");
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse()
        })
    });
}

fn bench_parser_complex(c: &mut Criterion) {
    c.bench_function("parser_complex", |b| {
        b.iter(|| {
            let source = black_box(
                r#"
                fn factorial(n: @int): @int
                    if n <= 1:
                        return 1
                    else:
                        return n * factorial(n - 1)
                
                struct User:
                    name: @str
                    age: @int
                    
                    fn greet(self):
                        print("Hello, " + self.name)
                "#,
            );
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse()
        })
    });
}

fn bench_full_pipeline(c: &mut Criterion) {
    c.bench_function("full_pipeline", |b| {
        b.iter(|| {
            let source = black_box(
                r#"
                fn fibonacci(n: @int): @int
                    if n <= 1:
                        return n
                    return fibonacci(n - 1) + fibonacci(n - 2)
                
                let result = fibonacci(10)
                print(result)
                "#,
            );
            let mut lexer = Lexer::new(source);
            let tokens = lexer.tokenize().unwrap();
            let mut parser = Parser::new(tokens);
            parser.parse()
        })
    });
}

criterion_group!(
    benches,
    bench_lexer_simple,
    bench_lexer_complex,
    bench_parser_simple,
    bench_parser_complex,
    bench_full_pipeline
);
criterion_main!(benches);
