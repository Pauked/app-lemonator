use std::{
    env,
    fs::File,
    path::{Path, PathBuf},
};

use colored::Colorize;
use eyre::{eyre, Context, Report, Result};
use log::{debug, error};
use plist::Value;
use powershell_script::PsScriptBuilder;

use crate::{
    constants, data,
    data::SearchMethod,
    paths::{self},
};

pub fn get_app_file_version(
    app: data::App,
    app_file_version: Option<data::FileVersion>,
) -> Result<data::FileVersion, Report> {
    match app_file_version {
        Some(app_file_version) => Ok(app_file_version),
        None => search_for_app_file_version(app.clone()).wrap_err(format!(
            "Failed to get app details for app '{}' using search method '{}' and search term '{}'",
            app.app_name, app.search_method, app.search_term
        )),
    }
}

fn search_for_app_file_version(app: data::App) -> Result<data::FileVersion, Report> {
    let app_path = match app.search_method {
        SearchMethod::PSGetApp => Ok(get_powershell_getxapppackage(app)?),
        SearchMethod::FolderSearch => Ok(get_folder_search(app)?),
        SearchMethod::Shortcut => Ok(get_shortcut(app)?),
    };

    match app_path {
        Ok(app_path) => {
            let app_file_version = get_file_version(&app_path)?;
            Ok(app_file_version)
        }
        Err(e) => Err(e),
    }
}

fn run_powershell_cmd(powershell_cmd: &str) -> Result<Vec<String>, Report> {
    if env::consts::OS != constants::OS_WINDOWS {
        return Err(eyre!(format!(
            "PowerShell is only supported on Windows, not on '{}'",
            env::consts::OS
        )));
    }

    let ps = PsScriptBuilder::new()
        .no_profile(true)
        .non_interactive(true)
        .hidden(false)
        .print_commands(false)
        .build();

    let output = ps.run(powershell_cmd).wrap_err(format!(
        "Failed to run powershell command '{}'",
        powershell_cmd
    ))?;

    let stdout_result = &output.stdout();
    match stdout_result {
        None => Err(eyre!(format!(
            "No stdout from PowerShell, command was '{}'",
            powershell_cmd
        ),)),
        Some(stdout_text) => Ok(stdout_text.split("\r\n").map(|s| s.to_string()).collect()),
    }
}

fn get_property_from_stdout(stdout_strings: Vec<String>, property_name: &str) -> String {
    let binding = "".to_string();
    let property = stdout_strings
        .iter()
        .find(|s| s.starts_with(property_name))
        .unwrap_or(&binding);
    let binding = property.replace(property_name, "");
    let property_value = binding.trim();
    property_value.to_string()
}

fn get_powershell_getxapppackage(app: data::App) -> Result<String, Report> {
    let stdout_strings = run_powershell_cmd(&format!(
        r#"Get-AppXPackage -Name {} | Format-List InstallLocation"#,
        app.search_term
    ))?;

    let app_path = get_property_from_stdout(stdout_strings, "InstallLocation : ");
    let mut full_app_path = PathBuf::from(&app_path);
    full_app_path.push(&app.exe_name);

    Ok(full_app_path.to_string_lossy().to_string())
}

fn get_windows_file_version_information(
    full_path: &str,
) -> Result<(String, String, String, String), eyre::Report> {
    let stdout_result = run_powershell_cmd(&format!(
        r#"(Get-Item "{}").VersionInfo.FileVersionRaw | Format-List -Property Major, Minor, Build, Revision"#,
        full_path
    ));

    match stdout_result {
        Ok(stdout_strings) => {
            let major = get_property_from_stdout(stdout_strings.clone(), "Major    : ");
            let minor = get_property_from_stdout(stdout_strings.clone(), "Minor    : ");
            let build = get_property_from_stdout(stdout_strings.clone(), "Build    : ");
            let revision = get_property_from_stdout(stdout_strings, "Revision : ");

            Ok((major, minor, build, revision))
        }
        Err(e) => Err(e),
    }
}

fn get_windows_file_description_information(full_path: &str) -> Result<String, eyre::Report> {
    let stdout_result = run_powershell_cmd(&format!(
        r#"(Get-Item "{}").VersionInfo | Format-List -Property FileDescription"#,
        full_path
    ));

    match stdout_result {
        Ok(stdout_strings) => {
            let app_description =
                get_property_from_stdout(stdout_strings.clone(), "FileDescription :");
            Ok(app_description)
        }
        Err(e) => Err(e),
    }
}

fn get_macos_file_version_information(full_path: &str) -> Result<(String, String), eyre::Report> {
    // Construct path to Info.plist
    let plist_path = Path::new(full_path).join("Contents").join("Info.plist");

    // Open the plist file
    let file = File::open(plist_path)?;

    // Parse the plist file
    let value: Value = plist::from_reader(file)?;

    // Extract the version information from the plist
    let version = value
        .as_dictionary()
        .and_then(|dict| dict.get("CFBundleShortVersionString"))
        .and_then(|title| title.as_string());

    let version_str = version.unwrap_or("");

    let app_description = value
        .as_dictionary()
        .and_then(|dict| dict.get("CFBundleName"))
        .and_then(|title| title.as_string());

    let app_description_str = app_description.unwrap_or("");

    Ok((version_str.to_string(), app_description_str.to_string()))
}

