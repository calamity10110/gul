#![allow(dead_code)]
// Block organizer - splits code into package blocks

use crate::ast::{Program, Statement};
use std::fs;
use std::path::Path;

pub struct BlockOrganizer {
    imports: Vec<Statement>,
    definitions: Vec<Statement>,
    functions: Vec<Statement>,
    async_functions: Vec<Statement>,
    custom_blocks: Vec<Statement>,
    main: Option<Statement>,
}

impl Default for BlockOrganizer {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockOrganizer {
    pub fn new() -> Self {
        BlockOrganizer {
            imports: Vec::new(),
            definitions: Vec::new(),
            functions: Vec::new(),
            async_functions: Vec::new(),
            custom_blocks: Vec::new(),
            main: None,
        }
    }

    pub fn organize(&mut self, program: &Program) {
        for statement in &program.statements {
            match statement {
                Statement::Import(_) => self.imports.push(statement.clone()),
                Statement::Definition { .. } => self.definitions.push(statement.clone()),
                Statement::Function { is_async, .. } => {
                    if *is_async {
                        self.async_functions.push(statement.clone());
                    } else {
                        self.functions.push(statement.clone());
                    }
                }
                Statement::CustomBlock { .. } => self.custom_blocks.push(statement.clone()),
                Statement::Main { .. } => self.main = Some(statement.clone()),
                _ => {}
            }
        }
    }

    pub fn write_blocks(&self, output_dir: &str) -> Result<(), String> {
        let output_path = Path::new(output_dir);

        // Create output directory if it doesn't exist
        fs::create_dir_all(output_path).map_err(|e| e.to_string())?;

        // Write imports.imp
        if !self.imports.is_empty() {
            let content = self.format_statements(&self.imports);
            fs::write(output_path.join("imports.imp"), content).map_err(|e| e.to_string())?;
        }

        // Write definitions.def
        if !self.definitions.is_empty() {
            let content = self.format_statements(&self.definitions);
            fs::write(output_path.join("definitions.def"), content).map_err(|e| e.to_string())?;
        }

        // Write async.asy
        if !self.async_functions.is_empty() {
            let content = self.format_statements(&self.async_functions);
            fs::write(output_path.join("async.asy"), content).map_err(|e| e.to_string())?;
        }

        // Write functions.fnc
        if !self.functions.is_empty() {
            let content = self.format_statements(&self.functions);
            fs::write(output_path.join("functions.fnc"), content).map_err(|e| e.to_string())?;
        }

        // Write custom.cs
        if !self.custom_blocks.is_empty() {
            let content = self.format_statements(&self.custom_blocks);
            fs::write(output_path.join("custom.cs"), content).map_err(|e| e.to_string())?;
        }

        // Write main.mn
        if let Some(main_stmt) = &self.main {
            // Format main statement properly
            let content = self.format_statements(std::slice::from_ref(main_stmt));
            fs::write(output_path.join("main.mn"), content).map_err(|e| e.to_string())?;
        }

        // Write package.toml
        let package_toml = self.generate_package_toml();
        fs::write(output_path.join("package.toml"), package_toml).map_err(|e| e.to_string())?;

        Ok(())
    }

    fn format_statements(&self, statements: &[Statement]) -> String {
        statements
            .iter()
            .map(|s| format!("{:?}\n", s))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn generate_package_toml(&self) -> String {
        r#"[package]
name = "my-package"
version = "0.1.0"
authors = []
description = "Auto-generated package"

[blocks]
imports = "imports.imp"
definitions = "definitions.def"
async = "async.asy"
functions = "functions.fnc"
custom = "custom.cs"
main = "main.mn"
"#
        .to_string()
    }
}

// Public API function
pub fn organize_program(program: &Program) -> Result<(), String> {
    let mut organizer = BlockOrganizer::new();
    organizer.organize(program);
    organizer.write_blocks("./output")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Expression;

    #[test]
    fn test_organize_blocks() {
        let mut organizer = BlockOrganizer::new();

        let program = Program {
            statements: vec![
                Statement::Import("std.io".to_string()),
                Statement::Definition {
                    name: "x".to_string(),
                    value: Expression::Integer(10),
                },
            ],
        };

        organizer.organize(&program);

        assert_eq!(organizer.imports.len(), 1);
        assert_eq!(organizer.definitions.len(), 1);
    }
}
