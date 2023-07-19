use std::process::Command;

use powershell_script::PsScriptBuilder;

use crate::db;

pub fn run_app(app: db::App) {

    if app.search_method.to_lowercase() == "get-appxpackage" {
        run_appxpackage(app);
        return;
    }

    println!("Unknown search method");
}

fn run_appxpackage(app: db::App) {
    let ps = PsScriptBuilder::new()
        .no_profile(true)
        .non_interactive(true)
        .hidden(false)
        .print_commands(false)
        .build();

    let powershell_cmd = format!(r#"Get-AppXPackage -Name {} | Select InstallLocation"#, app.search_term);
    let output = ps
        .run(&powershell_cmd)
        .unwrap();

    let stdout_text = &output.stdout().unwrap();
    let stdout_strings = stdout_text.split("\r\n").collect::<Vec<&str>>();
    let app_path = stdout_strings[3]; // welcome to assumption corner
    let full_app_name = format!("{}\\{}", app_path, app.exe_name);

    let result = Command::new(&full_app_name).spawn();
    match result {
        Ok(_) => println!("Opened {}! - {}", &app.app, &full_app_name),
        Err(e) => eprintln!("Failed to open {}: {:?}", &app.app, e),
    }
}

/*
fn list_files_and_folders(root_folder: &str) -> Result<(), std::io::Error> {
    let entries = fs::read_dir(root_folder)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy();

        if entry.file_type()?.is_dir() {
            println!("Folder: {}", file_name);
            if let Some(path_str) = path.to_str() {
                list_files_and_folders(path_str)?;
            }
        } else {
            println!("File: {}", file_name);
        }
    }

    Ok(())
}

*/
