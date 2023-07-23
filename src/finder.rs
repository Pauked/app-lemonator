use std::{path::PathBuf, io::{ErrorKind, Error}};

use powershell_script::PsScriptBuilder;

use crate::{db, paths};

#[derive(Debug)]
pub struct FileVersion {
    pub app: String,
    pub major: u32,
    pub minor: u32,
    pub build: u32,
    pub revision: u32,
}

fn run_powershell_cmd(powershell_cmd: &str) -> Vec<String> {
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

pub fn get_powershell_getxapppackage(app: db::App) -> Result<String, Error> {
    let stdout_strings = run_powershell_cmd(&format!(
        r#"Get-AppXPackage -Name {} | Format-List InstallLocation"#,
        app.search_term
    ));

    let app_path = get_property_from_stdout(stdout_strings, "InstallLocation : ");
    let mut full_app_name = PathBuf::from(&app_path);
    full_app_name.push(&app.exe_name);

    //open_process(app, &full_app_name.to_string_lossy());

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

pub fn get_folder_search(app: db::App) -> Result<String, Error> {
    let mut files: Vec<String> = Vec::new();

    let base_search_folder = paths::get_base_search_folder(&app.search_term);
    if !paths::folder_exists(&base_search_folder) {
        return Err(Error::new(ErrorKind::InvalidData, format!("Folder '{}' does not exist", &base_search_folder)));
    }

    paths::find_file_in_folders(&base_search_folder, &app.exe_name, &mut files);

    if files.is_empty() {
        return Err(Error::new(ErrorKind::InvalidData, format!("Failed to find file '{}'", &app.exe_name)));
    }

    // Get version details for all found files
    let mut file_versions: Vec<FileVersion> = Vec::new();
    for file in &files {
        println!("File: '{}'", file);
        let file_version = get_file_version(file);
        println!("File version: {:?}", file_version);
        file_versions.push(file_version);
    }

    // Now found the highest versioned file
    let highest_version = file_versions
        .iter()
        .max_by_key(|v| (v.major, v.minor, v.build, v.revision))
        .unwrap();
    println!("Highest version: {:?}", &highest_version);

    // Open it fella!
    //open_process(app, highest_version.app.as_str());

    Ok(highest_version.app.clone())
}
