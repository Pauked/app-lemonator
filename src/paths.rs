use std::{path::{Path, PathBuf}, env, fs};

use regex::Regex;

const LOCALAPPDATA: &str = "localappdata";

pub fn find_file_in_folders(root_folder: &str, find_file: &str, results: &mut Vec<String>) {
    println!("find_file_in_folders: {}", root_folder);

    if let Ok(entries) = fs::read_dir(root_folder) {
        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = path.file_name().unwrap();
            let folder = path.parent().unwrap();

            if path.is_dir() {
                if let Some(path_str) = path.to_str() {
                    find_file_in_folders(path_str, find_file, results);
                }
            } else if path.is_file() && file_name.to_string_lossy().to_lowercase() == find_file.to_lowercase() {
                let mut full_file = PathBuf::from(folder);
                full_file.push(file_name);
                results.push(full_file.display().to_string());
                println!("Found file: {}", full_file.display());
            }
        }
    }
}

pub fn folder_exists(folder_path: &str) -> bool {
    let path = Path::new(folder_path);
    path.is_dir()
}

/*
pub fn file_exists(file_path: &str) -> bool {
    let path = Path::new(file_path);
    path.is_file()
}
*/

fn get_environment_folder(name: &str) -> String {
    if let Ok(appdata) = env::var(name) {
        let appdata_path = std::path::Path::new(&appdata);
        println!("{} folder: {}", name, appdata_path.display());
        return appdata_path.display().to_string();
    } else {
        eprintln!("Failed to retrieve {} folder.", name);
    }

    String::from("")
}

/*
pub fn get_roaming_app_data_folder() -> String {
    get_environment_folder("APPDATA")
}
*/

pub fn get_local_app_data_folder() -> String {
    get_environment_folder(LOCALAPPDATA)
}

pub fn get_base_search_folder(source_folder: &str) -> String {

    println!("Source folder: '{}'", source_folder);
    let mut output = source_folder.to_string();

    // Look for special flags... at the start of the folder
    let re = Regex::new(r#"%([^%]+)%"#).unwrap();

    if let Some(capture) = re.captures(source_folder) {
        let captured_value = &capture[1];
        let mut env_var_value = String::new();

        match captured_value.to_lowercase().as_str() {
            LOCALAPPDATA => {
                env_var_value = get_local_app_data_folder();
            }
            _ => {
                eprintln!("Unknown environment variable: '{}'", captured_value);
            }
        }
        println!("Environment variable: '{}'", env_var_value);
        output = re.replace_all(source_folder, env_var_value.as_str()).to_string();

        println!("Output: {}", output);
    }

    output
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "windows")]
    use std::path::PathBuf;

    #[cfg(target_os = "windows")]
    use crate::paths::{get_local_app_data_folder, get_base_search_folder};

    #[cfg(target_os = "windows")]
    #[test]
    fn check_local_app_data_folder() {
        // Arrange
        let source_path = r#"%localappdata%\JetBrains"#;
        let mut file_path = PathBuf::from(get_local_app_data_folder());
        file_path.push("JetBrains");
        let expected = file_path.display().to_string();

        // Act
        let actual = get_base_search_folder(source_path);

        // Assert
        assert_eq!(actual, expected);
    }
}

