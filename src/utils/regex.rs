use std::collections::HashMap;

pub fn replace_path(path: &str, replacements: &HashMap<&str, &str>) -> String {
    let mut result = path.to_string();
    for (key, value) in replacements {
        result = result.replace(key, value);
    }
    result
}