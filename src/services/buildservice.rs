use std::{collections::HashMap, fs};

use crate::{structs::routesettings::RouteSettings, utils::{files::find_files_by_regex_inverse, regex::replace_path}};

pub struct BuildService;

impl BuildService {

    pub fn get_route_settings(file_folder: &std::path::PathBuf, file_extension: &str, route_settings: &HashMap<&str, RouteSettings>) -> RouteSettings {
        // Gets the route settings provided by the user
        let route_settings_path = file_folder.join("settings.json");
        let new_route_settings = if route_settings_path.exists() {
            let settings_content = fs::read_to_string(&route_settings_path).unwrap_or_else(|_| "{}".to_string());
            serde_json::from_str::<RouteSettings>(&settings_content).unwrap_or(RouteSettings::default())
        } else {
            RouteSettings::default()
        };

        // Merges the default route settings with the user-provided settings
        let route_settings = if route_settings.contains_key(file_extension) {
            route_settings.get(file_extension).unwrap().merge(&new_route_settings)
        } else {
            new_route_settings
        };

        return route_settings;
    }

    pub fn run_build_command(route_settings: &RouteSettings, source_file_folder: &std::path::PathBuf, output_file_folder: &std::path::PathBuf, source_file: &std::path::PathBuf) {
        if !route_settings.build_command.is_empty() {
            let mut final_args: Vec<String> = Vec::new();

            
            if !route_settings.build_args.is_empty() {
                for arg in &route_settings.build_args {
                    let mut formatted_arg = arg.to_string();
                    formatted_arg = formatted_arg.replace("<<output_folder>>", &output_file_folder.to_str().unwrap());
                    formatted_arg = formatted_arg.replace("<<source_file>>", &source_file.to_str().unwrap());
                    final_args.push(formatted_arg);
                }
            }

            let final_build_command = route_settings.build_command.clone();
            println!("\nRunning build command: {} {:?}\n", final_build_command, final_args);
            let status = std::process::Command::new(final_build_command)
                .current_dir(source_file_folder)
                .args(&final_args)
                .status()
                .expect("Failed to execute build command");

            if !status.success() {
                println!("Build command failed with status: {:?}", status);
            }
        }
    }

    pub fn copy_other_files(route_settings: &RouteSettings, file_folder: &std::path::PathBuf, router_output_folder_path: &std::path::PathBuf) {
        // Copies any other files in the folder that aren't the main file to the output folder
        let other_files = find_files_by_regex_inverse(&file_folder, r"^main\.");
        for other in other_files{
            let other_file_name = other.file_name().unwrap();
            let file_is_ignored = route_settings.ignore_files.contains(&other_file_name.to_str().unwrap().to_string());
            let file_is_settings = other.file_name().unwrap().to_str().unwrap() == "settings.json";
            if file_is_ignored || file_is_settings {
                continue;
            }

            

            let settings_file_path = if route_settings.root_folder.is_empty() { "<<output_folder>>" } else { &route_settings.root_folder };
            let other_file_path_str = replace_path(settings_file_path, &HashMap::from([
                ("<<output_folder>>", router_output_folder_path.to_str().unwrap()),
            ]));
            let other_file_path = std::path::PathBuf::from(other_file_path_str);
            let absolute_output_file_path = std::path::absolute(other_file_path.join(other_file_name)).unwrap();

            match fs::copy(&other, &absolute_output_file_path) {
                Ok(_) => continue,
                Err(e) => println!("Failed to copy extra file: {}", e),
            }
        }
    }
}