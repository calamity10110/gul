use crate::backend::interpreter::{Interpreter, Value};
use crate::frontend::lexer::Lexer;
use crate::frontend::parser::Parser;

/// A single entry in the output history
#[derive(Clone, Debug)]
pub enum OutputEntry {
    Input(String),
    Continuation(String),
    Result(String),
    Error(String),
}

/// Variable info for sidebar display
#[derive(Clone, Debug)]
pub struct VarInfo {
    pub name: String,
    pub type_name: String,
    pub short_value: String,
}

/// Core REPL state
pub struct ReplState {
    pub interpreter: Interpreter,
    pub output: Vec<OutputEntry>,
    pub history: Vec<String>,
    pub history_pos: Option<usize>,
    pub input: String,
    pub cursor_col: usize,
    pub multiline_buf: Vec<String>,
    pub in_multiline: bool,
    pub loaded_file: Option<String>,
    pub project_name: String,
}

impl ReplState {
    pub fn new(project_name: String) -> Self {
        Self {
            interpreter: Interpreter::new(),
            output: Vec::new(),
            history: Vec::new(),
            history_pos: None,
            input: String::new(),
            cursor_col: 0,
            multiline_buf: Vec::new(),
            in_multiline: false,
            loaded_file: None,
            project_name,
        }
    }

    /// Evaluate a line of GUL code. Returns true if :quit was requested.
    pub fn eval_line(&mut self, line: &str) -> bool {
        // Handle special commands
        if line.starts_with(':') {
            return self.handle_command(line);
        }

        // Check for multi-line start (line ends with ':')
        let trimmed = line.trim();
        if trimmed.ends_with(':') && !self.in_multiline {
            self.in_multiline = true;
            self.multiline_buf.push(line.to_string());
            self.output.push(OutputEntry::Input(line.to_string()));
            return false;
        }

        // In multi-line mode
        if self.in_multiline {
            if trimmed.is_empty() {
                // Empty line submits the block
                self.in_multiline = false;
                let full_source = self.multiline_buf.join("\n");
                self.multiline_buf.clear();
                self.evaluate_source(&full_source);
                return false;
            } else {
                self.multiline_buf.push(line.to_string());
                self.output.push(OutputEntry::Continuation(line.to_string()));
                return false;
            }
        }

        // Single-line evaluation
        self.output.push(OutputEntry::Input(line.to_string()));
        self.evaluate_source(line);
        self.history.push(line.to_string());
        self.history_pos = None;
        false
    }

    fn evaluate_source(&mut self, source: &str) {
        // Wrap bare expressions in mn: block for evaluation
        let wrapped = if source.contains("mn:") || source.contains("@fn ")
            || source.contains("const ") || source.contains("var ")
            || source.contains("@imp ") || source.contains("@type ") {
            source.to_string()
        } else {
            format!("mn:\n    print({})", source)
        };

        let mut lexer = Lexer::new(&wrapped);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Ok(program) => {
                match self.interpreter.run(&program) {
                    Ok(()) => {}
                    Err(e) => {
                        self.output.push(OutputEntry::Error(e));
                    }
                }
            }
            Err(e) => {
                self.output.push(OutputEntry::Error(format!("Parse error: {}", e)));
            }
        }
    }

    fn handle_command(&mut self, cmd: &str) -> bool {
        let parts: Vec<&str> = cmd.trim().splitn(2, ' ').collect();
        match parts[0] {
            ":help" => {
                self.output.push(OutputEntry::Result("Commands: :help :clear :load <file> :reset :quit".to_string()));
                self.output.push(OutputEntry::Result("Keys: Up/Down=history, Ctrl-C=cancel, Ctrl-L=clear, Ctrl-D=quit".to_string()));
            }
            ":clear" => {
                self.output.clear();
            }
            ":reset" => {
                self.interpreter = Interpreter::new();
                self.output.push(OutputEntry::Result("State reset.".to_string()));
            }
            ":load" => {
                if parts.len() > 1 {
                    let path = parts[1].trim();
                    match std::fs::read_to_string(path) {
                        Ok(source) => {
                            self.loaded_file = Some(path.to_string());
                            self.evaluate_source(&source);
                            self.output.push(OutputEntry::Result(format!("Loaded: {}", path)));
                        }
                        Err(e) => {
                            self.output.push(OutputEntry::Error(format!("Failed to load: {}", e)));
                        }
                    }
                } else {
                    self.output.push(OutputEntry::Error("Usage: :load <file.mn>".to_string()));
                }
            }
            ":quit" => return true,
            _ => {
                self.output.push(OutputEntry::Error(format!("Unknown command: {}", parts[0])));
            }
        }
        false
    }

    /// Get variables for sidebar display
    pub fn get_variables(&self) -> Vec<VarInfo> {
        self.interpreter.variables.iter()
            .filter(|(_, v)| !matches!(v, Value::NativeFunction(_)))
            .map(|(name, value)| {
                let type_name = match value {
                    Value::Integer(_) => "int",
                    Value::Float(_) => "float",
                    Value::String(_) => "str",
                    Value::Bool(_) => "bool",
                    Value::List(_) => "list",
                    Value::Dict(_) => "dict",
                    Value::Function(_, _, _) => "fn",
                    Value::Lambda(_, _) => "lambda",
                    Value::Null => "null",
                    _ => "any",
                }.to_string();
                let short_value = match value {
                    Value::Integer(n) => n.to_string(),
                    Value::Float(f) => format!("{:.4}", f),
                    Value::String(s) => {
                        if s.len() > 20 { format!("\"{}...\"", &s[..17]) }
                        else { format!("\"{}\"", s) }
                    }
                    Value::Bool(b) => b.to_string(),
                    Value::List(l) => format!("[..{}]", l.len()),
                    Value::Dict(d) => format!("{{..{}}}", d.len()),
                    Value::Function(params, _, _) => format!("fn({})", params.len()),
                    Value::Lambda(params, _) => format!("lambda({})", params.len()),
                    Value::Null => "null".to_string(),
                    _ => "...".to_string(),
                };
                VarInfo { name: name.clone(), type_name, short_value }
            })
            .collect()
    }

    pub fn history_up(&mut self) {
        if self.history.is_empty() { return; }
        let pos = match self.history_pos {
            Some(p) if p > 0 => p - 1,
            Some(p) => p,
            None => self.history.len() - 1,
        };
        self.history_pos = Some(pos);
        self.input = self.history[pos].clone();
        self.cursor_col = self.input.len();
    }

    pub fn history_down(&mut self) {
        match self.history_pos {
            Some(p) if p < self.history.len() - 1 => {
                self.history_pos = Some(p + 1);
                self.input = self.history[p + 1].clone();
                self.cursor_col = self.input.len();
            }
            _ => {
                self.history_pos = None;
                self.input.clear();
                self.cursor_col = 0;
            }
        }
    }
}
