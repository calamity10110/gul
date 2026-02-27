use crate::frontend::ast::*;
use super::values::{Value, ControlFlow};
use super::Interpreter;
use std::collections::HashMap;

impl Interpreter {
    pub(crate) fn evaluate(&mut self, expr: &Expression) -> Result<Value, String> {
        match expr {
            Expression::Integer(i) => Ok(Value::Integer(*i)),
            Expression::Float(f) => Ok(Value::Float(*f)),
            Expression::String(s) => Ok(Value::String(s.clone())),
            Expression::Bool(b) => Ok(Value::Bool(*b)),
            Expression::Identifier(name) => self
                .variables
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Undefined variable: {}", name)),
            Expression::Call { function, args } => {
                // Special handling for grad(f, x)
                if let Expression::Identifier(name) = function.as_ref() {
                    if name == "grad" {
                        if args.len() != 2 {
                            return Err("grad expects 2 arguments (function, value)".to_string());
                        }
                        let f_val = self.evaluate(&args[0])?;
                        let x_val = self.evaluate(&args[1])?;

                        let x_dual = match x_val {
                            Value::Integer(i) => Value::Dual(i as f64, 1.0),
                            Value::Float(f) => Value::Dual(f, 1.0),
                            _ => return Err("grad expects numeric value".to_string()),
                        };

                        match f_val {
                            Value::Lambda(params, body_expr) => {
                                if params.len() != 1 { return Err("grad expects function with 1 parameter".to_string()); }
                                let param_name = &params[0].0;
                                let old_val = self.variables.insert(param_name.clone(), x_dual);
                                let result = self.evaluate(&body_expr)?;
                                if let Some(v) = old_val { self.variables.insert(param_name.clone(), v); }
                                else { self.variables.remove(param_name); }

                                match result {
                                    Value::Dual(_, grad) => return Ok(Value::Float(grad)),
                                    Value::Integer(_) | Value::Float(_) => return Ok(Value::Float(0.0)),
                                    _ => return Err("Function did not return numeric value".to_string()),
                                }
                            }
                             _ => return Err("grad currently supports Arrow Functions only".to_string()),
                        }
                    }
                }

                let func_val = self.evaluate(function)?;
                let mut arg_vals = Vec::new();
                for arg in args {
                    arg_vals.push(self.evaluate(arg)?);
                }

                match func_val {
                    Value::NativeFunction(f) => Ok(f(arg_vals)),
                    Value::Function(params, outputs, body) => {
                         let mut local_interpreter = self.clone();
                         // Bind inputs
                         for (i, (param_name, _)) in params.iter().enumerate() {
                             if i < arg_vals.len() {
                                 local_interpreter.variables.insert(param_name.clone(), arg_vals[i].clone());
                             }
                         }
                         // Bind outputs as Null initially
                         for (out_name, _) in &outputs {
                             if !out_name.is_empty() {
                                 local_interpreter.variables.insert(out_name.clone(), Value::Null);
                             }
                         }

                         match local_interpreter.execute_statements(&body)? {
                             ControlFlow::Return(v) => Ok(v),
                             _ => {
                                 // Return outputs if no explicit return
                                 if outputs.is_empty() {
                                     Ok(Value::Null)
                                 } else if outputs.len() == 1 {
                                     let (out_name, _) = &outputs[0];
                                     if out_name.is_empty() {
                                          Ok(Value::Null)
                                     } else {
                                          Ok(local_interpreter.variables.get(out_name).cloned().unwrap_or(Value::Null))
                                     }
                                 } else {
                                     let mut res_map = HashMap::new();
                                     for (out_name, _) in &outputs {
                                         if !out_name.is_empty() {
                                             res_map.insert(out_name.clone(), local_interpreter.variables.get(out_name).cloned().unwrap_or(Value::Null));
                                         }
                                     }
                                     Ok(Value::Dict(res_map))
                                 }
                             }
                         }
                    }
                    Value::Lambda(params, body_expr) => {
                         // Execute expression body
                         for (i, (param, _ty)) in params.iter().enumerate() {
                             if i < arg_vals.len() {
                                 self.variables.insert(param.clone(), arg_vals[i].clone());
                             }
                         }
                         self.evaluate(&body_expr)
                    }
                    _ => Err(format!("Not a callable: {:?}", func_val)),
                }
            }
            Expression::Binary { left, op, right } => {
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;

                match (l, op, r) {
                    (Value::String(a), BinaryOp::Add, Value::String(b)) => {
                        Ok(Value::String(a + &b))
                    }
                    (Value::Integer(a), BinaryOp::Add, Value::Integer(b)) => {
                        Ok(Value::Integer(a + b))
                    }
                    (Value::Integer(a), BinaryOp::Subtract, Value::Integer(b)) => {
                        Ok(Value::Integer(a - b))
                    }
                    (Value::Integer(a), BinaryOp::Multiply, Value::Integer(b)) => {
                        Ok(Value::Integer(a * b))
                    }
                    (Value::Integer(a), BinaryOp::Divide, Value::Integer(b)) => {
                        if b == 0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Integer(a / b))
                        }
                    }
                    (Value::Float(a), BinaryOp::Add, Value::Float(b)) => {
                        Ok(Value::Float(a + b))
                    }
                    (Value::Float(a), BinaryOp::Subtract, Value::Float(b)) => {
                        Ok(Value::Float(a - b))
                    }
                    (Value::Float(a), BinaryOp::Multiply, Value::Float(b)) => {
                        Ok(Value::Float(a * b))
                    }
                    (Value::Float(a), BinaryOp::Divide, Value::Float(b)) => {
                        Ok(Value::Float(a / b))
                    }
                    // Auto-differentiation (Dual numbers)
                    (Value::Dual(v1, d1), BinaryOp::Add, Value::Dual(v2, d2)) => {
                        Ok(Value::Dual(v1 + v2, d1 + d2))
                    }
                    (Value::Dual(v1, d1), BinaryOp::Subtract, Value::Dual(v2, d2)) => {
                        Ok(Value::Dual(v1 - v2, d1 - d2))
                    }
                    (Value::Dual(v1, d1), BinaryOp::Multiply, Value::Dual(v2, d2)) => {
                        // Product rule: d(uv) = udv + vdu
                        Ok(Value::Dual(v1 * v2, v1 * d2 + v2 * d1))
                    }
                    (Value::Dual(v1, d1), BinaryOp::Divide, Value::Dual(v2, d2)) => {
                        // Quotient rule: d(u/v) = (vdu - udv) / v^2
                        Ok(Value::Dual(v1 / v2, (v2 * d1 - v1 * d2) / (v2 * v2)))
                    }
                    (Value::Dual(v1, d1), BinaryOp::Add, Value::Integer(n)) => {
                         Ok(Value::Dual(v1 + n as f64, d1))
                    }
                    (Value::Integer(n), BinaryOp::Add, Value::Dual(v2, d2)) => {
                         Ok(Value::Dual(n as f64 + v2, d2))
                    }
                    (Value::Dual(v1, d1), BinaryOp::Multiply, Value::Integer(n)) => {
                         Ok(Value::Dual(v1 * n as f64, d1 * n as f64))
                    }
                    (Value::Integer(n), BinaryOp::Multiply, Value::Dual(v2, d2)) => {
                         Ok(Value::Dual(n as f64 * v2, n as f64 * d2))
                    }
                     // Add more combinations (Float, etc.) as needed for basic support

                    (l, BinaryOp::Equal, r) => Ok(Value::Bool(l == r)),
                    (l, BinaryOp::NotEqual, r) => Ok(Value::Bool(l != r)),
                    (Value::Integer(a), BinaryOp::Less, Value::Integer(b)) => Ok(Value::Bool(a < b)),
                    (Value::Integer(a), BinaryOp::Greater, Value::Integer(b)) => {
                        Ok(Value::Bool(a > b))
                    }
                    (Value::Integer(a), BinaryOp::LessEqual, Value::Integer(b)) => {
                        Ok(Value::Bool(a <= b))
                    }
                    (Value::Integer(a), BinaryOp::GreaterEqual, Value::Integer(b)) => {
                        Ok(Value::Bool(a >= b))
                    }
                    (Value::Float(a), BinaryOp::Less, Value::Float(b)) => Ok(Value::Bool(a < b)),
                    (Value::Float(a), BinaryOp::Greater, Value::Float(b)) => Ok(Value::Bool(a > b)),
                    // Add more binary ops support...
                    _ => Ok(Value::Null),
                }
            }
            Expression::List(elements) => {
                let mut vals = Vec::new();
                for e in elements {
                    vals.push(self.evaluate(e)?);
                }
                Ok(Value::List(vals))
            }
            Expression::Dict(pairs) => {
                let mut dict = HashMap::new();
                for (k, v) in pairs {
                    let val = self.evaluate(v)?;
                    dict.insert(k.clone(), val);
                }
                Ok(Value::Dict(dict))
            }
            Expression::Member { object, member } => {
                let obj = self.evaluate(object)?;
                match obj {
                    Value::Object(_, fields) => fields
                        .get(member)
                        .cloned()
                        .ok_or_else(|| format!("Member not found: {}", member)),
                    Value::Dict(fields) => fields
                        .get(member)
                        .cloned()
                        .ok_or_else(|| format!("Member not found: {}", member)),
                    Value::Function(params, outputs, _) => {
                         // v3.2 Function Node properties
                         if member.starts_with("input_") {
                             let idx: usize = member[6..].parse().unwrap_or(0);
                             if idx > 0 && idx <= params.len() {
                                 let (name, _ty) = &params[idx-1];
                                 let mut map = HashMap::new();
                                 map.insert(name.clone(), Value::String(name.clone())); // Simplified proxy
                                 return Ok(Value::Dict(map));
                             }
                         } else if member.starts_with("output_") {
                             let idx: usize = member[7..].parse().unwrap_or(0);
                             if idx > 0 && idx <= outputs.len() {
                                 let (name, _ty) = &outputs[idx-1];
                                 return Ok(Value::String(name.clone()));
                             }
                         }
                         Err(format!("Function property not found: {}", member))
                    }
                    _ => Err(format!("Cannot access member '{}' on non-object", member)),
                }
            }
            Expression::Ownership { value, .. } => self.evaluate(value),
            Expression::Await(expr) => self.evaluate(expr),
            Expression::ListOp { op, args } => {
                match op {
                    ListOp::Car => {
                        if args.is_empty() {
                            return Err("car expects 1 argument".to_string());
                        }
                        let val = self.evaluate(&args[0])?;
                        if let Value::List(list) = val {
                            if list.is_empty() {
                                Ok(Value::Null) // or error?
                            } else {
                                Ok(list[0].clone())
                            }
                        } else {
                            Err("car expects a list".to_string())
                        }
                    }
                    ListOp::Cdr => {
                        if args.is_empty() {
                            return Err("cdr expects 1 argument".to_string());
                        }
                        let val = self.evaluate(&args[0])?;
                        if let Value::List(list) = val {
                            if list.is_empty() {
                                Ok(Value::List(vec![]))
                            } else {
                                Ok(Value::List(list[1..].to_vec()))
                            }
                        } else {
                            Err("cdr expects a list".to_string())
                        }
                    }
                    ListOp::Cons => {
                        if args.len() != 2 {
                            return Err("cons expects 2 arguments".to_string());
                        }
                        let head = self.evaluate(&args[0])?;
                        let tail = self.evaluate(&args[1])?;
                        if let Value::List(mut list) = tail {
                            list.insert(0, head);
                            Ok(Value::List(list))
                        } else {
                            Err("cons expects second argument to be a list".to_string())
                        }
                    }
                    ListOp::Map => {
                        // map(fn, list) - apply function to each element
                        if args.len() != 2 {
                            return Err("map expects 2 arguments (function, list)".to_string());
                        }
                        let func = self.evaluate(&args[0])?;
                        let list_val = self.evaluate(&args[1])?;
                        if let Value::List(items) = list_val {
                            let mut result = Vec::new();
                            for item in items {
                                // Apply function to each item
                                match &func {
                                    Value::NativeFunction(f) => {
                                        result.push(f(vec![item]));
                                    }
                                    Value::Function(params, _outputs, _body) => {
                                        // For user functions, simplified call
                                        if params.is_empty() {
                                            result.push(item);
                                        } else {
                                            // Would need proper scope handling
                                            result.push(item);
                                        }
                                    }
                                    _ => result.push(item),
                                }
                            }
                            Ok(Value::List(result))
                        } else {
                            Err("map expects second argument to be a list".to_string())
                        }
                    }
                    ListOp::Fold => {
                        // fold(fn, initial, list) - reduce list to single value
                        if args.len() != 3 {
                            return Err(
                                "fold expects 3 arguments (function, initial, list)".to_string()
                            );
                        }
                        let func = self.evaluate(&args[0])?;
                        let initial = self.evaluate(&args[1])?;
                        let list_val = self.evaluate(&args[2])?;
                        if let Value::List(items) = list_val {
                            let mut acc = initial;
                            for item in items {
                                match &func {
                                    Value::NativeFunction(f) => {
                                        acc = f(vec![acc, item]);
                                    }
                                    _ => {
                                        // For non-native functions, simplified handling
                                        acc = item;
                                    }
                                }
                            }
                            Ok(acc)
                        } else {
                            Err("fold expects third argument to be a list".to_string())
                        }
                    }
                    ListOp::Slice => {
                        // slice(list, start, end) - get sublist
                        if args.len() < 2 || args.len() > 3 {
                            return Err(
                                "slice expects 2-3 arguments (list, start, [end])".to_string()
                            );
                        }
                        let list_val = self.evaluate(&args[0])?;
                        let start_val = self.evaluate(&args[1])?;
                        let end_val = if args.len() == 3 {
                            Some(self.evaluate(&args[2])?)
                        } else {
                            None
                        };

                        if let Value::List(items) = list_val {
                            let start = match start_val {
                                Value::Integer(i) => i.max(0) as usize,
                                _ => return Err("slice start must be integer".to_string()),
                            };
                            let end = match end_val {
                                Some(Value::Integer(i)) => i.min(items.len() as i64) as usize,
                                None => items.len(),
                                _ => return Err("slice end must be integer".to_string()),
                            };
                            if start > end || start >= items.len() {
                                Ok(Value::List(vec![]))
                            } else {
                                Ok(Value::List(items[start..end.min(items.len())].to_vec()))
                            }
                        } else {
                            Err("slice expects first argument to be a list".to_string())
                        }
                    }
                }
            }
            Expression::Typed { expr, .. } => {
                // For gradual typing, just evaluate the inner expression
                // Type checking is done at compile time
                self.evaluate(expr)
            }
            Expression::Lambda { params, body } => {
                let typed_params: Vec<(String, Option<Type>)> = params.iter().map(|p| (p.clone(), None)).collect();
                Ok(Value::Lambda(typed_params, body.clone()))
            }
            _ => Ok(Value::Null),
        }
    }
}
