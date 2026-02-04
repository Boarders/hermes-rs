use serde_json::to_string_pretty;
use serde_json::{Map, Result, Value};

/// Pretty print JSON
pub fn pp_json(json: &Value) -> Result<String> {
    to_string_pretty(&json)
}

/// Truncate JSON to show only top-level structure
/// depth: how many levels deep to preserve
pub fn truncate_json(value: &Value, depth: usize) -> Value {
    if depth == 0 {
        match value {
            Value::Null => Value::String("null".to_string()),
            Value::Bool(_) => Value::String("bool".to_string()),
            Value::Number(_) => Value::String("number".to_string()),
            Value::String(_) => Value::String("\"...\"".to_string()),
            Value::Array(_) => Value::String("[...]".to_string()),
            Value::Object(_) => Value::String("{...}".to_string()),
        }
    } else {
        match value {
            Value::Object(obj) => {
                let mut map = Map::new();
                for (key, val) in obj {
                    map.insert(key.clone(), truncate_json(val, depth - 1));
                }
                Value::Object(map)
            }
            Value::Array(arr) => {
                if arr.is_empty() {
                    Value::Array(vec![])
                } else {
                    // Show first element truncated and indicate more
                    let first = truncate_json(&arr[0], depth - 1);
                    if arr.len() > 1 {
                        Value::Array(vec![
                            first,
                            Value::String(format!("... ({} more)", arr.len() - 1)),
                        ])
                    } else {
                        Value::Array(vec![first])
                    }
                }
            }
            _ => value.clone(),
        }
    }
}
