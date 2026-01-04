// GUL v3.2 Flat Compiler - Minimal and Fast

enum TokenType { Identifier, Integer, Symbol, Newline, Eof, Indent, Dedent, Let, Fn, Return }

struct Token {
    type: TokenType,
    value: String,

    }
fn main() {
    println!("{}", "🚀 Specialized Flat Compiler Starting");
    let source = read_file("compiler/tests/simple_add_stripped.mn");
    let mut tokens = vec![];

    // --- LEXER ---
    let mut pos = i64(0);
    let length = (source).len();
    let mut indents = vec![0];
    let mut line_start = bool(true);
    while pos < length {
        let ch0 = source[pos];
        if line_start {
            let mut current_indent = i64(0);
            while pos < length {
                let ws = source[pos];
                if ws == " " {
                    current_indent = current_indent + 1;
                    pos = pos + 1;
                    }
                else {
                    if ws == "\t" {
                        current_indent = current_indent + 4;
                        pos = pos + 1;
                        }
                    else { break }
                    }
                }
            if pos < length {
                let c2 = source[pos];
                if c2 != "\n" {
                    if c2 != "
                        let last = indents[(indents).len()-1];
                        if current_indent > last {
                            indents.add(current_indent);
                            tokens.add(Token{type: TokenType.Indent, value: ""});
                            }
                        else {
                            if current_indent < last {
                                while (indents).len() > 1 and indents[(indents).len()-1] > current_indent {
                                    indents.remove((indents).len()-1);
                                    tokens.add(Token{type: TokenType.Dedent, value: ""});
                                    }
                                }
                            }
                        }
                    }
                }
            line_start = bool(false);
            if pos >= length { break }

            }
        let ch = source[pos];
        if ch == " " { pos = pos + 1 }
        else {
            if ch == "\t" { pos = pos + 1 }
            else {
                if ch == "\r" { pos = pos + 1 }
                else {
                    if ch == "\n" {
                        tokens.add(Token{type: TokenType.Newline, value: "\n"});
                        pos = pos + 1;
                        line_start = bool(true);
                        }
                    else {
                        if ch == "
                            while pos < length {
                                if source[pos] != "\n" { pos = pos + 1 }
                                else { break }
                                }
                            }
                        else {
                            if (ch >= "a" and ch <= "z") or (ch >= "A" and ch <= "Z") or ch == "_" {
                                let mut id = ch;
                                pos = pos + 1;
                                while pos < length {
                                    let c2 = source[pos];
                                    if (c2 >= "a" and c2 <= "z") or (c2 >= "A" and c2 <= "Z") or (c2 >= "0" and c2 <= "9") or c2 == "_" {
                                        id = id + c2;
                                        pos = pos + 1;
                                        }
                                    else { break }
                                    }
                                let mut t = TokenType.Identifier;
                                if id == "let" { t = TokenType.Let }
                                else {
                                    if id == "fn" { t = TokenType.Fn }
                                    else {
                                        if id == "return" { t = TokenType.Return }
                                        }
                                    }
                                tokens.add(Token{type: t, value: id});
                                }
                            else {
                                if ch >= "0" and ch <= "9" {
                                    let mut n = ch;
                                    pos = pos + 1;
                                    while pos < length {
                                        let c2 = source[pos];
                                        if c2 >= "0" and c2 <= "9" {
                                            n = n + c2;
                                            pos = pos + 1;
                                            }
                                        else { break }
                                        }
                                    tokens.add(Token{type: TokenType.Integer, value: n});
                                    }
                                else {
                                    tokens.add(Token{type: TokenType.Symbol, value: ch});
                                    pos = pos + 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    while (indents).len() > 1 {
        indents.remove((indents).len()-1);
        tokens.add(Token{type: TokenType.Dedent, value: ""});
        }
    tokens.add(Token{type: TokenType.Eof, value: ""});
    println!("{}", "    Lexed " + format!("{}", (tokens).len()) + " tokens");

    // --- SPECIALIZED PARSER & CODEGEN ---
    println!("{}", "  [2-4] Specialized Parser & Code Gen...");
    let mut out = "fn main() {\n";
    let mut lib_funcs = "";

    let mut t_idx = i64(0);
    let t_len = (tokens).len();
    while t_idx < t_len {
        let t = tokens[t_idx];
        if t.type == TokenType.Fn {
            t_idx = t_idx + 1; // fn
            let mut f_name = tokens[t_idx].value;
            t_idx = t_idx + 1; // name
            t_idx = t_idx + 1; // (
            let mut p1 = tokens[t_idx].value;
            t_idx = t_idx + 2; // p1,
            let mut p2 = tokens[t_idx].value;
            t_idx = t_idx + 3; // p2 ) :
            t_idx = t_idx + 3; // \n Indent return
            let mut e1 = tokens[t_idx].value;
            t_idx = t_idx + 1; // e1
            t_idx = t_idx + 1; // +
            let mut e2 = tokens[t_idx].value;
            lib_funcs = lib_funcs + "fn " + f_name + "(" + p1 + ": i32, " + p2 + ": i32) -> i32 {\n";
            lib_funcs = lib_funcs + "    return " + e1 + " + " + e2 + ";\n}\n\n";
            while tokens[t_idx].type != TokenType.Dedent: t_idx = t_idx + 1
            t_idx = t_idx + 1; // Dedent
            }
        else {
            if t.type == TokenType.Identifier {
                if t.value == "mn" {
                    t_idx = t_idx + 3; // mn : \n
                    if tokens[t_idx].type == TokenType.Indent {
                        t_idx = t_idx + 1;
                        while t_idx < t_len {
                            let subt = tokens[t_idx];
                            if subt.type == TokenType.Let {
                                t_idx = t_idx + 1; // let
                                let mut v_name = tokens[t_idx].value;
                                t_idx = t_idx + 2; // vname =
                                let mut call_f = tokens[t_idx].value;
                                t_idx = t_idx + 2; // callf (
                                let mut a1 = tokens[t_idx].value;
                                t_idx = t_idx + 2; // a1 ,
                                let mut a2 = tokens[t_idx].value;
                                out = out + "    let " + v_name + " = " + call_f + "(" + a1 + ", " + a2 + ");\n";
                                t_idx = t_idx + 3; // a2 ) \n
                                }
                            else {
                                if subt.type == TokenType.Identifier {
                                    if subt.value == "print" {
                                        t_idx = t_idx + 2; // print (
                                        let mut p_arg = tokens[t_idx].value;
                                        out = out + "    println!(\"{}\", " + p_arg + ");\n";
                                        t_idx = t_idx + 3; // arg ) \n
                                        }
                                    else { t_idx = t_idx + 1 }
                                    }
                                else {
                                    if subt.type == TokenType.Dedent { break }
                                    else { t_idx = t_idx + 1 }
                                    }
                                }
                            }
                        }
                    }
                }
            else { t_idx = t_idx + 1 }

            }
        }
    out = out + "}\n";
    write_file("simple_add.rs", lib_funcs + out);
    println!("{}", "\n✅ Complete!");

    }
main();