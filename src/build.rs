use std::{collections::HashMap, path::{Path, PathBuf}};
use std::fs;
use crate::{services::buildservice::BuildService, utils::files::{copy_directory_recursive, find_files_by_regex_recursive}};
use crate::structs::routesettings::RouteSettings;




pub fn build_code(root_path: PathBuf) {
    let route_settings: HashMap<&str, RouteSettings> = HashMap::from([
        ("py", RouteSettings{
            build_command: "".to_string(),
            build_output_path: "main.py".to_string(),
            run_command: "py".to_string(),
            build_args: vec![],
            run_args: vec!["-u".to_string(), "<<file>>".to_string()],
            payload: None,
            ignore: false,
            root_folder: "".to_string(),
            ignore_files: vec![],
            process_path: "".to_string(),
        }),
        ("js", RouteSettings{
            build_command: "".to_string(),
            build_output_path: "main.js".to_string(),
            run_command: "node".to_string(),
            build_args: vec![],
            run_args: vec!["<<file>>".to_string()],
            payload: None,
            ignore: false,
            root_folder: "".to_string(),
            ignore_files: vec![],
            process_path: "".to_string(),
        }),
        ("java", RouteSettings{
            build_command: "javac".to_string(),
            build_output_path: "".to_string(),
            run_command: "java".to_string(),
            build_args: vec!["-d".to_string(), "<<output_folder>>".to_string(), "<<source_file>>".to_string()],
            run_args: vec!["-cp".to_string(), "<<folder>>".to_string(), "main".to_string()],
            payload: None,
            ignore: false,
            root_folder: "".to_string(),
            ignore_files: vec![],
            process_path: "".to_string(),
        }),
    ]);


    let output_path = root_path.join("output");
    let source_path = root_path.join("src");
    
    if output_path.exists() {
        println!("Deleting output folder: {:?}", std::path::absolute(&output_path).unwrap());
        let _ = fs::remove_dir_all(&output_path);
    }

    let files = find_files_by_regex_recursive(&source_path, r"^main\.");

    for file in files {
        println!("Found file: {:?}", &file);

        // The relative path of the file to build
        let relative_path = file.strip_prefix(&source_path).unwrap();
        let file_extension = relative_path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

        // The folder the file is in
        let file_folder = file.parent().unwrap_or_else(|| Path::new(""));

        let route_settings = BuildService::get_route_settings(&file_folder.to_path_buf(), file_extension, &route_settings);

        if route_settings.ignore {
            println!("Ignoring file: {:?} due to ignore flag in settings.", &file);
            continue;
        }

        // Skips if no route settings are found for the file
        if route_settings == RouteSettings::default() || route_settings.run_command.is_empty() {
            println!("No route settings found for file: {:?}, skipping build.", &file);
            continue;
        }

        // Gets the file path of the build output file
        let build_output_file_name = &route_settings.build_output_path;
        let build_output_file_path = std::path::absolute(file_folder.join(&build_output_file_name)).unwrap();

        // Gets the folder the build output will go to and the file path of the build output file in the output folder
        let router_output_folder_path = std::path::absolute(output_path.join(relative_path).parent().unwrap()).unwrap();
        let router_output_file_path = std::path::absolute(router_output_folder_path.join(&build_output_file_name)).unwrap();

        // Creates the required output folder if it doesn't exist
        let _ = fs::create_dir_all(&router_output_folder_path);

        // Builds the file using the specified build command
        let absolute_file_path = std::path::absolute(&file).unwrap();
        BuildService::run_build_command(&route_settings, &PathBuf::from(&file_folder), &router_output_folder_path.to_path_buf(), &absolute_file_path);
        
        if !build_output_file_name.is_empty() {
            // Copies the build output file to the output folder
            match fs::copy(&build_output_file_path, &router_output_file_path) {
                Ok(_) => println!("Successfully copied build output to {:?}", &router_output_file_path),
                Err(e) => eprintln!("Failed to copy build output: {}", e),
            }
        }

        BuildService::copy_other_files(&route_settings, &file_folder.to_path_buf(), &router_output_folder_path);

        // Writes the route settings to the output folder
        let json_string = serde_json::to_string_pretty(&route_settings);
        fs::write(router_output_folder_path.join("settings.json"), json_string.unwrap()).expect("Failed to write settings.json");

        println!("\n--------------------\n");
        // A list of commands to grab the build data and copy it into the output folder and a command to run the output file
    }

    let src_public = root_path.join("public");
    let output_public = output_path.join("__public__");
    copy_directory_recursive(&src_public, &output_public).expect("Failed to copy public folder");
}