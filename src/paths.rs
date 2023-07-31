use std::{
    env,
    path::{Path, PathBuf},
};

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, error};
use regex::Regex;
use walkdir::WalkDir;

const LOCALAPPDATA: &str = "localappdata";

pub fn find_file_in_folders(root_folder: &str, find_file: &str, results: &mut Vec<String>) {
    debug!("find_file_in_folders: '{}'", root_folder);

    // Create a new progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} [{elapsed_precise}] {msg}").unwrap(),
    );

    let mut found_count = 0;

    for entry in WalkDir::new(root_folder) {
        if let Ok(entry) = entry {
            // Check if the file name matches
            if entry.file_name().to_string_lossy().to_lowercase() == find_file.to_lowercase() {
                found_count += 1;
                results.push(entry.path().display().to_string());
            }

            // Set the message to the currently-searched directory
            pb.set_message(format!(
                "({}) Searching: '{}'",
                get_matches_count(found_count),
                truncate_middle(&entry.path().display().to_string(), 80)
            ));
        }

        pb.inc(1); // Increase the spinner's step
    }

    debug!("Match files found - {:?}", results);

    if found_count > 0 {
        pb.finish_with_message(format!(
            "Finished searching, found {}.",
            get_matches_count(found_count)
        ));
        return;
    }

    pb.finish_with_message(format!("Finished searching,'{}' not found.", find_file));
}

fn get_matches_count(found_count: i32) -> String {
    if found_count == 0 {
        return "0 matches".to_string();
    }
    let result = format!("{} matches", found_count);
    result.green().to_string()
}

fn truncate_middle(input: &str, size_limit: usize) -> String {
    let input_len = input.len();

    if input_len <= size_limit {
        // No need to truncate, return the original string.
        return input.to_string();
    }

    let middle_index = input_len / 2;
    let half_size_limit = size_limit / 2;
    let start_index = middle_index - half_size_limit;
    let end_index = middle_index + half_size_limit;

    // Remove the middle section from the string.
    let mut output: String = input.to_string();
    output.replace_range(start_index..end_index, "..");
    output
}

pub fn folder_exists(folder_path: &str) -> bool {
    let path = Path::new(folder_path);
    path.is_dir()
}

pub fn file_exists(file_path: &str) -> bool {
    let path = Path::new(file_path);
    path.is_file()
}

pub fn check_app_exists(app_path: &str) -> bool {
    #[cfg(target_os = "macos")]
    return folder_exists(app_path);
    #[cfg(target_os = "windows")]
    return file_exists(app_path);
}

pub fn get_full_path(base_path: &str, file_name: &str) -> String {
    let mut file_path = PathBuf::new();
    file_path.push(base_path);
    file_path.push(file_name);
    file_path.display().to_string()
}

pub fn get_temp_dir() -> String {
    let temp_dir = env::temp_dir();
    temp_dir.display().to_string()
}

fn get_environment_folder(name: &str) -> String {
    if let Ok(appdata) = env::var(name) {
        let appdata_path = std::path::Path::new(&appdata);
        debug!("Environment '{}' returns folder: '{}'", name, appdata_path.display());
        return appdata_path.display().to_string();
    } else {
        error!("Failed to retrieve environment '{}' folder.", name);
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
    debug!("Source folder: '{}'", source_folder);
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
                error!("Unknown environment variable: '{}'", captured_value);
            }
        }
        output = re
            .replace_all(source_folder, env_var_value.as_str())
            .to_string();

        debug!("Base search folder: '{}'", output);
    }

    output
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "windows")]
    use std::path::PathBuf;

    #[cfg(target_os = "windows")]
    use crate::paths::{get_base_search_folder, get_local_app_data_folder};

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
