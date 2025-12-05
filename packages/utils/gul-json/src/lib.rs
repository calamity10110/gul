// GUL JSON - JSON parsing and serialization

use std::collections::HashMap;

/// JSON Value
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Object(o) => Some(o),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }
}

/// Parse JSON string
pub fn parse(input: &str) -> Result<Value, String> {
    serde_json::from_str(input).map_err(|e| e.to_string())
}

/// Serialize to JSON string
pub fn stringify(value: &Value) -> Result<String, String> {
    serde_json::to_string(value).map_err(|e| e.to_string())
}

/// Serialize to pretty JSON string
pub fn stringify_pretty(value: &Value) -> Result<String, String> {
    serde_json::to_string_pretty(value).map_err(|e| e.to_string())
}

// Implement serde traits for Value
impl serde::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Null => serializer.serialize_none(),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::Number(n) => serializer.serialize_f64(*n),
            Value::String(s) => serializer.serialize_str(s),
            Value::Array(a) => a.serialize(serializer),
            Value::Object(o) => o.serialize(serializer),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v: serde_json::Value = serde::Deserialize::deserialize(deserializer)?;
        Ok(convert_from_serde(v))
    }
}

fn convert_from_serde(v: serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Bool(b),
        serde_json::Value::Number(n) => Value::Number(n.as_f64().unwrap_or(0.0)),
        serde_json::Value::String(s) => Value::String(s),
        serde_json::Value::Array(a) => {
            Value::Array(a.into_iter().map(convert_from_serde).collect())
        }
        serde_json::Value::Object(o) => Value::Object(
            o.into_iter()
                .map(|(k, v)| (k, convert_from_serde(v)))
                .collect(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_object() {
        let json = r#"{"name": "John", "age": 30}"#;
        let value = parse(json).unwrap();

        let obj = value.as_object().unwrap();
        assert_eq!(obj.get("name").unwrap().as_str(), Some("John"));
        assert_eq!(obj.get("age").unwrap().as_number(), Some(30.0));
    }

    #[test]
    fn test_parse_array() {
        let json = r#"[1, 2, 3]"#;
        let value = parse(json).unwrap();

        let arr = value.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0].as_number(), Some(1.0));
    }

    #[test]
    fn test_stringify() {
        let mut obj = HashMap::new();
        obj.insert("name".to_string(), Value::String("John".to_string()));

        let value = Value::Object(obj);
        let json = stringify(&value).unwrap();

        assert!(json.contains("name"));
        assert!(json.contains("John"));
    }
}
