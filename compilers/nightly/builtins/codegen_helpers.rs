// GUL v3.2 Compiler - Codegen Helpers
// Builtin functions for code transformation and generation

use std::collections::{HashMap, HashSet};

// ================================================================================
// STRING UTILITIES
// ================================================================================

pub fn escape_string(s: String)  ->  String {
    // Properly escape string literals for Rust output// 
    let mut result = String::new();
    for ch in s.chars() {
        match ch {
            '\n' => result.push_str("\\n"),
            '\t' => result.push_str("\\t"),
            '\r' => result.push_str("\\r"),
            '\\' => result.push_str("\\\\"),
            '"' => result.push_str("\\\""),
            _ => result.push(ch),
        }
    }
    return result;

}
pub fn quote_string(s: String)  ->  String {
    // Wrap string in quotes with proper escaping// 
    return "\"".to_string() + &escape_string(s) + "\"";

// ================================================================================
// TYPE CONVERSION
// ================================================================================

}
pub fn gul_type_to_rust(gul_type: String)  ->  String {
    // Convert GUL type annotation to Rust type// 
    if gul_type == "usize".to_string() {
        return "i64".to_string();
    }
    else if gul_type == "f64".to_string() || gul_type == "float".to_string() {
        return "f64".to_string();
    }
    else if gul_type == "str".to_string() || gul_type == "String".to_string() {
        return "String".to_string();
    }
    else if gul_type == "bool".to_string() {
        return "bool".to_string();
    }
    else if gul_type == "Vec".to_string() {
        return "Vec".to_string();
    }
    else if gul_type == "HashMap".to_string() {
        return "HashMap".to_string();
    }
    else if gul_type == "HashSet".to_string() {
        return "HashSet".to_string();
    }
    else {
        return gul_type;

    }
}
pub fn gul_generic_to_rust(type_str: String)  ->  String {
    // Convert Vec<T> to Vec<T>// 
    if type_str.starts_with("Vec[") {
        let inner = &type_str[4..type_str.len() - 1];
        return "Vec<".to_string() + &gul_type_to_rust(inner.to_string()) + ">";
    }
    else if type_str.starts_with("HashMap[") {
        let inner = &type_str[8..type_str.len() - 1]; // Simply copying inner content for now
        return "HashMap<".to_string() + inner + ">";
    }
    else {
        return gul_type_to_rust(type_str);


// ================================================================================
// SYNTAX TRANSFORMATIONS
// ================================================================================

    }
}
pub fn self_param_to_rust(param: String)  ->  String {
    // Convert GUL self params to Rust style// 
    if param == "self".to_string() {
        return "&self".to_string();
    }
    else if param == "ref self".to_string() {
        return "&mut self".to_string();
    }
    else if param == "own self".to_string() {
        return "self".to_string();
    }
    else {
        return param;

    }
}
pub fn struct_method_signature(name: String, params: Vec<String>, return_type: String, receiver: String)  ->  String {
    // Generate Rust method signature// 
    let mut result = "fn ".to_string() + &name + "(";

    if receiver != "" {
        result = result + &self_param_to_rust(receiver);
        if params.len() > 0 {
            result = result + ", ";

        }
    }
    let mut first = true;
    for p in params {
        if ! first {
            result = result + ", ";
        }
        result = result + &p;
        first = false;

    }
    result = result + ")";

    if return_type != "" {
        result = result + " -> " + &gul_type_to_rust(return_type);

    }
    return result;
}
// ================================================================================
// CODE FORMATTERS
// ================================================================================
pub fn indent_code(code: String, level: usize)  ->  String {
    // Add indentation to code block// 
    let indent = "    ".repeat(level);

    let mut result = "".to_string();
    let lines = code.split('\n');
    for line in lines {
        if !line.is_empty() {
            result = result + &indent + line + "\n";
        }
        else {
            result = result + "\n";

        }
    }
    return result;

}
pub fn wrap_in_braces(code: String)  ->  String {
    // Wrap code block in braces// 
    return "{\n".to_string() + &code + "}\n";
}