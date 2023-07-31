use std::process::Command;

use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm};
use tabled::settings::{object::Rows, Modify, Style, Width};

use crate::{cli, db, finder};

pub async fn create_db() {
    db::create_db().await;
}

pub async fn open_app(app_name: &str) {
    let app = db::get_app(app_name).await;
    //println!("Running app '{}'", app.app_name);
    //runner::run_app(app).await;
    // TODO: If app not found, error code?

    let app_path = finder::get_app_path(app.clone(), app.app_path.clone());
    if app_path.is_empty() {
        return;
    }

    open_process(app.clone(), &app_path).await;
    db::update_app_path(&app.app_name, app.id, &app_path).await;
}

pub async fn add_app(
    app_name: String,
    exe_name: String,
    search_term: String,
    search_method: cli::SearchMethod,
) {
    db::add_app(&app_name, &exe_name, &search_term, search_method).await;
}

pub async fn delete_app(app_name: &str) {
    db::delete_app(app_name).await;
}

async fn update_app_path_for_list(apps: Vec<db::App>) {
    for app in apps {
        let app_path = finder::get_app_path(app.clone(), None);
        if !app_path.is_empty() {
            db::update_app_path(&app.app_name, app.id, &app_path).await;
        }
    }
}

pub async fn update_app(app_name: Option<String>) {
    match app_name {
        Some(app_name) => {
            update_app_path_for_list(vec![db::get_app(&app_name).await]).await;
        }
        None => {
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(
                    "Do you want to update the app path for all apps? This may take a while.",
                )
                .interact()
                .unwrap()
            {
                update_app_path_for_list(db::get_apps().await).await;
            }
        }
    }
}

pub async fn list_app(app_name: Option<String>) {
    match app_name {
        Some(app_name) => {
            let app = db::get_app(&app_name).await;
            //println!("{:#?}", app);
            println!("{}", tabled::Table::new(vec![app]).with(Style::modern()));
        }
        None => {
            let apps = db::get_apps().await;
            println!("{}", "App Listing".blue());
            println!(
                "{}",
                tabled::Table::new(apps)
                    .with(Modify::new(Rows::new(1..)).with(Width::wrap(30).keep_words()))
                    .with(Style::modern())
            );
        }
    }
}

pub fn reset() {
    // Prompt the user for confirmation to delete the file
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to reset the database? All data will be deleted.")
        .interact()
        .unwrap()
    {
        db::reset_db();
    } else {
        println!("Reset not confirmed.");
    }
}

pub fn testings() {
    // Could be any code calls, my WIP section
    println!("Testing!");
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

    db::update_last_opened(&app.app_name, app.id).await;
}
