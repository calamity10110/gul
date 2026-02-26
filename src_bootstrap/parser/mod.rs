#![allow(unused_mut)]
// Parser module - builds AST from tokens

use crate::ast::{Type, *};
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::Eof)
    }

    fn peek(&self, offset: usize) -> &Token {
        self.tokens
            .get(self.position + offset)
            .unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.current_token() == &expected {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, found {:?}",
                expected,
                self.current_token()
            ))
        }
    }

    fn skip_newlines(&mut self) {
        while matches!(self.current_token(), Token::Newline) {
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();

        while self.current_token() != &Token::Eof {
            self.skip_newlines();

            if self.current_token() == &Token::Eof {
                break;
            }

            statements.push(self.parse_statement()?);
        }

        Ok(Program { statements })
    }

    // Context-aware parsing methods
    pub fn parse_def_file(&mut self) -> Result<Program, String> {
        self.parse_tokens_strict(|stmt| {
            matches!(
                stmt,
                Statement::Import(_)
                    | Statement::Definition { .. }
                    | Statement::GlobalDef { .. }
                    | Statement::StructDef { .. }
                    | Statement::ForeignBlock { .. } // Allow extern blocks in def files
            )
        })
    }

    pub fn parse_fnc_file(&mut self) -> Result<Program, String> {
        self.parse_tokens_strict(|stmt| {
            matches!(
                stmt,
                Statement::Function { .. } |
            Statement::Import(_) | // Allow imports in fnc for now
            Statement::ForeignBlock { .. }
            )
        })
    }

    pub fn parse_mn_file(&mut self) -> Result<Program, String> {
        self.parse_tokens_strict(|stmt| {
            matches!(stmt, Statement::Main { .. } | Statement::Import(_))
        })
    }

    fn parse_tokens_strict<F>(&mut self, validator: F) -> Result<Program, String>
    where
        F: Fn(&Statement) -> bool,
    {
        let mut statements = Vec::new();
        while self.current_token() != &Token::Eof {
            self.skip_newlines();
            if self.current_token() == &Token::Eof {
                break;
            }

            let stmt = self.parse_statement()?;
            if !validator(&stmt) {
                return Err(format!(
                    "Statement type {:?} not allowed in this file context",
                    stmt
                ));
            }
            statements.push(stmt);
        }
        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        self.skip_newlines();

        match self.current_token() {
            // v3.0 keywords (highest priority)
            Token::Let => self.parse_let_definition(),
            Token::Var => self.parse_var_definition(),

            // Import statements
            Token::Imp | Token::Import => self.parse_import(),

            // Legacy definition keywords
            Token::Def => self.parse_definition(),
            Token::Const => self.parse_const_definition(),
            Token::Mut => self.parse_mut_definition(),

            // Struct
            Token::Struct => self.parse_struct(),

            // Functions
            Token::Fn => self.parse_function(false),
            Token::Async => self.parse_function(true),
            Token::Asy => self.parse_function(true), // legacy support

            // Foreign code blocks
            Token::Extern => self.parse_extern_block(),
            Token::Cs => self.parse_custom_block(), // legacy support

            // Main entry
            Token::Main | Token::Mn => {
                if self.peek(1) == &Token::LeftParen {
                    self.parse_expression_statement()
                } else {
                    self.parse_main()
                }
            }

            // Control flow
            Token::If => self.parse_if(),
            Token::Loop => self.parse_loop(),
            Token::For => self.parse_for(false),
            Token::Parallel => self.parse_for(true),
            Token::While => self.parse_while(false),
            Token::AlsoFor => self.parse_for(true),
            Token::AlsoWhile => self.parse_while(true),
            Token::Return => self.parse_return(),
            Token::Break => {
                self.advance();
                self.skip_newlines();
                Ok(Statement::Break)
            }
            Token::Continue => {
                self.advance();
                self.skip_newlines();
                Ok(Statement::Continue)
            }
            Token::Try => self.parse_try_catch(),

            // Annotations (@)
            Token::At => self.parse_annotation_statement(),

            // Mutable assignment (?var)
            Token::QuestionMark => self.parse_mutable_assignment(),

            Token::Throw => {
                self.advance();
                let expr = self.parse_expression()?;
                self.skip_newlines();
                Ok(Statement::Throw(expr))
            }
            // If starts with (, could be parenthesized expression OR lambda params
            Token::LeftParen => self.parse_paren_or_lambda(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_paren_or_lambda(&mut self) -> Result<Statement, String> {
        // We are at '('. Check if it's a lambda or expression.
        // Simplified lookahead: (x, y) => ...
        // If we see '=>' effectively after closing paren, it's a lambda.
        // This is tricky with simple lookahead.
        // Let's assume for now if we parse parens and see '=>', it turns into lambda?
        // But expression parsing consumes tokens.
        // We can try to parse as lambda first if it looks like params?
        // Or just enhance parse_expression to handle it.
        // Actually, lambda is an Expression, not a Statement (usually).
        // But if it's top level statement? " (x) => 2 " is a valid statement expression.
        
        let expr = self.parse_expression()?;
        self.skip_newlines();
        Ok(Statement::Expression(expr))
    }

    fn parse_annotation_statement(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip '@'

        match self.current_token() {
            // v3.0 annotations
            Token::Identifier(name) if name == "imp" => self.parse_at_import(),
            Token::Imp | Token::Import => self.parse_at_import(),
            Token::Identifier(name) if name == "ui" => self.parse_at_ui(),
            Token::Identifier(name) if name == "python" => self.parse_at_lang_block("python"),
            Token::Identifier(name) if name == "rust" => self.parse_at_lang_block("rust"),
            Token::Identifier(name) if name == "sql" => self.parse_at_lang_block("sql"),
            Token::Identifier(name) if name == "js" => self.parse_at_lang_block("js"),
            Token::Identifier(name) if name == "cpp" => self.parse_at_lang_block("cpp"),
            Token::Identifier(name) if name == "test" => self.parse_test_annotation(),

            Token::Let => self.parse_let_definition(),
            Token::Var => self.parse_var_definition(),
            Token::Global => self.parse_global_def(),
            Token::Fn => self.parse_function(false),
            Token::Asy => self.parse_function(true),
            Token::Cs => self.parse_custom_block(), // @cs syntax
            _ => Err(format!("Unexpected annotation: {:?}", self.current_token())),
        }
    }

    fn parse_global_def(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'global'

        let mutable = if self.current_token() == &Token::QuestionMark {
            self.advance();
            true
        } else {
            false
        };

        if let Token::Identifier(name) = self.current_token() {
            let var_name = name.clone();
            self.advance();

            // Handle optional dotted path for state (e.g. ?game_state.high_score)
            let mut full_name = var_name;
            while self.current_token() == &Token::Dot {
                self.advance();
                if let Token::Identifier(part) = self.current_token() {
                    full_name.push('.');
                    full_name.push_str(part);
                    self.advance();
                }
            }

            self.expect(Token::Equal)?;
            let value = self.parse_expression()?;
            self.skip_newlines();

            Ok(Statement::GlobalDef {
                name: full_name,
                value,
                mutable,
            })
        } else {
            Err("Expected identifier after @global".to_string())
        }
    }

    fn parse_mutable_assignment(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip '?'

        if let Token::Identifier(name) = self.current_token() {
            let var_name = name.clone();
            self.advance();

            self.expect(Token::Equal)?;
            let value = self.parse_expression()?;
            self.skip_newlines();

            // Treat as assignment to mutable variable, or definition if new?
            // For now map to Assignment, but name includes '?' implicit check in interpreter?
            // Actually, let's map to Definition if we want it to declare.
            // But spec says "?count = 0" is mutable variable.
            // Reuse Definition node but perhaps mark it?
            // Or just convention: name starts with `?`?
            // Let's prepend `?` to name to keep convention from `parse_definition`.

            Ok(Statement::Assignment {
                name: var_name,
                value,
            })
        } else {
            Err("Expected identifier after '?'".to_string())
        }
    }

    fn parse_import(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'imp' or 'import'

        let mut modules = Vec::new();

        // Check for block style: @imp:
        if self.current_token() == &Token::Colon {
            self.advance();
            self.expect(Token::Newline)?;
            self.expect(Token::Indent)?;
            while self.current_token() != &Token::Dedent && self.current_token() != &Token::Eof {
                if matches!(self.current_token(), Token::Comma | Token::Newline) {
                    self.advance();
                    continue;
                }
                
                let m = self.parse_single_import_path()?;
                modules.extend(m);
                
                if self.current_token() == &Token::Comma {
                    self.advance();
                }
                self.skip_newlines();
            }
            self.expect(Token::Dedent)?;
        } else {
            // Inline style
            modules = self.parse_single_import_path()?;
        }

        self.skip_newlines();
        Ok(Statement::Import(modules))
    }

    fn parse_single_import_path(&mut self) -> Result<Vec<String>, String> {
        let mut modules = Vec::new();

        if let Token::Identifier(part) = self.current_token() {
            let mut module_path = part.clone();
            self.advance();

            // Handle dotted paths like std.io
            while self.current_token() == &Token::Dot {
                self.advance();
                if let Token::Identifier(next_part) = self.current_token() {
                    module_path.push('.');
                    module_path.push_str(next_part);
                    self.advance();
                }
            }

            // Handle grouped sub-imports like std{io, http}
            if matches!(self.current_token(), Token::LeftBrace | Token::LeftParen) {
                self.advance();
                while !matches!(self.current_token(), Token::RightBrace | Token::RightParen | Token::Eof) {
                    if matches!(self.current_token(), Token::Comma | Token::Newline) {
                        self.advance();
                        continue;
                    }

                    if let Token::Identifier(sub) = self.current_token() {
                        modules.push(format!("{}.{}", module_path, sub));
                        self.advance();
                    } else {
                        break;
                    }

                    if self.current_token() == &Token::Comma {
                        self.advance();
                    }
                }
                self.advance(); // Skip closer
            } else {
                modules.push(module_path);
            }
        }
        Ok(modules)
    }

    fn parse_struct(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'struct'

        if let Token::Identifier(name) = self.current_token() {
            let struct_name = name.clone();
            self.advance();

            self.expect(Token::Colon)?;
            self.expect(Token::Newline)?;
            self.expect(Token::Indent)?;

            let mut fields = Vec::new();
            let mut methods = Vec::new();

            while self.current_token() != &Token::Dedent && self.current_token() != &Token::Eof {
                self.skip_newlines();
                if self.current_token() == &Token::Dedent || self.current_token() == &Token::Eof {
                    break;
                }

                if matches!(self.current_token(), Token::Fn | Token::Asy) {
                    let is_async = self.current_token() == &Token::Asy;
                    methods.push(self.parse_function(is_async)?);
                } else if let Token::Identifier(field_name) = self.current_token() {
                    let fname = field_name.clone();
                    self.advance();
                    self.expect(Token::Colon)?;

                    let type_name = self.parse_type_annotation()?;
                    fields.push((fname, type_name));

                    self.skip_newlines();
                } else {
                    return Err(format!(
                        "Unexpected token in struct: {:?}",
                        self.current_token()
                    ));
                }
            }
            self.expect(Token::Dedent)?;

            Ok(Statement::StructDef {
                name: struct_name,
                fields,
                methods,
            })
        } else {
            Err("Expected struct name".to_string())
        }
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        // Handle optional @ prefix (v3.2)
        if self.current_token() == &Token::At {
            self.advance();
        }

        match self.current_token() {
            Token::Identifier(name) => {
                let type_name = name.clone();
                self.advance();

                match type_name.as_str() {
                    "int" => Ok(Type::Int),
                    "float" | "flt" => Ok(Type::Float),
                    "str" => Ok(Type::String),
                    "bool" => Ok(Type::Bool),
                    "any" => Ok(Type::Any),
                    "list" => {
                        if self.current_token() == &Token::Less {
                             self.advance();
                             let inner = self.parse_type()?;
                             self.expect(Token::Greater)?;
                             Ok(Type::List(Box::new(inner)))
                        } else {
                             Ok(Type::List(Box::new(Type::Any)))
                        }
                    }
                    "dict" => Ok(Type::Dict(Box::new(Type::Any), Box::new(Type::Any))),
                    _unit_name => {
                        // Check for unit types
                        if let Token::Unit(unit) = self.current_token() {
                            let unit_name = unit.clone();
                            self.advance();
                            Ok(Type::Unit(unit_name))
                        } else {
                            // Treat as custom/struct type? 
                            Ok(Type::Any) 
                        }
                    }
                }
            }
            _ => Err(format!("Expected type name, found {:?}", self.current_token())),
        }
    }

    fn parse_try_catch(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'try'

        self.expect(Token::Colon)?;
        self.skip_newlines();

        // Parse try block
        let try_body = self.parse_block()?;

        let mut catch_var = None;
        let mut catch_body = None;
        let mut finally_body = None;

        // Check for catch
        if self.current_token() == &Token::Catch {
            self.advance(); // Skip 'catch'

            // Optional exception variable
            if let Token::Identifier(var_name) = self.current_token() {
                catch_var = Some(var_name.clone());
                self.advance();
            }

            self.expect(Token::Colon)?;
            self.skip_newlines();
            catch_body = Some(self.parse_block()?);
        }

        // Check for finally
        if self.current_token() == &Token::Finally {
            self.advance(); // Skip 'finally'
            self.expect(Token::Colon)?;
            self.skip_newlines();
            finally_body = Some(self.parse_block()?);
        }

        Ok(Statement::Try {
            try_body,
            catch_var,
            catch_body,
            finally_body,
        })
    }

    fn parse_type_annotation(&mut self) -> Result<String, String> {
        if let Token::Identifier(name) = self.current_token() {
            let mut type_str = name.clone();
            self.advance();

            if self.current_token() == &Token::Less {
                self.advance();
                type_str.push('<');
                type_str.push_str(&self.parse_type_annotation()?);

                while self.current_token() == &Token::Comma {
                    self.advance();
                    type_str.push_str(", ");
                    type_str.push_str(&self.parse_type_annotation()?);
                }

                self.expect(Token::Greater)?;
                type_str.push('>');
            }
            Ok(type_str)
        } else {
            // Handle primitive types like 'int' if they are tokens, or fallback
            if let Token::Identifier(s) = self.current_token() {
                let t = s.clone();
                self.advance();
                Ok(t)
            } else {
                // For now return "any" if unknown
                Ok("any".to_string())
            }
        }
    }

    fn parse_definition(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'def'

        // Check for ownership keywords FIRST
        let mut _ownership: Option<Ownership> = None;
        if matches!(self.current_token(), Token::Own | Token::Ref | Token::Copy) {
            self.advance();
            // Map to local if needed, currently just consuming
        }

        // Check for mutability marker (?)
        let _is_mutable = if self.current_token() == &Token::QuestionMark {
            self.advance();
            true
        } else {
            false
        };

        if let Token::Identifier(name) = self.current_token() {
            let var_name = name.clone(); // Store without ? prefix
            self.advance();

            // Handle optional type annotation (@int, @str, etc.)
            if self.current_token() == &Token::Colon {
                self.advance();
                // Skip type annotation for now
                if self.current_token() == &Token::At {
                    self.advance();
                }
                if let Token::Identifier(_type_name) = self.current_token() {
                    self.advance();
                }
            }

            self.expect(Token::Equal)?;

            let value = self.parse_expression()?;
            self.skip_newlines();

            Ok(Statement::Definition {
                name: var_name,
                value,
            })
        } else {
            Err("Expected identifier after 'def'".to_string())
        }
    }

    fn parse_function(&mut self, is_async_fn: bool) -> Result<Statement, String> {
        self.advance(); // Skip 'fn', 'async', or '@'

        // v3.0: If we were at 'async', we might now be at 'fn'. Skip it too.
        if is_async_fn && self.current_token() == &Token::Fn {
            self.advance();
        }

        let func_name = match self.current_token() {
            Token::Identifier(name) => name.clone(),
            Token::Main => "main".to_string(),
            Token::Mn => "main".to_string(),
            _ => return Err(format!("Expected function name, found {:?}", self.current_token())),
        };
        self.advance();

            // Inputs
            self.expect(Token::LeftParen)?;
            let mut params = Vec::new();
            while self.current_token() != &Token::RightParen {
                let mut param_type = None;
                // Handle @type prefix in parameters
                if self.current_token() == &Token::At {
                    self.advance();
                    param_type = Some(self.parse_type()?);
                }

                if let Token::Identifier(name) = self.current_token() {
                    let param_name = name.clone();
                    self.advance();
                    // Optional suffix type annotation: param: type
                    if self.current_token() == &Token::Colon {
                        self.advance();
                        param_type = Some(self.parse_type()?);
                    }
                    params.push((param_name, param_type));

                    if self.current_token() == &Token::Comma {
                        self.advance();
                    }
                } else {
                    break;
                }
            }
            self.expect(Token::RightParen)?;

            // v3.2 Optional Outputs
            let mut outputs = Vec::new();
            if self.current_token() == &Token::LeftParen {
                self.advance();
                while self.current_token() != &Token::RightParen {
                     let mut out_type = None;
                     if self.current_token() == &Token::At {
                         self.advance();
                         out_type = Some(self.parse_type()?);
                     }
                     if let Token::Identifier(name) = self.current_token() {
                         let out_name = name.clone();
                         self.advance();
                         outputs.push((out_name, out_type));
                         if self.current_token() == &Token::Comma { self.advance(); }
                     } else {
                         // Just a type? (@int)
                         if let Some(ty) = out_type {
                              outputs.push(("".to_string(), Some(ty)));
                         } else {
                              break;
                         }
                     }
                }
                self.expect(Token::RightParen)?;
            }

            // Body
            // v3.2 supports => or = or : followed by block
            let body = if self.current_token() == &Token::FatArrow {
                eprintln!("DEBUG: parse_function found FatArrow");
                self.advance();
                let expr = self.parse_expression()?;
                vec![Statement::Return(Some(expr))]
            } else if self.current_token() == &Token::Equal {
                eprintln!("DEBUG: parse_function found Equal");
                self.advance();
                let expr = self.parse_expression()?;
                vec![Statement::Return(Some(expr))]
            } else {
                self.expect(Token::Colon)?;
                self.skip_newlines();
                self.parse_block()?
            };

            Ok(Statement::Function {
                name: func_name,
                params,
                outputs,
                body,
                is_async: is_async_fn,
            })
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = Vec::new();

        self.expect(Token::Indent)?;

        loop {
            self.skip_newlines();

            match self.current_token() {
                Token::Eof => break,
                Token::Dedent => {
                    self.advance();
                    break;
                }
                _ => {
                    statements.push(self.parse_statement()?);
                }
            }
        }

        Ok(statements)
    }

    #[allow(dead_code)]
    fn is_block_end(&self) -> bool {
        // Simple heuristic: if we see certain keywords at the start, it's likely a new block
        matches!(self.current_token(), Token::Elif | Token::Else | Token::Eof)
    }

    fn parse_custom_block(&mut self) -> Result<Statement, String> {
        // Handle both 'cs' and '@cs' syntax
        if self.current_token() == &Token::At {
            self.advance(); // Skip '@'
        }
        self.advance(); // Skip 'cs'

        if let Token::Identifier(lang) = self.current_token() {
            let language = lang.clone();
            self.advance();

            self.expect(Token::Colon)?;
            self.skip_newlines();

            // Collect all lines until we hit a top-level statement
            let mut code_lines: Vec<String> = Vec::new();

            while self.current_token() != &Token::Eof {
                if matches!(
                    self.current_token(),
                    Token::Def
                        | Token::Fn
                        | Token::Asy
                        | Token::Cs
                        | Token::Mn
                        | Token::Main
                        | Token::Imp
                        | Token::At
                ) {
                    break;
                }

                // Collect the line (simplified - just skip for now)
                // In real implementation, we'd collect the actual code text
                self.advance();

                if self.current_token() == &Token::Newline {
                    self.advance();
                    // Check if next line is dedented (starts with top-level keyword)
                    if matches!(
                        self.current_token(),
                        Token::Def
                            | Token::Fn
                            | Token::Asy
                            | Token::Cs
                            | Token::Mn
                            | Token::Main
                            | Token::Imp
                            | Token::At
                    ) {
                        break;
                    }
                }
            }

            Ok(Statement::ForeignBlock {
                language,
                code: code_lines.join("\n"),
            })
        } else {
            Err("Expected language name after 'cs'".to_string())
        }
    }

    fn parse_main(&mut self) -> Result<Statement, String> {
        let token = self.current_token().clone();
        self.advance(); // Skip 'mn' or 'main'

        match token {
            Token::Mn => {
                // v3 syntax: mn:
                // Just expect colon directly after 'mn'
                if self.current_token() == &Token::Colon {
                    self.advance(); // Skip ':'
                    self.skip_newlines();
                    let body = self.parse_block()?;
                    Ok(Statement::Main { body })
                } else if self.current_token() == &Token::LeftBracket {
                    // v3 graph syntax: mn: [...]
                    self.advance(); // Skip '['
                    self.skip_newlines();
                    // Parse graph items (simplified - treat as block for now)
                    let mut body = Vec::new();
                    while self.current_token() != &Token::RightBracket
                        && self.current_token() != &Token::Eof
                    {
                        body.push(self.parse_statement()?);
                        self.skip_newlines();
                    }
                    self.expect(Token::RightBracket)?;
                    Ok(Statement::Main { body })
                } else {
                    Err(format!(
                        "Expected ':' or '[' after 'mn', found {:?}",
                        self.current_token()
                    ))
                }
            }
            Token::Main => {
                // Deprecated v2 syntax: main():
                eprintln!("Warning: 'main' keyword is deprecated, use 'mn:' instead");
                self.expect(Token::LeftParen)?;
                self.expect(Token::RightParen)?;
                self.expect(Token::Colon)?;
                self.skip_newlines();
                let body = self.parse_block()?;
                Ok(Statement::Main { body })
            }
            _ => Err("Expected 'mn' or 'main'".to_string()),
        }
    }

    fn parse_if(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'if'

        let condition = self.parse_expression()?;
        self.expect(Token::Colon)?;
        self.skip_newlines();

        let then_body = self.parse_block()?;

        // Check for elif/else
        let mut else_body = None;

        if self.current_token() == &Token::Elif {
            // Parse elif as nested if-else
            else_body = Some(vec![self.parse_if()?]);
        } else if self.current_token() == &Token::Else {
            self.advance();
            self.expect(Token::Colon)?;
            self.skip_newlines();
            else_body = Some(self.parse_block()?);
        }

        Ok(Statement::If {
            condition,
            then_body,
            else_body,
        })
    }

    fn parse_loop(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'loop'
        self.expect(Token::Colon)?;
        self.skip_newlines();

        let body = self.parse_block()?;

        Ok(Statement::Loop { body })
    }

    fn parse_for(&mut self, is_parallel: bool) -> Result<Statement, String> {
        self.advance(); // Skip 'for'

        if let Token::Identifier(var) = self.current_token() {
            let variable = var.clone();
            self.advance();

            self.expect(Token::In)?;

            let iterable = self.parse_expression()?;
            self.expect(Token::Colon)?;
            self.skip_newlines();

            let body = self.parse_block()?;

            Ok(Statement::For {
                variable,
                iterable,
                body,
                is_parallel,
            })
        } else {
            Err("Expected variable name after 'for'".to_string())
        }
    }

    fn parse_while(&mut self, is_parallel: bool) -> Result<Statement, String> {
        self.advance(); // Skip 'while' or 'also_while'
        let condition = self.parse_expression()?;

        self.expect(Token::Colon)?;
        self.skip_newlines();

        let body = self.parse_block()?;

        Ok(Statement::While {
            condition,
            body,
            is_parallel,
        })
    }
    fn parse_return(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'return'

        // Check if there's an expression to return
        let value = if matches!(self.current_token(), Token::Newline | Token::Eof) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.skip_newlines();

        Ok(Statement::Return(value))
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, String> {
        // Check if this is an assignment (identifier = expression)
        if let Token::Identifier(name) = self.current_token() {
            let var_name = name.clone();

            // Look ahead to see if next token is '='
            if self.peek(1) == &Token::Equal {
                // This is an assignment
                self.advance(); // Skip identifier
                self.advance(); // Skip '='

                let value = self.parse_expression()?;
                self.skip_newlines();

                return Ok(Statement::Assignment {
                    name: var_name,
                    value,
                });
            }
        }

        // Otherwise, parse as expression statement
        let expr = self.parse_expression()?;
        self.skip_newlines();
        Ok(Statement::Expression(expr))
    }

    // Updated expression parsing to include Lambdas
    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_lambda_or_logical_or()
    }

    fn parse_lambda_or_logical_or(&mut self) -> Result<Expression, String> {
        // Parse left-hand side (potential params)
        let expr = self.parse_logical_or()?;

        // Check for FatArrow
        if self.current_token() == &Token::FatArrow {
            self.advance(); // consume '=>'
            
            let params = self.extract_params_from_expr(&expr)?;
            let body = self.parse_expression()?; // Recursively parse body
            
            return Ok(Expression::Lambda {
                params,
                body: Box::new(body),
            });
        }

        Ok(expr)
    }

    fn extract_params_from_expr(&self, expr: &Expression) -> Result<Vec<String>, String> {
        match expr {
            Expression::Identifier(name) => Ok(vec![name.clone()]),
            Expression::List(elements) => {
                let mut params = Vec::new();
                for e in elements {
                    if let Expression::Identifier(name) = e {
                        params.push(name.clone());
                    } else {
                        return Err("Lambda parameters must be identifiers".to_string());
                    }
                }
                Ok(params)
            }
             // Handle (param) parenthesized which might return grouping logic if applicable
             // If parse_primary groups (x) as x (identifier), it falls into first match.
             // If (a,b) is generic List, it falls into second match.
            _ => Err("Invalid lambda parameters".to_string()),
        }
    }

    fn parse_logical_or(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_logical_and()?;

        while self.current_token() == &Token::Or {
            self.advance();
            let right = self.parse_logical_and()?;
            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_equality()?;

        while self.current_token() == &Token::And {
            self.advance();
            let right = self.parse_equality()?;
            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_comparison()?;

        while matches!(self.current_token(), Token::EqualEqual | Token::BangEqual) {
            let op = match self.current_token() {
                Token::EqualEqual => BinaryOp::Equal,
                Token::BangEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_addition()?;

        while matches!(
            self.current_token(),
            Token::Less | Token::LessEqual | Token::Greater | Token::GreaterEqual
        ) {
            let op = match self.current_token() {
                Token::Less => BinaryOp::Less,
                Token::LessEqual => BinaryOp::LessEqual,
                Token::Greater => BinaryOp::Greater,
                Token::GreaterEqual => BinaryOp::GreaterEqual,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_addition()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_addition(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_multiplication()?;

        while matches!(self.current_token(), Token::Plus | Token::Minus) {
            let op = match self.current_token() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_multiplication()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_multiplication(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_power()?;

        while matches!(
            self.current_token(),
            Token::Star | Token::Slash | Token::Percent
        ) {
            let op = match self.current_token() {
                Token::Star => BinaryOp::Multiply,
                Token::Slash => BinaryOp::Divide,
                Token::Percent => BinaryOp::Modulo,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_power()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_unary()?;

        if self.current_token() == &Token::Caret {
            self.advance();
            let right = self.parse_power()?; // Right associative
            left = Expression::Binary {
                left: Box::new(left),
                op: BinaryOp::Power,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expression, String> {
        match self.current_token() {
            Token::Not => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expression::Unary {
                    op: UnaryOp::Not,
                    operand: Box::new(operand),
                })
            }
            Token::Minus => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expression::Unary {
                    op: UnaryOp::Negate,
                    operand: Box::new(operand),
                })
            }
            Token::Await => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expression::Await(Box::new(operand)))
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_primary()?;

        loop {
            match self.current_token() {
                // v2.1: All bracket types can be used for function calls
                Token::LeftParen | Token::LeftBracket | Token::LeftBrace => {
                    let open_bracket = self.current_token().clone();

                    // Determine if this is a function call or index access
                    // For identifiers followed by brackets, treat as function call
                    let is_function_call =
                        matches!(expr, Expression::Identifier(_) | Expression::Member { .. });

                    if is_function_call || open_bracket == Token::LeftParen {
                        // Function call with any bracket type
                        self.advance();
                        let mut args = Vec::new();

                        while !self.current_token().is_close_bracket() {
                            // Check for named parameters (key=value or key:value)
                            args.push(self.parse_expression()?);

                            if self.current_token() == &Token::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }

                        // v2.1: Validate bracket matching
                        if !open_bracket.brackets_match(self.current_token()) {
                            return Err(format!(
                                "Mismatched brackets: opened with {:?}, closed with {:?}",
                                open_bracket,
                                self.current_token()
                            ));
                        }
                        self.advance(); // consume closing bracket

                        expr = Expression::Call {
                            function: Box::new(expr),
                            args,
                        };
                    } else {
                        // Index access (only for LeftBracket in non-function context)
                        self.advance();
                        let index = self.parse_expression()?;

                        // v2.1: Accept any matching closing bracket
                        if !open_bracket.brackets_match(self.current_token()) {
                            return Err(format!(
                                "Mismatched brackets: opened with {:?}, closed with {:?}",
                                open_bracket,
                                self.current_token()
                            ));
                        }
                        self.advance();

                        expr = Expression::Index {
                            object: Box::new(expr),
                            index: Box::new(index),
                        };
                    }
                }
                Token::Dot => {
                    // Member access
                    self.advance();

                    if let Token::Identifier(member) = self.current_token() {
                        let member_name = member.clone();
                        self.advance();

                        expr = Expression::Member {
                            object: Box::new(expr),
                            member: member_name,
                        };
                    } else {
                        return Err("Expected member name after '.'".to_string());
                    }
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        match self.current_token() {
            Token::Integer(n) => {
                let value = *n;
                self.advance();
                Ok(Expression::Integer(value))
            }
            Token::Float(n) => {
                let value = *n;
                self.advance();
                Ok(Expression::Float(value))
            }
            Token::String(s) => {
                let value = s.clone();
                self.advance();
                Ok(Expression::String(value))
            }
            Token::Bool(b) => {
                let value = *b;
                self.advance();
                Ok(Expression::Bool(value))
            }
            Token::Identifier(name) => {
                let id = name.clone();
                self.advance();
                Ok(Expression::Identifier(id))
            }
            Token::Main | Token::Mn => {
                self.advance();
                Ok(Expression::Identifier("main".to_string()))
            }
            Token::UiSprite(content) => {
                let sprite_content = content.clone();
                self.advance();

                // Parser for UI components (simplified)
                // Format: sprite_type{prop1=val1, prop2=val2} or just sprite_type
                let parts: Vec<&str> = sprite_content.splitn(2, '{').collect();
                let sprite_type = parts[0].to_string();
                let properties = Vec::<(String, Expression)>::new(); // TODO: Parse properties

                Ok(Expression::UiSprite {
                    sprite_type,
                    properties,
                })
            }
            Token::Await => {
                self.advance();
                let expr = self.parse_expression()?;
                Ok(Expression::Await(Box::new(expr)))
            }
            Token::At => {
                self.advance();
                if let Token::Identifier(name) = self.current_token() {
                    let id = format!("@{}", name);
                    self.advance();
                    Ok(Expression::Identifier(id))
                } else {
                    Err("Expected built-in name after '@'".to_string())
                }
            }
            // v2.1: All bracket types can be used for grouping or collections
            Token::LeftParen | Token::LeftBracket | Token::LeftBrace => {
                let open_bracket = self.current_token().clone();
                self.advance();

                // Check if empty
                if open_bracket.brackets_match(self.current_token()) {
                    self.advance();
                    return Ok(Expression::List(Vec::new()));
                }

                // Parse first element
                let first_expr = self.parse_expression()?;

                // Check what follows to determine type
                match self.current_token() {
                    // Colon after first element = dictionary
                    Token::Colon => {
                        self.advance();
                        let first_value = self.parse_expression()?;
                        let mut pairs = vec![];

                        // Extract key from first_expr
                        let key = match first_expr {
                            Expression::Identifier(k) => k,
                            _ => return Err("Dictionary key must be identifier".to_string()),
                        };
                        pairs.push((key, first_value));

                        // Parse remaining pairs
                        while self.current_token() == &Token::Comma {
                            self.advance();
                            if open_bracket.brackets_match(self.current_token()) {
                                break; // trailing comma
                            }
                            if let Token::Identifier(key) = self.current_token() {
                                let key_name = key.clone();
                                self.advance();
                                self.expect(Token::Colon)?;
                                let value = self.parse_expression()?;
                                pairs.push((key_name, value));
                            } else {
                                return Err("Expected key identifier in dict".to_string());
                            }
                        }

                        // v2.1: Validate bracket matching
                        if !open_bracket.brackets_match(self.current_token()) {
                            return Err(format!(
                                "Mismatched brackets: opened with {:?}, closed with {:?}",
                                open_bracket,
                                self.current_token()
                            ));
                        }
                        self.advance();
                        Ok(Expression::Dict(pairs))
                    }
                    // Comma = list
                    Token::Comma => {
                        let mut elements = vec![first_expr];
                        while self.current_token() == &Token::Comma {
                            self.advance();
                            if open_bracket.brackets_match(self.current_token()) {
                                break; // trailing comma
                            }
                            elements.push(self.parse_expression()?);
                        }

                        // v2.1: Validate bracket matching
                        if !open_bracket.brackets_match(self.current_token()) {
                            return Err(format!(
                                "Mismatched brackets: opened with {:?}, closed with {:?}",
                                open_bracket,
                                self.current_token()
                            ));
                        }
                        self.advance();
                        Ok(Expression::List(elements))
                    }
                    // Closing bracket = single element (grouping or single-element list)
                    _ if open_bracket.brackets_match(self.current_token()) => {
                        self.advance();
                        // For () this is grouping, for [] and {} this is single-element list
                        if open_bracket == Token::LeftParen {
                            Ok(first_expr) // Grouping
                        } else {
                            Ok(Expression::List(vec![first_expr])) // Single element list
                        }
                    }
                    _ => {
                        // v2.1: Check for mismatched brackets
                        if self.current_token().is_close_bracket() {
                            return Err(format!(
                                "Mismatched brackets: opened with {:?}, closed with {:?}",
                                open_bracket,
                                self.current_token()
                            ));
                        }
                        Err(format!(
                            "Unexpected token in collection: {:?}",
                            self.current_token()
                        ))
                    }
                }
            }
            _ => Err(format!(
                "Unexpected token in expression: {:?}",
                self.current_token()
            )),
        }
    }

    // v2.0 parsing methods
    fn parse_const_definition(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'const'

        if let Token::Identifier(name) = self.current_token() {
            let var_name = name.clone();
            self.advance();

            // Optional type annotation
            let type_annotation = if self.current_token() == &Token::Colon {
                self.advance();
                Some(self.parse_type()?)
            } else {
                None
            };

            self.expect(Token::Equal)?;
            let mut value = self.parse_expression()?;
            self.skip_newlines();

            // If we have a type annotation, wrap the expression
            if let Some(ty) = type_annotation {
                value = Expression::Typed {
                    expr: Box::new(value),
                    ty,
                };
            }

            Ok(Statement::Definition {
                name: var_name,
                value,
            })
        } else {
            Err("Expected identifier after 'const'".to_string())
        }
    }

    fn parse_mut_definition(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'mut'

        if let Token::Identifier(name) = self.current_token() {
            let var_name = name.clone();
            self.advance();

            // Optional type annotation
            let type_annotation = if self.current_token() == &Token::Colon {
                self.advance();
                Some(self.parse_type()?)
            } else {
                None
            };

            self.expect(Token::Equal)?;
            let mut value = self.parse_expression()?;
            self.skip_newlines();

            // If we have a type annotation, wrap the expression
            if let Some(ty) = type_annotation {
                value = Expression::Typed {
                    expr: Box::new(value),
                    ty,
                };
            }

            Ok(Statement::Definition {
                name: var_name,
                value,
            })
        } else {
            Err("Expected identifier after 'mut'".to_string())
        }
    }

    // v3.0 parsing methods
    fn parse_let_definition(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'let'

        if let Token::Identifier(name) = self.current_token() {
            let var_name = name.clone();
            self.advance();

            // Optional type annotation
            let type_annotation = if self.current_token() == &Token::Colon {
                self.advance();
                Some(self.parse_type()?)
            } else {
                None
            };

            self.expect(Token::Equal)?;
            let mut value = self.parse_expression()?;
            self.skip_newlines();

            // If we have a type annotation, wrap the expression
            if let Some(ty) = type_annotation {
                value = Expression::Typed {
                    expr: Box::new(value),
                    ty,
                };
            }

            Ok(Statement::Definition {
                name: var_name,
                value,
            })
        } else {
            Err("Expected identifier after 'let'".to_string())
        }
    }

    fn parse_var_definition(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'var'

        if let Token::Identifier(name) = self.current_token() {
            let var_name = name.clone();
            self.advance();

            // Optional type annotation
            let type_annotation = if self.current_token() == &Token::Colon {
                self.advance();
                Some(self.parse_type()?)
            } else {
                None
            };

            self.expect(Token::Equal)?;
            let mut value = self.parse_expression()?;
            self.skip_newlines();

            // If we have a type annotation, wrap the expression
            if let Some(ty) = type_annotation {
                value = Expression::Typed {
                    expr: Box::new(value),
                    ty,
                };
            }

            // For v3.0, 'var' creates a mutable variable
            // We use the same Definition node but could track mutability in AST
            Ok(Statement::Definition {
                name: var_name,
                value,
            })
        } else {
            Err("Expected identifier after 'var'".to_string())
        }
    }


    fn parse_at_import(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'imp'

        // v3.0 @imp syntax:
        // Style 1: @imp std{io} - single line
        // Style 2: @imp: (followed by indented list)
        // Style 3: @imp(std{io}, python{numpy}) - parenthesized

        if self.current_token() == &Token::Colon {
            // Style 2: Block style with colon
            // @imp:
            //     std.io,
            //     std.http
            self.advance(); // Skip ':'
            self.skip_newlines();

            let mut modules = Vec::new();
            // Parse indented list until dedent
            if self.current_token() == &Token::Indent {
                self.advance();
                while self.current_token() != &Token::Dedent && self.current_token() != &Token::Eof
                {
                    if matches!(self.current_token(), Token::Comma | Token::Newline) {
                        self.advance();
                        continue;
                    }
                    if let Token::Identifier(part) = self.current_token() {
                        let mut module_path = part.clone();
                        self.advance();
                        while self.current_token() == &Token::Dot {
                            self.advance();
                            if let Token::Identifier(next) = self.current_token() {
                                module_path.push('.');
                                module_path.push_str(next);
                                self.advance();
                            }
                        }
                        modules.push(module_path);
                    } else {
                        self.advance();
                    }
                }
                if self.current_token() == &Token::Dedent {
                    self.advance();
                }
            }
            return Ok(Statement::Import(modules));
        }

        if self.current_token() == &Token::LeftParen {
            // Style 3: Parenthesized - @imp(std.io, python{numpy})
            self.advance();
            let mut modules = Vec::new();
            while !matches!(self.current_token(), Token::RightParen | Token::Eof) {
                if matches!(self.current_token(), Token::Comma | Token::Newline) {
                    self.advance();
                    continue;
                }
                if let Token::Identifier(part) = self.current_token() {
                    let mut module_path = part.clone();
                    self.advance();
                    while self.current_token() == &Token::Dot {
                        self.advance();
                        if let Token::Identifier(next) = self.current_token() {
                            module_path.push('.');
                            module_path.push_str(next);
                            self.advance();
                        }
                    }
                    // Handle grouped imports like python{numpy}
                    if self.current_token() == &Token::LeftBrace {
                        self.advance();
                        while !matches!(self.current_token(), Token::RightBrace | Token::Eof) {
                            if let Token::Identifier(sub) = self.current_token() {
                                modules.push(format!("{}:{}", module_path, sub));
                            }
                            self.advance();
                        }
                        if self.current_token() == &Token::RightBrace {
                            self.advance();
                        }
                    } else {
                        modules.push(module_path);
                    }
                } else {
                    self.advance();
                }
            }
            if self.current_token() == &Token::RightParen {
                self.advance();
            }
            self.skip_newlines();
            return Ok(Statement::Import(modules));
        }

        // Style 1: Single line - same as regular import
        if let Token::Identifier(first_part) = self.current_token() {
            let mut module_path = first_part.clone();
            self.advance();

            // Handle grouped imports: std{io, http}
            if self.current_token() == &Token::LeftBrace {
                self.advance();
                module_path.push('{');
                while !matches!(self.current_token(), Token::RightBrace | Token::Eof) {
                    if let Token::Identifier(part) = self.current_token() {
                        module_path.push_str(part);
                    }
                    self.advance();
                    if self.current_token() == &Token::Comma {
                        module_path.push(',');
                        self.advance();
                    }
                }
                module_path.push('}');
                if self.current_token() == &Token::RightBrace {
                    self.advance();
                }
            }

            // Handle dotted imports
            while self.current_token() == &Token::Dot {
                self.advance();
                module_path.push('.');
                if let Token::Identifier(part) = self.current_token() {
                    module_path.push_str(part);
                    self.advance();
                }
            }

            self.skip_newlines();
            Ok(Statement::Import(vec![module_path]))
        } else {
            Err("Expected module name after '@imp'".to_string())
        }
    }

    fn parse_at_ui(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'ui'

        // Parse component name and properties
        // @ui button{text: "Click"}
        if let Token::Identifier(component) = self.current_token() {
            let component_name = component.clone();
            self.advance();

            // Parse properties in braces
            let mut props = vec![];
            if self.current_token() == &Token::LeftBrace {
                self.advance();
                while !matches!(self.current_token(), Token::RightBrace | Token::Eof) {
                    self.skip_newlines();
                    if let Token::Identifier(key) = self.current_token() {
                        let k = key.clone();
                        self.advance();
                        self.expect(Token::Colon)?;
                        let val = self.parse_expression()?;
                        props.push((k, val));
                        
                        if self.current_token() == &Token::Comma {
                            self.advance();
                        }
                    } else {
                        break;
                    }
                    self.skip_newlines();
                }
                self.expect(Token::RightBrace)?;
            }

            self.skip_newlines();

            // Return as expression statement with UI component call
            // ui.component_name({prop1: val1, ...})
            Ok(Statement::Expression(Expression::Call {
                function: Box::new(Expression::Member {
                    object: Box::new(Expression::Identifier("ui".to_string())),
                    member: component_name,
                }),
                args: vec![Expression::Dict(props)],
            }))
        } else {
            Err("Expected component name after '@ui'".to_string())
        }
    }

    fn parse_at_lang_block(&mut self, language: &str) -> Result<Statement, String> {
        self.advance(); // Skip language identifier (python/rust/sql/etc.)

        // Expect { to start the block
        self.expect(Token::LeftBrace)?;

        // Collect code until closing brace
        let mut code = String::new();
        let mut brace_depth = 1;

        while self.current_token() != &Token::Eof && brace_depth > 0 {
            match self.current_token() {
                Token::LeftBrace => {
                    brace_depth += 1;
                    code.push('{');
                    self.advance();
                }
                Token::RightBrace => {
                    brace_depth -= 1;
                    if brace_depth > 0 {
                        code.push('}');
                    }
                    self.advance();
                }
                Token::Newline => {
                    code.push('\n');
                    self.advance();
                }
                Token::String(s) => {
                    code.push('"');
                    code.push_str(s);
                    code.push('"');
                    self.advance();
                }
                Token::Identifier(id) => {
                    code.push_str(id);
                    code.push(' ');
                    self.advance();
                }
                // Handle Keywords
                Token::Import | Token::Imp => { code.push_str("import "); self.advance(); }
                Token::Fn => { code.push_str("fn "); self.advance(); }
                Token::Def => { code.push_str("def "); self.advance(); } 
                Token::Return => { code.push_str("return "); self.advance(); }
                Token::If => { code.push_str("if "); self.advance(); }
                Token::Else => { code.push_str("else "); self.advance(); }
                Token::Elif => { code.push_str("elif "); self.advance(); }
                Token::For => { code.push_str("for "); self.advance(); }
                Token::While => { code.push_str("while "); self.advance(); }
                Token::Loop => { code.push_str("loop "); self.advance(); }
                Token::Break => { code.push_str("break "); self.advance(); }
                Token::Continue => { code.push_str("continue "); self.advance(); }
                Token::Try => { code.push_str("try "); self.advance(); }
                Token::Catch => { code.push_str("catch "); self.advance(); }
                Token::Finally => { code.push_str("finally "); self.advance(); }
                Token::Throw => { code.push_str("throw "); self.advance(); }
                Token::Const => { code.push_str("const "); self.advance(); }
                Token::Let => { code.push_str("let "); self.advance(); }
                Token::Mut => { code.push_str("mut "); self.advance(); }
                Token::Var => { code.push_str("var "); self.advance(); }
                Token::Struct => { code.push_str("struct "); self.advance(); }
                Token::Main | Token::Mn => { code.push_str("main "); self.advance(); }
                
                Token::LeftParen => {
                    code.push('(');
                    self.advance();
                }
                Token::RightParen => {
                    code.push(')');
                    self.advance();
                }
                Token::LeftBracket => {
                    code.push('[');
                    self.advance();
                }
                Token::RightBracket => {
                    code.push(']');
                    self.advance();
                }
                Token::Comma => {
                    code.push(',');
                    code.push(' ');
                    self.advance();
                }
                Token::Dot => {
                    code.push('.');
                    self.advance();
                }
                Token::Colon => {
                    code.push(':');
                    code.push(' ');
                    self.advance();
                }
                Token::Equal => {
                    code.push(' ');
                    code.push('=');
                    code.push(' ');
                    self.advance();
                }
                // Handle Literals
                Token::Integer(i) => {
                    code.push_str(&i.to_string());
                    self.advance();
                }
                Token::Float(f) => {
                    code.push_str(&f.to_string());
                    self.advance();
                }
                Token::Bool(b) => {
                    code.push_str(&b.to_string());
                    self.advance();
                }
                // Handle basic operators
                Token::Plus => { code.push('+'); self.advance(); }
                Token::Minus => { code.push('-'); self.advance(); }
                Token::Star => { code.push('*'); self.advance(); }
                Token::Slash => { code.push('/'); self.advance(); }
                Token::Percent => { code.push('%'); self.advance(); }
                Token::Not => { code.push('!'); self.advance(); }
                
                // Fallback for others
                _ => {
                    // Try to approximate string representation if possible, or just skip?
                    // Skipping causes syntax errors in foreign code.
                    // Better to insert a placeholder or try to print debug?
                    // For now, let's assume unknown tokens are rare in simple foreign blocks
                    // or add more cases as needed.
                    self.advance();
                }
            }
        }

        self.skip_newlines();

        Ok(Statement::ForeignBlock {
            language: language.to_string(),
            code,
        })
    }

    fn parse_test_annotation(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'test'
        self.skip_newlines();

        // Next should be a function definition
        if self.current_token() == &Token::Fn {
            self.parse_function(false)
        } else {
            Err("Expected 'fn' after '@test'".to_string())
        }
    }

    fn parse_extern_block(&mut self) -> Result<Statement, String> {
        self.advance(); // Skip 'extern'

        // Parse language specifier
        let language = if let Token::Identifier(lang) = self.current_token() {
            let lang_name = lang.clone();
            self.advance();
            lang_name
        } else {
            return Err("Expected language identifier after 'extern'".to_string());
        };

        // Parse block content - for now, expect a simple block
        self.expect(Token::LeftBrace)?;

        // For v2.0, we'll parse function definitions within extern blocks
        // For now, collect everything until closing brace
        let mut code = String::new();
        let mut brace_depth = 1;

        while self.current_token() != &Token::Eof && brace_depth > 0 {
            match self.current_token() {
                Token::LeftBrace => {
                    brace_depth += 1;
                    code.push('{');
                    self.advance();
                }
                Token::RightBrace => {
                    brace_depth -= 1;
                    if brace_depth > 0 {
                        code.push('}');
                    }
                    self.advance();
                }
                Token::Newline => {
                    code.push('\n');
                    self.advance();
                }
                Token::String(s) => {
                    code.push('"');
                    code.push_str(s);
                    code.push('"');
                    self.advance();
                }
                Token::Identifier(id) => {
                    code.push_str(id);
                    self.advance();
                }
                _ => {
                    // For other tokens, just add their string representation
                    code.push_str(&format!("{:?}", self.current_token()).to_lowercase());
                    self.advance();
                }
            }
        }

        Ok(Statement::ForeignBlock {
            language,
            code: code.trim().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_simple_definition() {
        let mut lexer = Lexer::new("def x = 10");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Definition { name, .. } => assert_eq!(name, "x"),
            _ => panic!("Expected definition"),
        }
    }

    #[test]
    fn test_parse_expression() {
        let mut lexer = Lexer::new("def result = 2 + 3 * 4");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_parse_function() {
        let mut lexer = Lexer::new("fn add(a, b):\n    return a + b");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Function { name, params, .. } => {
                assert_eq!(name, "add");
                assert_eq!(params.len(), 2);
            }
            _ => panic!("Expected function"),
        }
    }

    #[test]
    fn test_parse_async_function() {
        let mut lexer =
            Lexer::new("asy fetch(url):\n    res = await http.get(url)\n    return res.text()");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Function { name, is_async, params, outputs, .. } => {
                let p_str = params.iter().map(|p| p.0.clone()).collect::<Vec<_>>().join(", ");
                let o_str = if outputs.is_empty() { "".to_string() } else { format!("({})", outputs.iter().map(|o| o.0.clone()).collect::<Vec<_>>().join(", ")) };
                format!("{}fn {}({}) {} {{ ... }}", if *is_async { "async " } else { "" }, name, p_str, o_str);
            }
            _ => panic!("Expected async function"),
        }
    }

    #[test]
    #[ignore] // Ownership keywords in parameters are deprecated in v3.2
    fn test_parse_ownership_in_parameters() {
        let mut lexer = Lexer::new("fn process(own data, ref config):\n    result = data + config.value\n    return result");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Function { name, params, .. } => {
                assert_eq!(name, "process");
                assert_eq!(params.len(), 2);
                // Note: ownership parsing would need to be implemented in the AST
            }
            _ => panic!("Expected function with ownership parameters"),
        }
    }

    #[test]
    fn test_parse_ui_sprite_expression() {
        let mut lexer = Lexer::new("def slider = ^&^[slider{min=0, max=100, value=50}]");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Definition { name, value } => {
                assert_eq!(name, "slider");
                // Check if the value is a UiSprite expression
                match value {
                    Expression::UiSprite { .. } => {} // Success
                    _ => panic!("Expected UiSprite expression"),
                }
            }
            _ => panic!("Expected definition with UI sprite"),
        }
    }

    #[test]
    fn test_parse_custom_block() {
        let mut lexer = Lexer::new("cs rust:\n    let x = 5");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::ForeignBlock { language, .. } => {
                assert_eq!(language, "rust");
            }
            _ => panic!("Expected custom block"),
        }
    }

    #[test]
    fn test_parse_if_elif_else() {
        let mut lexer =
            Lexer::new("if x > 0:\n    return x\nelif x < 0:\n    return -x\nelse:\n    return 0");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::If {
                then_body,
                else_body,
                ..
            } => {
                assert_eq!(then_body.len(), 1);
                assert!(else_body.is_some());
            }
            _ => panic!("Expected if statement"),
        }
    }

    #[test]
    fn test_parse_for_loop() {
        let mut lexer = Lexer::new("for item in collection:\n    process(item)");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::For { variable, .. } => {
                assert_eq!(variable, "item");
            }
            _ => panic!("Expected for loop"),
        }
    }

    #[test]
    fn test_parse_while_loop() {
        let mut lexer = Lexer::new("while condition:\n    update()");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::While { .. } => {
                // Success
            }
            _ => panic!("Expected while loop"),
        }
    }

    #[test]
    fn test_parse_main_function() {
        // v3 syntax: just `mn:` without main() or parentheses
        let mut lexer = Lexer::new("mn:\n    print(\"Hello, World!\")\n    ui.print(^&^[tree])\n    data = await fetch(\"https://api.example.com\")\n    print(data)");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Main { .. } => {
                // Success
            }
            _ => panic!("Expected main function"),
        }
    }

    #[test]
    fn test_parse_import_statement() {
        // Test simple import (just module name, not dotted path)
        let mut lexer = Lexer::new("imp std");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Import(modules) => {
                assert_eq!(modules.len(), 1);
                assert_eq!(modules[0], "std");
            }
            _ => panic!("Expected import statement"),
        }
    }

    #[test]
    fn test_parse_complex_expression() {
        let mut lexer = Lexer::new("def result = (a + b) * c / d - e");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        // The expression parsing should handle operator precedence correctly
    }

    #[test]
    fn test_parse_nested_function_calls() {
        let mut lexer = Lexer::new("def result = max(min(a, b), c)");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        // Should parse nested function calls correctly
    }

    // v2.1 Bracket Equivalence Parser Tests
    #[test]
    fn test_v21_function_call_with_brackets() {
        // Function call using [] instead of ()
        let mut lexer = Lexer::new("def result = print[\"hello\"]");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Definition { name, value } => {
                assert_eq!(name, "result");
                match value {
                    Expression::Call { function: _, args } => {
                        assert_eq!(args.len(), 1);
                    }
                    _ => panic!("Expected function call"),
                }
            }
            _ => panic!("Expected definition"),
        }
    }

    #[test]
    fn test_v21_function_call_with_braces() {
        // Function call using {} instead of ()
        let mut lexer = Lexer::new("def result = print{\"hello\"}");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Definition { name, value } => {
                assert_eq!(name, "result");
                match value {
                    Expression::Call { function: _, args } => {
                        assert_eq!(args.len(), 1);
                    }
                    _ => panic!("Expected function call"),
                }
            }
            _ => panic!("Expected definition"),
        }
    }

    #[test]
    fn test_v21_list_with_parens() {
        // List using () instead of []
        let mut lexer = Lexer::new("def data = (1, 2, 3)");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Definition { name, value } => {
                assert_eq!(name, "data");
                match value {
                    Expression::List(elements) => {
                        assert_eq!(elements.len(), 3);
                    }
                    _ => panic!("Expected list"),
                }
            }
            _ => panic!("Expected definition"),
        }
    }

    #[test]
    fn test_v21_list_with_braces() {
        // List using {} instead of []
        let mut lexer = Lexer::new("def data = {1, 2, 3}");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Definition { name, value } => {
                assert_eq!(name, "data");
                match value {
                    Expression::List(elements) => {
                        assert_eq!(elements.len(), 3);
                    }
                    _ => panic!("Expected list"),
                }
            }
            _ => panic!("Expected definition"),
        }
    }

    #[test]
    fn test_v21_dict_with_brackets() {
        // Dict using [] instead of {}
        let mut lexer = Lexer::new("def config = [host: \"localhost\", port: 8080]");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Definition { name, value } => {
                assert_eq!(name, "config");
                match value {
                    Expression::Dict(pairs) => {
                        assert_eq!(pairs.len(), 2);
                        assert_eq!(pairs[0].0, "host");
                        assert_eq!(pairs[1].0, "port");
                    }
                    _ => panic!("Expected dict"),
                }
            }
            _ => panic!("Expected definition"),
        }
    }

    #[test]
    fn test_v21_empty_collections() {
        // Empty list with different brackets
        let mut lexer = Lexer::new("def empty = []");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        match &program.statements[0] {
            Statement::Definition { value, .. } => match value {
                Expression::List(elements) => assert_eq!(elements.len(), 0),
                _ => panic!("Expected empty list"),
            },
            _ => panic!("Expected definition"),
        }
    }

    #[test]
    fn test_v21_mixed_bracket_nesting() {
        // Nested with different bracket types
        let mut lexer = Lexer::new("def x = func{a, [1, 2], (3, 4)}");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Definition { name, value } => {
                assert_eq!(name, "x");
                match value {
                    Expression::Call { args, .. } => {
                        assert_eq!(args.len(), 3);
                    }
                    _ => panic!("Expected function call"),
                }
            }
            _ => panic!("Expected definition"),
        }
    }
}
