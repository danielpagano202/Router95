use std::{fs, io, path::{Path, PathBuf}};
use regex::Regex;
use walkdir::WalkDir;

pub fn find_file_by_regex(dir: &Path, pattern: &str) -> Option<std::path::PathBuf> {
    let re: Regex = match Regex::new(pattern) {
        // 1. Unwraps the successfully compiled Regex
        Ok(code) => code, 
        
        // 2. Handles the error and falls back to a default valid Regex
        Err(error) => {
            println!("Failed to compile regex pattern: {}, falling back to default.", error);
            Regex::new("main.*").unwrap() 
        }
    };

    if fs::read_dir(dir).is_err() {
        println!("Error opening directory: {:?}", dir);
        return None;
    }

    let entries = fs::read_dir(dir).expect("Error opening directory");
    for entry in entries.flatten() {
        if entry.file_type().unwrap().is_file() {
            let file_name = entry.file_name().to_string_lossy().into_owned();
            if re.is_match(&file_name) {
                return Some(entry.path());
            }
        }
    }

    return None;
}

pub fn find_files_by_regex_recursive(dir: &Path, pattern: &str) -> Vec<PathBuf> {
    let re = Regex::new(pattern).unwrap();
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_name = entry.file_name().to_string_lossy().into_owned();
            if re.is_match(&file_name) {
                files.push(PathBuf::from(entry.path()))
            }
        }
    }

    return files;
}

pub fn find_files_by_regex_inverse(dir: &Path, pattern: &str) -> Vec<PathBuf> {
    let re = Regex::new(pattern).unwrap();
    let mut files: Vec<PathBuf> = Vec::new();
    let entries = fs::read_dir(dir).ok();

    for entry in entries.into_iter().flatten() {
        let entry_unwrap = entry.unwrap_or_else(|e| panic!("Failed to read directory entry: {}", e));
        if *(&entry_unwrap.file_type().unwrap().is_file()) {
            let file_name = &entry_unwrap.file_name().to_string_lossy().into_owned();
            if !re.is_match(&file_name) {
                files.push(PathBuf::from(&entry_unwrap.path()))
            }
        }
    }

    return files;
}

pub fn get_folder_by_regex(path: &Path, pattern: &str) -> Option<PathBuf> {
    let re = Regex::new(pattern).unwrap();
    if path.is_dir() {
        for entry in fs::read_dir(&path).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                let folder_name = entry.file_name().to_string_lossy().into_owned();
                if re.is_match(&folder_name) {
                    return Some(entry.path());
                }
            }
        }
    }
    None
}

pub fn copy_directory_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    // 1. Ensure the destination directory folder path exists
    fs::create_dir_all(dst)?;

    // 2. Read the items inside the current source directory folder boundary
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        // 3. Build unique absolute or relative PathBuf variants for each entry target
        let src_entry_path: PathBuf = entry.path();
        let dst_entry_path: PathBuf = dst.join(entry.file_name());
        // 4. Branch logic execution based on whether the entity is a nested subfolder
        if file_type.is_dir() {
            // Recursively pass PathBuf references down into the next directory tier
            copy_directory_recursive(&src_entry_path, &dst_entry_path)?;
        } else {
            // Safe to copy the file directly since it is confirmed not to be a directory
            fs::copy(&src_entry_path, &dst_entry_path)?;
        }
    }
    Ok(())
}