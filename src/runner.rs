use std::process::Command;

use crate::{db, finder, cli::SearchMethod};

pub async fn run_app(app: db::App) {
    match app.found_path {
        Some(ref found_path) => {
            open_process(app.clone(), found_path).await;
        }
        None => {
            let search_method: SearchMethod = app.search_method.parse().unwrap();
            let found_path = match search_method {
                SearchMethod::PSGetApp => finder::get_powershell_getxapppackage(app.clone()),
                SearchMethod::FolderSearch => finder::get_folder_search(app.clone()),
                SearchMethod::Shortcut => finder::get_shortcut(app.clone()),
            };

            match found_path {
                Ok(found_path) => {
                    open_process(app.clone(), &found_path).await;
                    db::update_app_found_path(&app.app_name, app.id, &found_path).await;
                }
                Err(e) => {
                    eprintln!("Failed to find app '{}': {:?}", app.app_name, e);
                }
            }
        }
    }
}

async fn open_process(app: db::App, full_app_name: &str) {
    #[cfg(target_os = "windows")]
    let result = Command::new(full_app_name).spawn();
    #[cfg(target_os = "macos")]
    let result = Command::new("open").arg(full_app_name).spawn();

    match result {
        Ok(_) => println!("Opened '{}'! - '{}'", &app.app_name, &full_app_name),
        Err(e) => eprintln!("Failed to open '{}': {:?}", &app.app_name, e),
    }

    db::update_last_run(&app.app_name, app.id).await;

}
