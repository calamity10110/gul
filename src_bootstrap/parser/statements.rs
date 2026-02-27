// Statement parsing methods for Parser

use crate::ast::{Type, *};
use crate::lexer::Token;
use super::Parser;

impl Parser {
    pub(crate) fn parse_statement(&mut self) -> Result<Statement, String> {
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

    pub(crate) fn parse_block(&mut self) -> Result<Vec<Statement>, String> {
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

    pub(crate) fn parse_expression_statement(&mut self) -> Result<Statement, String> {
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