pub fn parse_file_version_to_i32(version: &str) -> Option<(u32, u32, u32, u32)> {
    let parts: Vec<_> = version
        .split('.')
        .map(|part| part.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
        .ok()?; // FIXME: Improve error handling?

    match parts.len() {
        1 => Some((parts[0], 0, 0, 0)),
        2 => Some((parts[0], parts[1], 0, 0)),
        3 => Some((parts[0], parts[1], parts[2], 0)),
        4 => Some((parts[0], parts[1], parts[2], parts[3])),
        _ => None,
    }
}
/*
// FIXME: Refactor error handling in get_file_version
fn get_file_version(full_path: &str) -> Result<FileVersion, Report> {
    let stdout_result = run_powershell_cmd(&format!(
        r#"(Get-Item "{}").VersionInfo.FileVersionRaw | Format-List -Property Major, Minor, Build, Revision"#,
        full_path
    ));

    match stdout_result {
        Ok(stdout_strings) => {
            let major = get_property_from_stdout(stdout_strings.clone(), "Major    : ");
            let minor = get_property_from_stdout(stdout_strings.clone(), "Minor    : ");
            let build = get_property_from_stdout(stdout_strings.clone(), "Build    : ");
            let revision = get_property_from_stdout(stdout_strings, "Revision : ");

            Ok(FileVersion {
                app: full_path.to_string(),
                major: major.parse::<u32>().unwrap_or(0),
                minor: minor.parse::<u32>().unwrap_or(0),
                build: build.parse::<u32>().unwrap_or(0),
                revision: revision.parse::<u32>().unwrap_or(0),
            })
        }
        Err(e) => Err(e),
    }
}
*/

fn get_file_version(full_path: &str) -> Result<data::FileVersion, eyre::Report> {
    if env::consts::OS == constants::OS_WINDOWS {
        let (major, minor, build, revision) = get_windows_file_version_information(full_path)?;
        let app_description = get_windows_file_description_information(full_path)?;

        return Ok(data::FileVersion {
            app_description: app_description.to_string(),
            path: full_path.to_string(),
            major: major.parse::<u32>().unwrap_or(0),
            minor: minor.parse::<u32>().unwrap_or(0),
            build: build.parse::<u32>().unwrap_or(0),
            revision: revision.parse::<u32>().unwrap_or(0),
        });
    }

    if env::consts::OS == constants::OS_MACOS {
        let (version, app_description) = get_macos_file_version_information(full_path)?;

        match parse_file_version_to_i32(&version) {
            Some((major, minor, build, revision)) => {
                return Ok(data::FileVersion {
                    app_description,
                    path: full_path.to_string(),
                    major,
                    minor,
                    build,
                    revision,
                });
            }
            None => {
                return Err(eyre::eyre!(format!(
                    "Failed to parse the version string - '{}",
                    version,
                )))
            }
        }
    }

    Err(eyre::eyre!(format!(
        "get_file_version is only supported on Windows and MacOS, not on '{}'",
        env::consts::OS
    )))
}

fn get_folder_search(app: data::App) -> Result<String, Report> {
    debug!("get_folder_search for app '{}'", app.app_name.blue());
    let mut files: Vec<String> = Vec::new();

    let base_folder = paths::get_base_folder(&app.search_term);

    if !paths::folder_exists(&base_folder) {
        return Err(eyre!(format!(
            "Base Folder '{}' does not exist",
            &base_folder
        ),));
    }

    paths::find_file_in_folders(&base_folder, &app.exe_name, &mut files);

    if files.is_empty() {
        return Err(eyre!(format!(
            "No matches found for '{}' using recursive search in folder '{}'",
            &app.exe_name, &base_folder
        )));
    }

    if env::consts::OS == constants::OS_WINDOWS || env::consts::OS == constants::OS_MACOS {
        debug!("Checking file versions for {} files", files.len());

        // Get version details for all found files
        let mut file_versions: Vec<data::FileVersion> = Vec::new();
        for file in &files {
            debug!("File: '{}'", file);
            let file_version_result = get_file_version(file);
            match file_version_result {
                Ok(file_version) => {
                    debug!("File version: {:?}", file_version);
                    file_versions.push(file_version);
                }
                Err(e) => {
                    error!("Failed to get file version for '{}': {:?}", file, e);
                }
            }
        }

        // TODO: Add comparison to existing app version, if available!

        // Now found the highest versioned file
        let highest_version = file_versions
            .iter()
            .max_by_key(|v| (v.major, v.minor, v.build, v.revision))
            .unwrap();
        debug!("Highest version: {:?}", &highest_version);

        return Ok(highest_version.path.clone());
    }

    Err(eyre!(format!(
        "Unsupported OS '{}', for folder search",
        env::consts::OS
    )))
}

fn get_shortcut(app: data::App) -> Result<String, Report> {
    debug!("get_shortcut_search");

    let base_folder = paths::get_base_folder(&app.search_term);
    let mut path = PathBuf::from(base_folder);
    path.push(&app.exe_name);

    if paths::check_app_exists(&path.to_string_lossy()) {
        return Ok(path.to_string_lossy().to_string());
    }

    Err(eyre!(format!(
        "File does not exist '{}'",
        path.to_string_lossy()
    )))
}
