use std::{
    env,
    path::{Path, PathBuf},
};

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, error};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use walkdir::WalkDir;

use crate::constants;

const ROAMINGAPPDATA: &str = "appdata";
const LOCALAPPDATA: &str = "localappdata";
const PERSONALDROPBOX: &str = "personaldropbox";
const BUSINESSDROPBOX: &str = "businessdropbox";

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
        debug!(
            "Environment '{}' returns folder: '{}'",
            name,
            appdata_path.display()
        );
        return appdata_path.display().to_string();
    } else {
        error!("Failed to retrieve environment '{}' folder.", name);
    }

    String::from("")
}

pub fn get_roaming_app_data_folder() -> String {
    get_environment_folder(ROAMINGAPPDATA)
}

pub fn get_local_app_data_folder() -> String {
    get_environment_folder(LOCALAPPDATA)
}

#[derive(Debug, Deserialize, Serialize)]
struct DropboxConfig {
    personal: Option<DropboxInfo>,
    business: Option<DropboxInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DropboxInfo {
    path: String,
    //host: u64,
    //is_team: bool,
    //subscription_type: String,
}

fn get_dropbox_folder(tag: &str) -> String {
    // https://help.dropbox.com/installs/locate-dropbox-folder

    let mut dropbox_config_path = String::new();
    if env::consts::OS == constants::OS_WINDOWS {
        // If Windows, find and load: %APPDATA%\Dropbox\info.json or %LOCALAPPDATA%\Dropbox\info.json
        let dropbox_location = "Dropbox\\info.json";

        // Check the Roaming folder first
        dropbox_config_path = get_full_path(&get_roaming_app_data_folder(), dropbox_location);
        debug!(
            "Dropbox config path, roaming appdata: '{}'",
            dropbox_config_path
        );

        if !file_exists(&dropbox_config_path) {
            debug!("  Not found in roaming appdata, checking local appdata.");
            // Check the Local folder
            dropbox_config_path = get_full_path(&get_local_app_data_folder(), dropbox_location);
            debug!(
                "Dropbox config path, local appdata: '{}'",
                dropbox_config_path
            );
        }

        if !file_exists(&dropbox_config_path) {
            error!("Failed to find Dropbox config file.");
            return String::new();
        }
    }

    if env::consts::OS == constants::OS_MACOS {
        panic!("Implement get_dropbox_folder() for macOS");
    }

    let json = std::fs::read_to_string(&dropbox_config_path).unwrap();
    debug!("Dropbox config JSON: '{}'", &json);

    get_dropbox_folder_from_json(tag, &dropbox_config_path, &json)
}

fn get_dropbox_folder_from_json(tag: &str, dropbox_config_path: &str, json: &str) -> String {
    // Deserialize the JSON data into the Rust struct
    let dropbox_config: DropboxConfig = match from_str(json) {
        Ok(config) => config,
        Err(e) => {
            error!(
                "Error parsing Dropbox JSON from '{}', error {}",
                dropbox_config_path, e
            );
            return String::new();
        }
    };

    if tag == PERSONALDROPBOX {
        if let Some(personal) = dropbox_config.personal {
            return personal.path;
        }
    } else if tag == BUSINESSDROPBOX {
        if let Some(business) = dropbox_config.business {
            return business.path;
        }
    }

    String::new()
}

pub fn get_base_folder(source_folder: &str) -> String {
    debug!("Source base folder: '{}'", source_folder);
    let mut output = source_folder.to_string();

    // Look for special flags... at the start of the folder
    let re = Regex::new(r#"%([^%]+)%"#).unwrap();

    if let Some(capture) = re.captures(source_folder) {
        let captured_value = &capture[1];
        let mut env_var_value = String::new();
        debug!("  Found path variable: '{}'", captured_value);

        match captured_value.to_lowercase().as_str() {
            LOCALAPPDATA => {
                env_var_value = get_local_app_data_folder();
            }
            ROAMINGAPPDATA => {
                env_var_value = get_roaming_app_data_folder();
            }
            PERSONALDROPBOX => {
                env_var_value = get_dropbox_folder(PERSONALDROPBOX);
            }
            BUSINESSDROPBOX => {
                env_var_value = get_dropbox_folder(BUSINESSDROPBOX);
            }
            _ => {
                error!("Unknown path variable: '{}'", captured_value);
            }
        }
        output = re
            .replace_all(source_folder, env_var_value.as_str())
            .to_string();

        debug!("Expanded base folder: '{}'", output);
    }

    output
}

pub fn parse_arguments(input: &str) -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    let mut current_arg = String::new();
    let mut inside_quotes = false;

    for c in input.chars() {
        match c {
            ' ' if !inside_quotes => {
                if !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            '"' => {
                inside_quotes = !inside_quotes;
                //current_arg.push(c);
            }
            _ => {
                current_arg.push(c);
            }
        }
    }

    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    args
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "windows")]
    use std::path::PathBuf;

    #[cfg(target_os = "windows")]
    use crate::paths::{get_base_folder, get_local_app_data_folder};
    use crate::paths::{get_dropbox_folder_from_json, BUSINESSDROPBOX, PERSONALDROPBOX, parse_arguments};

    #[cfg(target_os = "windows")]
    #[test]
    fn check_local_app_data_folder() {
        // Arrange
        let source_path = r#"%localappdata%\JetBrains"#;
        let mut file_path = PathBuf::from(get_local_app_data_folder());
        file_path.push("JetBrains");
        let expected = file_path.display().to_string();

        // Act
        let actual = get_base_folder(source_path);

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_dropbox_folders() {
        // Arrange
        let expected_personal = r#"C:\DropBox\Personal"#;
        let expected_business = r#"C:\DropBox\Business"#;
        let json_data = r#"
{
    "personal": {
        "path": "C:\\DropBox\\Personal",
        "host": 123456789,
        "is_team": false,
        "subscription_type": "Basic"
    },
    "business": {
        "path": "C:\\DropBox\\Business",
        "host": 123456789,
        "is_team": true,
        "subscription_type": "Business"
    }
}"#;

        // Act
        let actual_personal = get_dropbox_folder_from_json(PERSONALDROPBOX, "test", json_data);
        let actual_business = get_dropbox_folder_from_json(BUSINESSDROPBOX, "test", json_data);

        // Assert
        assert_eq!(actual_personal, expected_personal);
        assert_eq!(actual_business, expected_business);
    }

    #[test]
    fn parse_arguments_chrome_default_profile() {
        // Arrange
        let input = r#" --args --profile-directory="Default""#;
        let expected = vec![
            "--args".to_string(),
            "--profile-directory=Default".to_string(),
        ];

        // Act
        let actual = parse_arguments(input);

        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_arguments_chrome_profile_1() {
        // Arrange
        let input = r#" --args --profile-directory="Profile 1""#;
        let expected = vec![
            "--args".to_string(),
            "--profile-directory=Profile 1".to_string(),
        ];

        // Act
        let actual = parse_arguments(input);

        // Assert
        assert_eq!(actual, expected);
    }
}
