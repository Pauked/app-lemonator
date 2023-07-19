use colored::Colorize;
use powershell_script::PsScriptBuilder;
use std::{env, fs, process::Command};

use crate::db;

pub fn run_app(app: db::App) {
    if app.search_method.to_lowercase() == "get-appxpackage" {
        run_appxpackage(app);
        return;
    }

    if app.search_method.to_lowercase() == "localappdata" {
        run_localappdata(app);
        return;
    }

    println!("Unknown search method");
}

fn open_process(app: db::App, full_app_name: &str) {
    let result = Command::new(full_app_name).spawn();
    match result {
        Ok(_) => println!("Opened {}! - {}", &app.app, &full_app_name),
        Err(e) => eprintln!("Failed to open {}: {:?}", &app.app, e),
    }
}

fn run_appxpackage(app: db::App) {
    let ps = PsScriptBuilder::new()
        .no_profile(true)
        .non_interactive(true)
        .hidden(false)
        .print_commands(false)
        .build();

    let powershell_cmd = format!(
        r#"Get-AppXPackage -Name {} | Select InstallLocation"#,
        app.search_term
    );
    let output = ps.run(&powershell_cmd).unwrap();

    let stdout_text = &output.stdout().unwrap();
    let stdout_strings = stdout_text.split("\r\n").collect::<Vec<&str>>();
    let app_path = stdout_strings[3]; // welcome to assumption corner
    let full_app_name = format!("{}\\{}", app_path, app.exe_name);

    open_process(app, &full_app_name);
}

fn run_localappdata(app: db::App) {
    let mut files: Vec<String> = Vec::new();
    find_file_in_folders(&get_local_app_data_folder(), &app.exe_name, &mut files);

    if files.is_empty() {
        eprintln!("Failed to find file {}", &app.exe_name);
        return;
    }

    files.sort();
    files.reverse();
    open_process(app, &files[0]);
}

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
    get_environment_folder("LOCALAPPDATA")
}

pub fn find_file_in_folders(root_folder: &str, find_file: &str, results: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(root_folder) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                let file_name = path.file_name().unwrap().to_string_lossy();
                let folder = path.parent().unwrap().to_string_lossy();

                if path.is_dir() {
                    if let Some(path_str) = path.to_str() {
                        find_file_in_folders(path_str, find_file, results);
                    }
                } else if path.is_file() && file_name.to_lowercase() == find_file.to_lowercase() {
                    let full_file = format!("{}\\{}", folder, file_name);
                    results.push(full_file.clone());
                    println!("Found file: {}", full_file.red());
                }
            }
        }
    }
}
