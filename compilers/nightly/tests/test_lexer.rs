// GUL Compiler - Lexer Test Suite
// Comprehensive tests for lexer/tokenizer

use crate::compiler::lexer::lexer;
use crate::compiler::lexer::token;
use crate::std::testing;

// Test helper to compare token lists
fn assert_tokens_equal(actual: Vec<Token>, expected: Vec<TokenType>, test_name: String) {
    if (actual).len() != (expected).len() {
        format!(println!("{}", "FAIL {}: Expected {} tokens, got {}"), test_name, (expected).len(), (actual).len());
        return bool(false);

    }
    for i in range((actual).len()) {
        if actual[i].type != expected[i] {
            format!(println!("{}", "FAIL {}: Token {} expected {}, got {}"), test_name, i, expected[i], actual[i].type);
            return bool(false);

        }
    }
    format!(println!("{}", "PASS {}"), test_name);
    return bool(true);

// ================================================================================
// BASIC TOKEN TESTS
// ================================================================================

}
fn test_empty_file() {
    let tokens = tokenize("");
    assert_tokens_equal(tokens, vec![TokenType::Eof], "Empty file");

}
fn test_single_integer() {
    let tokens = tokenize("42");
    assert_tokens_equal(tokens, vec![;
        TokenType::Integer,
        TokenType::Eof;
    ], "Single integer");

}
fn test_single_float() {
    let tokens = tokenize("3.14");
    assert_tokens_equal(tokens, vec![;
        TokenType::Float,
        TokenType::Eof;
    ], "Single float");

}
fn test_scientific_notation() {
    let tokens = tokenize("1.23e10");
    assert_tokens_equal(tokens, vec![;
        TokenType::Float,
        TokenType::Eof;
    ], "Scientific notation");

}
fn test_single_identifier() {
    let tokens = tokenize("hello");
    assert_tokens_equal(tokens, vec![;
        TokenType::Identifier,
        TokenType::Eof;
    ], "Single identifier");

}
fn test_single_keyword() {
    let tokens = tokenize("let");
    assert_tokens_equal(tokens, vec![;
        TokenType::Let,
        TokenType::Eof;
    ], "Single keyword (let)");

// ================================================================================
// OPERATOR TESTS
// ================================================================================

}
fn test_arithmetic_operators() {
    let tokens = tokenize("+ - * / %");
    assert_tokens_equal(tokens, vec![;
        TokenType::Plus,
        TokenType::Minus,
        TokenType::Star,
        TokenType::Slash,
        TokenType::Percent,
        TokenType::Eof;
    ], "Arithmetic operators");

}
fn test_compound_operators() {
    let tokens = tokenize("**");
    assert_tokens_equal(tokens, vec![;
        TokenType::DoubleStar,
        TokenType::Eof;
    ], "Double star operator");

}
fn test_comparison_operators() {
    let tokens = tokenize("== != > >= < <=");
    assert_tokens_equal(tokens, vec![;
        TokenType::EqualEqual,
        TokenType::NotEqual,
        TokenType::Greater,
        TokenType::GreaterEq,
        TokenType::Less,
        TokenType::LessEq,
        TokenType::Eof;
    ], "Comparison operators");

}
fn test_logical_operators() {
    let tokens = tokenize("&& || ! && ||");
    assert_tokens_equal(tokens, vec![;
        TokenType::And, // 'and' keyword
        TokenType::Or, // 'or' keyword
        TokenType::Not, // 'not' keyword
        TokenType::And, // &&
        TokenType::Or, // ||
        TokenType::Eof;
    ], "Logical operators");

}
fn test_arrows() {
    let tokens = tokenize("-> =>"),
    assert_tokens_equal(tokens, vec![;
        TokenType::Arrow,
        TokenType::FatArrow,
        TokenType::Eof;
    ], "Arrow operators");

//================================================================================
// KEYWORD TESTS
// ================================================================================

}
fn test_all_keywords() {
    let source = "let let mut fn async struct enum match if elif else for while loop in break continue return try catch finally mn";
    let tokens = tokenize(source);
    assert_tokens_equal(tokens, vec![;
        TokenType::Let, TokenType::Var, TokenType::Fn, TokenType::Async,
        TokenType::Struct, TokenType::Enum, TokenType::Match,
        TokenType::If, TokenType::Elif, TokenType::Else,
        TokenType::For, TokenType::While, TokenType::Loop, TokenType::In,
        TokenType::Break, TokenType::Continue, TokenType::Return,
        TokenType::Try, TokenType::Catch, TokenType::Finally,
        TokenType::Mn,
        TokenType::Eof;
    ], "All keywords");

}
fn test_ownership_keywords() {
    let tokens = tokenize("borrow ref move kept");
    assert_tokens_equal(tokens, vec![;
        TokenType::Borrow,
        TokenType::Ref,
        TokenType::Move,
        TokenType::Kept,
        TokenType::Eof;
    ], "Ownership keywords");

// ================================================================================
// TYPE CONSTRUCTOR TESTS
// ================================================================================

}
fn test_type_constructors() {
    let tokens = tokenize("i64 f64 String bool Vec @tuple HashSet HashMap");
    assert_tokens_equal(tokens, vec![;
        TokenType::AtInt, TokenType::AtFloat, TokenType::AtStr, TokenType::AtBool,
        TokenType::AtList, TokenType::AtTuple, TokenType::AtSet, TokenType::AtDict,
        TokenType::Eof;
    ], "Type constructors");

}
fn test_decorator_tokens() {
    let tokens = tokenize("@imp @python @rust @sql @js @ui");
    assert_tokens_equal(tokens, vec![;
        TokenType::AtImp, TokenType::AtPython, TokenType::AtRust,
        TokenType::AtSql, TokenType::AtJs, TokenType::AtUi,
        TokenType::Eof;
    ], "Decorator tokens");

// ================================================================================
// STRING TESTS
// ================================================================================

}
fn test_simple_string() {
    let tokens = tokenize("\"hello\"");
    let actual = tokens[0];

    if actual.type != TokenType::String {
        println!("{}", "FAIL Simple string: Wrong token type");
        return;

    }
    if actual.value != "hello" {
        format!(println!("{}", "FAIL Simple string: Expected 'hello', got '{}'"), actual.value);
        return;

    }
    println!("{}", "PASS Simple string");

}
fn test_string_with_escapes() {
    let tokens = tokenize("\"hello\\nworld\\t!\"");
    let actual = tokens[0];

    // Should have newline and tab in the value
    if actual.value != "hello\nworld\t!" {
        format!(println!("{}", "FAIL String escapes: Expected 'hello\\nworld\\t!', got '{}'"), actual.value);
        return;

    }
    println!("{}", "PASS String with escapes");

}
fn test_single_quote_string() {
    let tokens = tokenize("'hello'");
    assert_tokens_equal(tokens, vec![;
        TokenType::String,
        TokenType::Eof;
    ], "Single quote string");

// ================================================================================
// INDENTATION TESTS
// ================================================================================

}
fn test_simple_indentation() {
    let source = "if x:\n    println!("{}", y)";
    let tokens = tokenize(source);
    assert_tokens_equal(tokens, vec![;
        TokenType::If,
        TokenType::Identifier, // x
        TokenType::Colon,
        TokenType::Newline,
        TokenType::Indent,
        TokenType::Identifier, // print
        TokenType::LeftParen,
        TokenType::Identifier, // y
        TokenType::RightParen,
        TokenType::Dedent,
        TokenType::Eof;
    ], "Simple indentation");

}
fn test_nested_indentation() {
    let source = "if a:\n    if b:\n        foo()\n    bar()";
    let tokens = tokenize(source);

    // Should have: IF ID : NL INDENT IF ID : NL INDENT ID() DEDENT ID() DEDENT EOF
    let mut expected_count = 20; // Approximate
    if (tokens).len() < expected_count {
        format!(println!("{}", "FAIL Nested indentation: Too few tokens ({})"), (tokens).len());
        return;

    }
    println!("{}", "PASS Nested indentation");

}
fn test_multiple_dedents() {
    let source = "if a:\n    if b:\n        if c:\n            foo()\nbar()";
    let tokens = tokenize(source);

    // Count dedents - should be 3 before bar()
    let mut dedent_count = 0;
    for token in tokens {
        if token.type == TokenType::Dedent {
            dedent_count = dedent_count + 1;

        }
    }
    if dedent_count != 3 {
        format!(println!("{}", "FAIL Multiple dedents: Expected 3, got {}"), dedent_count);
        return;

    }
    println!("{}", "PASS Multiple dedents");

// ================================================================================
// COMMENT TESTS
// ================================================================================

}
fn test_single_line_comment() {
    let source = "let x = 5; // This is a comment"
    let tokens = tokenize(source);

    // Comment should be skipped
    assert_tokens_equal(tokens, vec![;
        TokenType::Let,
        TokenType::Identifier, // x
        TokenType::Equal,
        TokenType::Integer, // 5
        TokenType::Eof;
    ], "Single line comment");

}
fn test_comment_only_line() {
    let source = "; // Just a comment\nlet x = 5"
    let tokens = tokenize(source);

    // Comment line should produce only newline
    assert_tokens_equal(tokens, vec![;
        TokenType::Newline,
        TokenType::Let,
        TokenType::Identifier,
        TokenType::Equal,
        TokenType::Integer,
        TokenType::Eof;
    ], "Comment only line");

// ================================================================================
// COMPLEX EXPRESSION TESTS
// ================================================================================

}
fn test_variable_declaration() {
    let source = "let x = i64(42)";
    let tokens = tokenize(source);
    assert_tokens_equal(tokens, vec![;
        TokenType::Let,
        TokenType::Identifier, // x
        TokenType::Equal,
        TokenType::AtInt,
        TokenType::LeftParen,
        TokenType::Integer, // 42
        TokenType::RightParen,
        TokenType::Eof;
    ], "Variable declaration");

}
fn test_function_definition() {
    let source = "fn add(a, b):\n    return a + b";
    let tokens = tokenize(source);

    let mut has_fn = bool(false);
    let mut has_return = bool(false);
    let mut has_indent = bool(false);

    for token in tokens {
        if token.type == TokenType::Fn {
            has_fn = bool(true);
        }
        if token.type == TokenType::Return {
            has_return = bool(true);
        }
        if token.type == TokenType::Indent {
            has_indent = bool(true);

        }
    }
    if has_fn && has_return && has_indent {
        println!("{}", "PASS Function definition");
    }
    else {
        println!("{}", "FAIL Function definition: Missing expected tokens");

    }
}
fn test_import_statement() {
    let source = "@imp std.io";
    let tokens = tokenize(source);
    assert_tokens_equal(tokens, vec![;
        TokenType::AtImp,
        TokenType::Identifier, // std
        TokenType::Dot,
        TokenType::Identifier, // io
        TokenType::Eof;
    ], "Import statement");

}
fn test_list_literal() {
    let source = "Vec<1, 2, 3>";
    let tokens = tokenize(source);
    assert_tokens_equal(tokens, vec![;
        TokenType::AtList,
        TokenType::LeftBracket,
        TokenType::Integer, // 1
        TokenType::Comma,
        TokenType::Integer, // 2
        TokenType::Comma,
        TokenType::Integer, // 3
        TokenType::RightBracket,
        TokenType::Eof;
    ], "List literal");

}
fn test_dict_literal() {
    let source = "HashMap{name: \"Alice\", age: 30}";
    let tokens = tokenize(source);

    let mut has_dict = bool(false);
    let mut has_colon = bool(false);
    let mut has_comma = bool(false);

    for token in tokens {
        if token.type == TokenType::AtDict {
            has_dict = bool(true);
        }
        if token.type == TokenType::Colon {
            has_colon = bool(true);
        }
        if token.type == TokenType::Comma {
            has_comma = bool(true);

        }
    }
    if has_dict && has_colon && has_comma {
        println!("{}", "PASS Dict literal");
    }
    else {
        println!("{}", "FAIL Dict literal");

// ================================================================================
// EDGE CASE TESTS
// ================================================================================

    }
}
fn test_consecutive_operators() {
    let source = "++--";
    let tokens = tokenize(source);
    assert_tokens_equal(tokens, vec![;
        TokenType::Plus,
        TokenType::Plus,
        TokenType::Minus,
        TokenType::Minus,
        TokenType::Eof;
    ], "Consecutive operators");

}
fn test_number_with_dots() {
    let source = "1..10";
    let tokens = tokenize(source);
    assert_tokens_equal(tokens, vec![;
        TokenType::Integer, // 1
        TokenType::DotDot, // ..
        TokenType::Integer, // 10
        TokenType::Eof;
    ], "Range with double dot");

}
fn test_mixed_quotes() {
    let source = "\"hello\" 'world'";
    let tokens = tokenize(source);
    assert_tokens_equal(tokens, vec![;
        TokenType::String,
        TokenType::String,
        TokenType::Eof;
    ], "Mixed quote styles");

}
fn test_very_long_identifier() {
    let long_id = "a" * 1000; // 1000 character identifier
    let tokens = tokenize(long_id);

    if tokens[0].type == TokenType::Identifier && (tokens[0].value).len() == 1000 {
        println!("{}", "PASS Very long identifier");
    }
    else {
        println!("{}", "FAIL Very long identifier");

    }
}
fn test_large_number() {
    let source = "99999999999999999999";
    let tokens = tokenize(source);
    assert_tokens_equal(tokens, vec![;
        TokenType::Integer,
        TokenType::Eof;
    ], "Large number");

// ================================================================================
// MAIN TEST RUNNER
// ================================================================================

}
fn run_all_tests() {
    println!("{}", "=" * 70);
    println!("{}", "GUL LEXER TEST SUITE");
    println!("{}", "=" * 70);
    println!("{}", );

    println!("{}", "--- Basic Token Tests ---");
    test_empty_file();
    test_single_integer();
    test_single_float();
    test_scientific_notation();
    test_single_identifier();
    test_single_keyword();
    println!("{}", );

    println!("{}", "--- Operator Tests ---");
    test_arithmetic_operators();
    test_compound_operators();
    test_comparison_operators();
    test_logical_operators();
    test_arrows();
    println!("{}", );

    println!("{}", "--- Keyword Tests ---");
    test_all_keywords();
    test_ownership_keywords();
    println!("{}", );

    println!("{}", "--- Type Constructor Tests ---");
    test_type_constructors();
    test_decorator_tokens();
    println!("{}", );

    println!("{}", "--- String Tests ---");
    test_simple_string();
    test_string_with_escapes();
    test_single_quote_string();
    println!("{}", );

    println!("{}", "--- Indentation Tests ---");
    test_simple_indentation();
    test_nested_indentation();
    test_multiple_dedents();
    println!("{}", );

    println!("{}", "--- Comment Tests ---");
    test_single_line_comment();
    test_comment_only_line();
    println!("{}", );

    println!("{}", "--- Complex Expression Tests ---");
    test_variable_declaration();
    test_function_definition();
    test_import_statement();
    test_list_literal();
    test_dict_literal();
    println!("{}", );

    println!("{}", "--- Edge Case Tests ---");
    test_consecutive_operators();
    test_number_with_dots();
    test_mixed_quotes();
    test_very_long_identifier();
    test_large_number();
    println!("{}", );

    println!("{}", "=" * 70);
    println!("{}", "TEST SUITE COMPLETE");
    println!("{}", "=" * 70);

// Main entry point
}
fn main() {
    run_all_tests();
}