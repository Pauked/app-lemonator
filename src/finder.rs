use std::{
    env,
    io::{Error, ErrorKind},
    path::PathBuf,
};

use colored::Colorize;
use log::{error, debug};
use powershell_script::PsScriptBuilder;

use crate::{cli::SearchMethod, constants, db, paths};

#[derive(Debug)]
pub struct FileVersion {
    pub app: String,
    pub major: u32,
    pub minor: u32,
    pub build: u32,
    pub revision: u32,
}

pub fn get_app_path(app: db::App, app_path: Option<String>) -> String {
    match app_path {
        Some(app_path) => app_path,
        None => match search_for_app_path(app.clone()) {
            Ok(app_path) => app_path,
            Err(e) => {
                error!("Failed to find app path '{}': {:?}", app.app_name.blue(), e);
                String::new()
            }
        },
    }
}

fn search_for_app_path(app: db::App) -> Result<String, Error> {
    let search_method: SearchMethod = app.search_method.parse().unwrap();
    match search_method {
        SearchMethod::PSGetApp => get_powershell_getxapppackage(app),
        SearchMethod::FolderSearch => get_folder_search(app),
        SearchMethod::Shortcut => get_shortcut(app),
    }
}

fn run_powershell_cmd(powershell_cmd: &str) -> Vec<String> {
    #[cfg(target_os = "macos")]
    // FIXME: Rework code so you CANNOT get here
    panic!("Powershell is not supported on Mac");

    let ps = PsScriptBuilder::new()
        .no_profile(true)
        .non_interactive(true)
        .hidden(false)
        .print_commands(false)
        .build();

    let output = ps.run(powershell_cmd).unwrap();

    let stdout_text = &output.stdout().unwrap();
    stdout_text.split("\r\n").map(|s| s.to_string()).collect()
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

fn get_powershell_getxapppackage(app: db::App) -> Result<String, Error> {
    let stdout_strings = run_powershell_cmd(&format!(
        r#"Get-AppXPackage -Name {} | Format-List InstallLocation"#,
        app.search_term
    ));

    let app_path = get_property_from_stdout(stdout_strings, "InstallLocation : ");
    let mut full_app_name = PathBuf::from(&app_path);
    full_app_name.push(&app.exe_name);

    Ok(full_app_name.to_string_lossy().to_string())
}

fn get_file_version(full_path: &str) -> FileVersion {
    let stdout_strings = run_powershell_cmd(&format!(
        r#"(Get-Item "{}").VersionInfo.FileVersionRaw | Format-List -Property Major, Minor, Build, Revision"#,
        full_path
    ));

    let major = get_property_from_stdout(stdout_strings.clone(), "Major    : ");
    let minor = get_property_from_stdout(stdout_strings.clone(), "Minor    : ");
    let build = get_property_from_stdout(stdout_strings.clone(), "Build    : ");
    let revision = get_property_from_stdout(stdout_strings, "Revision : ");

    FileVersion {
        app: full_path.to_string(),
        major: major.parse::<u32>().unwrap_or(0),
        minor: minor.parse::<u32>().unwrap_or(0),
        build: build.parse::<u32>().unwrap_or(0),
        revision: revision.parse::<u32>().unwrap_or(0),
    }
}

fn get_folder_search(app: db::App) -> Result<String, Error> {
    debug!("get_folder_search for app '{}'", app.app_name.blue());
    let mut files: Vec<String> = Vec::new();

    let base_search_folder = paths::get_base_search_folder(&app.search_term);

    if !paths::folder_exists(&base_search_folder) {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Base Search Folder '{}' does not exist", &base_search_folder),
        ));
    }

    paths::find_file_in_folders(&base_search_folder, &app.exe_name, &mut files);

    if files.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Failed to find file '{}'", &app.exe_name),
        ));
    }

    if env::consts::OS == constants::OS_WINDOWS {
        debug!("Checking file versions for {} files", files.len());

        // Get version details for all found files
        let mut file_versions: Vec<FileVersion> = Vec::new();
        for file in &files {
            debug!("File: '{}'", file);
            let file_version = get_file_version(file);
            debug!("File version: {:?}", file_version);
            file_versions.push(file_version);
        }

        // Now found the highest versioned file
        let highest_version = file_versions
            .iter()
            .max_by_key(|v| (v.major, v.minor, v.build, v.revision))
            .unwrap();
        debug!("Highest version: {:?}", &highest_version);

        return Ok(highest_version.app.clone());
    } else if env::consts::OS == constants::OS_MAC && files.len() == 1 {
        // FIXME: This is a hack for now. Need file versio checking for Mac.
        debug!("App found: {:?}", files[0].clone());
        return Ok(files[0].clone());
    }

    Err(Error::new(ErrorKind::Unsupported, "Unsupported OS"))
}

fn get_shortcut(app: db::App) -> Result<String, Error> {
    debug!("get_shortcut_search");

    let mut path = PathBuf::from(&app.search_term);
    path.push(&app.exe_name);

    if paths::check_app_exists(&path.to_string_lossy()) {
        return Ok(path.to_string_lossy().to_string());
    }

    Err(Error::new(
        ErrorKind::NotFound,
        format!("Failed to find file '{}'", path.to_string_lossy()),
    ))
}
