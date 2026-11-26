use std::{collections::HashMap, path::{Path, PathBuf}};

use crate::{structs::routesettings::RouteSettings, utils::{files::{find_file_by_regex, get_folder_by_regex}, regex::replace_path}};

pub struct PathService;

impl PathService {
    pub fn get_process_path(process_dir: &PathBuf, route_settings: &RouteSettings) -> Option<PathBuf> {
        let pattern = if route_settings.process_path.is_empty() { 
            "main*".to_string() 
        } else { 
            replace_path(&route_settings.process_path, &HashMap::from([("<<folder>>", process_dir.to_str().unwrap())]))  
        };
        find_file_by_regex(Path::new(&process_dir), &pattern)
    }

    pub fn get_supplied_values_from_path(original_path: &PathBuf, actual_path: &PathBuf) -> HashMap<String, String> {
        let mut supplied_values = HashMap::new();
        let original_segments: Vec<&str> = original_path.components()
            .map(|comp| comp.as_os_str().to_str().unwrap_or(""))
            .rev()
            .collect();
        let actual_segments: Vec<&str> = actual_path.components()
            .map(|comp| comp.as_os_str().to_str().unwrap_or(""))
            .rev()
            .collect();


        for (original_segment, actual_segment) in original_segments.iter().zip(actual_segments.iter()) {
            if actual_segment.starts_with("[") && actual_segment.ends_with("]") {
                let key = actual_segment.trim_start_matches("[").trim_end_matches("]").to_string();
                supplied_values.insert(key, original_segment.to_string());
            }
        }

        supplied_values
    }

    
    pub fn get_folder_path(web_path: &str, root_path: &PathBuf) -> Option<PathBuf> {
        let mut accumulated_path = root_path.to_str().unwrap_or("").to_string();
        for segment in web_path.split("/").filter(|s| !s.is_empty()) {
            let accumulated_as_path = Path::new(&accumulated_path);
            let regular_path = get_folder_by_regex(accumulated_as_path, segment);
            match regular_path {
                Some(path) => {
                    accumulated_path = path.to_str().unwrap_or("").to_string();
                },
                None => {
                    let id_path = get_folder_by_regex(accumulated_as_path, r"\[.*\]");
                    match id_path {
                        Some(path) => {
                            accumulated_path = path.to_str().unwrap_or("").to_string();
                        },
                        None => {
                            return None;
                        }
                    }
                }
            }
        }
        return Some(PathBuf::from(accumulated_path));
    }
}