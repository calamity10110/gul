// Lexer module - tokenizes source code

pub mod scanner;
pub mod tokens;
pub use scanner::Lexer;
pub use tokens::Token;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("def x = 10");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Def);
        assert_eq!(tokens[1], Token::Identifier("x".to_string()));
        assert_eq!(tokens[2], Token::Equal);
        assert_eq!(tokens[3], Token::Integer(10));
    }

    #[test]
    fn test_ui_sprite() {
        let mut lexer = Lexer::new("def tree = ^&^[tree]");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Def);
        assert_eq!(tokens[1], Token::Identifier("tree".to_string()));
        assert_eq!(tokens[2], Token::Equal);
        assert_eq!(tokens[3], Token::UiSprite("tree".to_string()));
    }

    #[test]
    fn test_ui_sprite_with_properties() {
        let mut lexer = Lexer::new("^&^[slider{min=0, max=100}]");
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens[0],
            Token::UiSprite("slider{min=0, max=100}".to_string())
        );
    }

    #[test]
    fn test_multiline_comment() {
        let mut lexer = Lexer::new("#[\nThis is a\nmulti-line comment\n]#\ndef x = 5");
        let tokens = lexer.tokenize();

        // Should skip the multi-line comment
        assert_eq!(tokens[0], Token::Newline);
        assert_eq!(tokens[1], Token::Def);
        assert_eq!(tokens[2], Token::Identifier("x".to_string()));
    }

    #[test]
    fn test_scientific_units() {
        let mut lexer = Lexer::new("def speed = 10 m/s");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Def);
        assert_eq!(tokens[1], Token::Identifier("speed".to_string()));
        assert_eq!(tokens[2], Token::Equal);
        assert_eq!(tokens[3], Token::Integer(10));
        assert_eq!(tokens[4], Token::Unit("m/s".to_string()));
    }

    #[test]
    fn test_scientific_units_complex() {
        let mut lexer = Lexer::new("def accel = 9.81 m/s^2");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Def);
        assert_eq!(tokens[1], Token::Identifier("accel".to_string()));
        assert_eq!(tokens[2], Token::Equal);
        assert_eq!(tokens[3], Token::Float(9.81));
        assert_eq!(tokens[4], Token::Unit("m/s^2".to_string()));
    }

    #[test]
    fn test_comparison_operators() {
        let mut lexer = Lexer::new("a == b != c < d <= e > f >= g");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Identifier("a".to_string()));
        assert_eq!(tokens[1], Token::EqualEqual);
        assert_eq!(tokens[2], Token::Identifier("b".to_string()));
        assert_eq!(tokens[3], Token::BangEqual);
        assert_eq!(tokens[4], Token::Identifier("c".to_string()));
        assert_eq!(tokens[5], Token::Less);
        assert_eq!(tokens[6], Token::Identifier("d".to_string()));
        assert_eq!(tokens[7], Token::LessEqual);
        assert_eq!(tokens[8], Token::Identifier("e".to_string()));
        assert_eq!(tokens[9], Token::Greater);
        assert_eq!(tokens[10], Token::Identifier("f".to_string()));
        assert_eq!(tokens[11], Token::GreaterEqual);
    }

    #[test]
    fn test_logical_operators() {
        let mut lexer = Lexer::new("a && b || !c");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Identifier("a".to_string()));
        assert_eq!(tokens[1], Token::And);
        assert_eq!(tokens[2], Token::Identifier("b".to_string()));
        assert_eq!(tokens[3], Token::Or);
        assert_eq!(tokens[4], Token::Not);
        assert_eq!(tokens[5], Token::Identifier("c".to_string()));
    }

    #[test]
    fn test_bitwise_operators() {
        let mut lexer = Lexer::new("a & b | c << d >> e");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Identifier("a".to_string()));
        assert_eq!(tokens[1], Token::Ampersand);
        assert_eq!(tokens[2], Token::Identifier("b".to_string()));
        assert_eq!(tokens[3], Token::Pipe);
        assert_eq!(tokens[4], Token::Identifier("c".to_string()));
        assert_eq!(tokens[5], Token::LessLess);
        assert_eq!(tokens[6], Token::Identifier("d".to_string()));
        assert_eq!(tokens[7], Token::GreaterGreater);
    }

    #[test]
    fn test_ownership_keywords() {
        let mut lexer = Lexer::new("own x ref y copy z");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Own);
        assert_eq!(tokens[1], Token::Identifier("x".to_string()));
        assert_eq!(tokens[2], Token::Ref);
        assert_eq!(tokens[3], Token::Identifier("y".to_string()));
        assert_eq!(tokens[4], Token::Copy);
        assert_eq!(tokens[5], Token::Identifier("z".to_string()));
    }

    #[test]
    fn test_async_await() {
        let mut lexer = Lexer::new("asy fetch():\n    res = await http.get(url)");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Asy);
        assert_eq!(tokens[1], Token::Identifier("fetch".to_string()));
        assert_eq!(tokens[2], Token::LeftParen);
        assert_eq!(tokens[3], Token::RightParen);
        assert_eq!(tokens[4], Token::Colon);
        assert_eq!(tokens[5], Token::Newline);
        // ... more tokens
        let await_pos = tokens.iter().position(|t| matches!(t, Token::Await));
        assert!(await_pos.is_some());
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_float_numbers() {
        let mut lexer = Lexer::new("3.14 2.718 0.5");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::Float(3.14));
        assert_eq!(tokens[1], Token::Float(2.718));
        assert_eq!(tokens[2], Token::Float(0.5));
    }

    #[test]
    fn test_string_literals() {
        let mut lexer = Lexer::new("\"Hello, World!\" \"test\"");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::String("Hello, World!".to_string()));
        assert_eq!(tokens[1], Token::String("test".to_string()));
    }

    #[test]
    fn test_control_flow_keywords() {
        let mut lexer =
            Lexer::new("if x > 0:\n    return x\nelif x < 0:\n    return -x\nelse:\n    return 0");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::If);
        let elif_pos = tokens.iter().position(|t| matches!(t, Token::Elif));
        assert!(elif_pos.is_some());
        let else_pos = tokens.iter().position(|t| matches!(t, Token::Else));
        assert!(else_pos.is_some());
        let return_count = tokens.iter().filter(|t| matches!(t, Token::Return)).count();
        assert_eq!(return_count, 3);
    }

    #[test]
    fn test_ui_sprite_variations() {
        // Test various UI sprite formats from SYNTAX.md
        // These test the ^&^[component{properties}] syntax for inline UI components
        let test_cases = vec![
            ("^&^[tree]", "tree"),
            (
                "^&^[slider{min=0, max=100, value=50}]",
                "slider{min=0, max=100, value=50}",
            ),
            (
                "^&^[button{text=\"Click Me\"}]",
                "button{text=\"Click Me\"}",
            ),
            ("^&^[image:icon]", "image:icon"),
            (
                "^&^[sprite:player{x=10, y=20, width=32, height=32}]",
                "sprite:player{x=10, y=20, width=32, height=32}",
            ),
        ];

        for (input, expected) in test_cases {
            let mut lexer = Lexer::new(input);
            let tokens = lexer.tokenize();
            assert_eq!(tokens[0], Token::UiSprite(expected.to_string()));
        }
    }

    #[test]
    fn test_ui_macro_syntax() {
        let mut lexer = Lexer::new("ui![button{text=\"Click\"}]");
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens[0],
            Token::UiSprite("button{text=\"Click\"}".to_string())
        );
    }

    #[test]
    fn test_foreign_language_blocks() {
        // Test foreign language block syntax from SYNTAX.md
        // This tests the cs language: ... syntax for multi-language integration
        let test_cases = vec![
            ("cs rust:\n    fn sum(a: i32, b: i32) -> i32 { a + b }", "rust"),
            ("cs python:\n    import math\n    def compute(x):\n        return math.sin(x)", "python"),
            ("cs js:\n    export function greet(name) {\n        return `Hello, ${name}!`;\n    }", "js"),
            ("cs ts:\n    export function add(a: number, b: number): number {\n        return a + b;\n    }", "ts"),
            ("cs c:\n    int fibonacci(int n) {\n        return n <= 1 ? n : fibonacci(n-1) + fibonacci(n-2);\n    }", "c"),
            ("cs sql:\n    select * from users where age > 18 order by name;", "sql"),
        ];

        for (input, expected_lang) in test_cases {
            let mut lexer = Lexer::new(input);
            let tokens = lexer.tokenize();

            // Find the Cs token followed by identifier
            let cs_pos = tokens.iter().position(|t| matches!(t, Token::Cs));
            assert!(cs_pos.is_some(), "Expected Cs token in: {}", input);

            // Check that the next token is the expected language identifier
            if let Some(pos) = cs_pos {
                if pos + 1 < tokens.len() {
                    if let Token::Identifier(lang) = &tokens[pos + 1] {
                        assert_eq!(
                            lang, expected_lang,
                            "Expected language {} but got {}",
                            expected_lang, lang
                        );
                    } else {
                        panic!("Expected identifier after Cs token");
                    }
                }
            }
        }
    }

    #[test]
    fn test_type_annotations() {
        let mut lexer =
            Lexer::new("def name: str = \"Alice\"\ndef age: int = 30\ndef speed: float = 10.5 m/s");
        let tokens = lexer.tokenize();

        // Check for colon tokens (type annotations)
        let colon_count = tokens.iter().filter(|t| matches!(t, Token::Colon)).count();
        assert_eq!(colon_count, 3);

        // Check for type identifiers
        let type_identifiers: Vec<_> = tokens
            .iter()
            .filter_map(|t| {
                if let Token::Identifier(id) = t {
                    if id == "str" || id == "int" || id == "float" {
                        Some(id.as_str())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(type_identifiers, vec!["str", "int", "float"]);
    }

    #[test]
    fn test_loop_constructs() {
        let mut lexer = Lexer::new("loop:\n    print(\"infinite\")\n    break_if(condition)\n\nfor item in collection:\n    process(item)\n\nwhile condition:\n    update()");
        let tokens = lexer.tokenize();

        // Check for loop keywords
        assert!(tokens.iter().any(|t| matches!(t, Token::Loop)));
        assert!(tokens.iter().any(|t| matches!(t, Token::For)));
        assert!(tokens.iter().any(|t| matches!(t, Token::While)));
        assert!(tokens.iter().any(|t| matches!(t, Token::In)));
    }

    #[test]
    fn test_import_statements() {
        let mut lexer =
            Lexer::new("imp std.io\nimp std.http\nimp ai.cv\nimp ui\nimp embedded.gpio");
        let tokens = lexer.tokenize();

        // Check for import keywords
        let import_count = tokens.iter().filter(|t| matches!(t, Token::Imp)).count();
        assert_eq!(import_count, 5);

        // Check for dot notation in imports
        let dot_count = tokens.iter().filter(|t| matches!(t, Token::Dot)).count();
        assert_eq!(dot_count, 4); // std.io, std.http, ai.cv, embedded.gpio
    }

    #[test]
    fn test_main_entry_point() {
        // v3 syntax: just `mn:` without main() or parentheses
        let mut lexer = Lexer::new("mn:\n    print(\"Hello, World!\")\n    ui.print(^&^[tree])\n    data = await fetch(\"https://api.example.com\")\n    print(data)");
        let tokens = lexer.tokenize();

        // Check for mn keyword
        assert!(tokens.iter().any(|t| matches!(t, Token::Mn)));

        // Check for await keyword
        assert!(tokens.iter().any(|t| matches!(t, Token::Await)));

        // Check for UI sprite
        assert!(tokens.iter().any(|t| matches!(t, Token::UiSprite(_))));
    }

    #[test]
    fn test_scientific_expressions() {
        // Test scientific notation and expressions from SYNTAX.md
        // Covers physics formulas, chemistry expressions, and mathematical operations
        let test_cases = vec![
            (
                "def F = m * a",
                vec![
                    Token::Def,
                    Token::Identifier("F".to_string()),
                    Token::Equal,
                    Token::Identifier("m".to_string()),
                    Token::Star,
                    Token::Identifier("a".to_string()),
                ],
            ),
            (
                "def E = m * c^2",
                vec![
                    Token::Def,
                    Token::Identifier("E".to_string()),
                    Token::Equal,
                    Token::Identifier("m".to_string()),
                    Token::Star,
                    Token::Identifier("c".to_string()),
                    Token::Caret,
                    Token::Integer(2),
                ],
            ),
            (
                "def v = d / t",
                vec![
                    Token::Def,
                    Token::Identifier("v".to_string()),
                    Token::Equal,
                    Token::Identifier("d".to_string()),
                    Token::Slash,
                    Token::Identifier("t".to_string()),
                ],
            ),
            (
                "def pH = -log10([H+])",
                vec![
                    Token::Def,
                    Token::Identifier("pH".to_string()),
                    Token::Equal,
                    Token::Minus,
                    Token::Identifier("log10".to_string()),
                    Token::LeftParen,
                    Token::LeftBracket,
                    Token::Identifier("H".to_string()),
                    Token::Plus,
                    Token::RightBracket,
                    Token::RightParen,
                ],
            ),
        ];

        for (input, expected_pattern) in test_cases {
            let mut lexer = Lexer::new(input);
            let tokens = lexer.tokenize();

            // Check that we have the expected token pattern
            for (i, expected_token) in expected_pattern.iter().enumerate() {
                if i < tokens.len() {
                    match (expected_token, &tokens[i]) {
                        (Token::Identifier(expected), Token::Identifier(actual)) => {
                            assert_eq!(expected, actual)
                        }
                        (expected, actual) => assert_eq!(expected, actual),
                    }
                }
            }
        }
    }

    #[test]
    fn test_v0_11_0_tokens() {
        let mut lexer = Lexer::new("?count = 0 @int @global");
        let tokens = lexer.tokenize();

        assert_eq!(tokens[0], Token::QuestionMark);
        assert_eq!(tokens[1], Token::Identifier("count".to_string()));
        assert_eq!(tokens[2], Token::Equal);
        assert_eq!(tokens[3], Token::Integer(0));
        assert_eq!(tokens[4], Token::At);
        assert_eq!(tokens[5], Token::Identifier("int".to_string()));
        assert_eq!(tokens[6], Token::At);
        assert_eq!(tokens[7], Token::Global);
    }

    #[test]
    fn test_indentation_tracking() {
        let code =
            "def func():\n    x = 1\n    if x > 0:\n        print(x)\n        y = 2\n    return x";
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize();

        // Find indent and dedent tokens
        let indent_count = tokens.iter().filter(|t| matches!(t, Token::Indent)).count();
        let dedent_count = tokens.iter().filter(|t| matches!(t, Token::Dedent)).count();

        // Should have 2 indents (after func(): and after if:) and 2 dedents
        assert_eq!(indent_count, 2, "Expected 2 INDENT tokens");
        assert_eq!(dedent_count, 2, "Expected 2 DEDENT tokens");

        // Verify token sequence includes proper indentation
        assert!(tokens.contains(&Token::Indent));
        assert!(tokens.contains(&Token::Dedent));
    }

    #[test]
    fn test_nested_indentation() {
        let code = "if a:\n    if b:\n        if c:\n            x = 1\n        y = 2\n    z = 3";
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize();

        let indent_count = tokens.iter().filter(|t| matches!(t, Token::Indent)).count();
        let dedent_count = tokens.iter().filter(|t| matches!(t, Token::Dedent)).count();

        // Should have 3 indents and 3 dedents for nested blocks
        assert_eq!(indent_count, 3);
        assert_eq!(dedent_count, 3);
    }

    // v2.1 Bracket Equivalence Tests
    #[test]
    fn test_bracket_helper_is_open() {
        assert!(Token::LeftParen.is_open_bracket());
        assert!(Token::LeftBracket.is_open_bracket());
        assert!(Token::LeftBrace.is_open_bracket());
        assert!(!Token::RightParen.is_open_bracket());
        assert!(!Token::Plus.is_open_bracket());
    }

    #[test]
    fn test_bracket_helper_is_close() {
        assert!(Token::RightParen.is_close_bracket());
        assert!(Token::RightBracket.is_close_bracket());
        assert!(Token::RightBrace.is_close_bracket());
        assert!(!Token::LeftParen.is_close_bracket());
        assert!(!Token::Minus.is_close_bracket());
    }

    #[test]
    fn test_bracket_helper_matching_close() {
        assert_eq!(Token::LeftParen.matching_close(), Some(Token::RightParen));
        assert_eq!(
            Token::LeftBracket.matching_close(),
            Some(Token::RightBracket)
        );
        assert_eq!(Token::LeftBrace.matching_close(), Some(Token::RightBrace));
        assert_eq!(Token::RightParen.matching_close(), None);
        assert_eq!(Token::Plus.matching_close(), None);
    }

    #[test]
    fn test_bracket_helper_brackets_match() {
        assert!(Token::LeftParen.brackets_match(&Token::RightParen));
        assert!(Token::LeftBracket.brackets_match(&Token::RightBracket));
        assert!(Token::LeftBrace.brackets_match(&Token::RightBrace));

        // Mismatched should be false
        assert!(!Token::LeftParen.brackets_match(&Token::RightBracket));
        assert!(!Token::LeftParen.brackets_match(&Token::RightBrace));
        assert!(!Token::LeftBracket.brackets_match(&Token::RightParen));
    }

    #[test]
    fn test_v21_all_bracket_types_tokenized() {
        let mut lexer = Lexer::new("func() func[] func{}");
        let tokens = lexer.tokenize();

        // Should have all bracket types
        assert!(tokens.iter().any(|t| matches!(t, Token::LeftParen)));
        assert!(tokens.iter().any(|t| matches!(t, Token::RightParen)));
        assert!(tokens.iter().any(|t| matches!(t, Token::LeftBracket)));
        assert!(tokens.iter().any(|t| matches!(t, Token::RightBracket)));
        assert!(tokens.iter().any(|t| matches!(t, Token::LeftBrace)));
        assert!(tokens.iter().any(|t| matches!(t, Token::RightBrace)));
    }
}
