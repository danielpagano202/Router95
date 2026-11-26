use std::collections::HashMap;

use serde_json::Value;

pub fn walk_json(
    value: &Value,
    previous: Vec<String>,
    result: &mut HashMap<Vec<String>, Value>,
) {
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                let mut current = previous.clone();
                current.push(key.clone());

                walk_json(value, current, result);
            }
        }

        Value::Array(array) => {
            for (index, value) in array.iter().enumerate() {
                let mut current = previous.clone();
                current.push(index.to_string());

                walk_json(value, current, result);
            }
        }

        // We've reached an actual value
        Value::String(_) |
        Value::Number(_) |
        Value::Bool(_) |
        Value::Null => {
            result.insert(previous, value.clone());
        }
    }
}

pub fn match_json(v_type: &str, value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(_) => v_type.contains("string"),
        serde_json::Value::Number(_) => v_type.contains("number"),
        serde_json::Value::Bool(_) => v_type.contains("bool"),
        serde_json::Value::Null => v_type.contains("?") || v_type == "null",
        _ => false,
    }
}