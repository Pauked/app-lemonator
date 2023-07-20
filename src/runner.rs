use std::process::Command;

use powershell_script::PsScriptBuilder;

use crate::{db, cli, paths};

pub fn run_app(app: db::App) {
    if app.search_method == cli::SearchMethod::PSGetApp.to_string() {
        run_powershell_getapp(app);
        return;
    }

    if app.search_method == cli::SearchMethod::FolderSearch.to_string() {
        run_folder_search(app);
        return;
    }

    eprintln!("Unknown search method");
}

fn open_process(app: db::App, full_app_name: &str) {
    let result = Command::new(full_app_name).spawn();
    match result {
        Ok(_) => println!("Opened {}! - {}", &app.app, &full_app_name),
        Err(e) => eprintln!("Failed to open {}: {:?}", &app.app, e),
    }
}

fn run_powershell_getapp(app: db::App) {
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

fn run_folder_search(app: db::App) {
    let mut files: Vec<String> = Vec::new();

    let base_search_folder = paths::get_base_search_folder(&app.search_term);
    if !paths::folder_exists(&base_search_folder) {
        eprintln!("Folder {} does not exist", &base_search_folder);
        return;
    }

    paths::find_file_in_folders(&base_search_folder, &app.exe_name, &mut files);

    if files.is_empty() {
        eprintln!("Failed to find file {}", &app.exe_name);
        return;
    }

    // TODO: Fix this hack to find the highest versioned path
    files.sort();
    files.reverse();
    open_process(app, &files[0]);
}
