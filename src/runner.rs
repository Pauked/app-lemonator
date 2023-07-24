use std::{process::Command, env};

use crate::{cli, db, finder};

pub async fn run_app(app: db::App) {
    match app.found_path {
        Some(ref found_path) => {
            open_process(app.clone(), found_path).await;
        }
        None => {
            if app.search_method == cli::SearchMethod::PSGetApp.to_string() {
                let found_path = finder::get_powershell_getxapppackage(app.clone());
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

            if app.search_method == cli::SearchMethod::FolderSearch.to_string() {
                let found_path = finder::get_folder_search(app.clone());
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

            if app.search_method == cli::SearchMethod::Shortcut.to_string() {
                let found_path = finder::get_shortcut(app.clone());
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
    /*
    if app.search_method == cli::SearchMethod::PSGetApp.to_string() {
        run_powershell_getxapppackage(app);
        return;
    }

    if app.search_method == cli::SearchMethod::FolderSearch.to_string() {
        run_folder_search(app);
        return;
    }
    */

    //eprintln!("Unknown search method");
}

async fn open_process(app: db::App, full_app_name: &str) {
    if env::consts::OS == "windows" {
        let result = Command::new(full_app_name).spawn();
        match result {
            Ok(_) => println!("Opened '{}'! - '{}'", &app.app_name, &full_app_name),
            Err(e) => eprintln!("Failed to open '{}': {:?}", &app.app_name, e),
        }
    } else if env::consts::OS == "macos" {
        let result = Command::new("open").arg(full_app_name).spawn();
        match result {
            Ok(_) => println!("Opened '{}'! - '{}'", &app.app_name, &full_app_name),
            Err(e) => eprintln!("Failed to open '{}': {:?}", &app.app_name, e),
        }
    }
    db::update_last_run(&app.app_name, app.id).await;

}
