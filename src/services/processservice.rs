use std::{collections::HashMap, fs, path::{PathBuf}};

use crate::{services::pathservice::PathService, structs::{asyncprocess::AsyncProcess, routesettings::RouteSettings}, utils::regex::replace_path};

pub struct ProcessService;

impl ProcessService {

    pub fn get_route_settings(path: &str, root_path: &PathBuf) -> RouteSettings {
        let root_output_path = std::path::absolute(root_path).unwrap(); 
        let web_path = std::path::Path::new(&path).strip_prefix("/").unwrap_or(std::path::Path::new(&path));
        let process_dir = PathService::get_folder_path(web_path.to_str().unwrap(), &root_output_path).unwrap_or(root_output_path.join(&web_path));
        let route_settings_path = process_dir.join("settings.json");
        let settings_content = fs::read_to_string(&route_settings_path).unwrap_or_else(|_| "{}".to_string());
        serde_json::from_str::<RouteSettings>(&settings_content).unwrap_or(RouteSettings::default())
    }

    pub async fn spawn_route_process<'a>(path: &str, process_path: &PathBuf, root_path: &PathBuf, process_dir: &PathBuf, processes: &'a mut tokio::sync::MutexGuard<'_, HashMap<String, AsyncProcess>>) -> Option<&'a mut AsyncProcess> {
        if let std::collections::hash_map::Entry::Vacant(entry) = processes.entry(path.to_string()) {
            // Get route settings
            let route_settings = ProcessService::get_route_settings(path, root_path);

            let mut final_args: Vec<String> = Vec::new();

            if !route_settings.run_args.is_empty() {
                for arg in &route_settings.run_args {
                    let mut formatted_arg = arg.to_string();
                    formatted_arg = replace_path(&formatted_arg, &HashMap::from([
                        ("<<file>>", process_path.to_str().unwrap()),
                        ("<<folder>>", process_dir.to_str().unwrap()),
                    ]));
                    final_args.push(formatted_arg);
                }

            }

            let mut final_route_command = route_settings.run_command.clone();
            
            final_route_command = replace_path(&final_route_command, &HashMap::from([
                ("<<file>>", process_path.to_str().unwrap()),
                ("<<folder>>", process_dir.to_str().unwrap()),
            ]));

            let final_args_pointers: Vec<&str> = final_args.iter().map(|s| s.as_str()).collect();
            let new_process = AsyncProcess::new(&final_route_command, &final_args_pointers).unwrap();
            entry.insert(new_process);
        }
        return Some(processes.get_mut(path).unwrap());
    }

}