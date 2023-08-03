use std::process::Command;

use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm};
use log::{debug, error, info};
use tabled::settings::{object::Rows, Modify, Style, Width};

use crate::{cli, db, finder, paths};

pub async fn create_db() {
    db::create_db().await;
}

pub async fn open_app(app_name: &str) {
    match db::get_app(app_name).await {
        Ok(app) => {
            let app_path = finder::get_app_path(app.clone(), app.app_path.clone());
            if app_path.is_empty() {
                return;
            }

            open_process(app.clone(), &app_path).await;
            match db::update_app_path(app.id, &app_path).await {
                Ok(_) => {
                    debug!(
                        "Updated app for app_path '{}' to '{}'",
                        app.app_name.blue(),
                        app_path.magenta()
                    );
                }
                Err(error) => {
                    error!(
                        "Error updating app_path for '{}': {}",
                        app.app_name.blue(),
                        error
                    );
                }
            }
        }
        Err(e) => {
            error!("Failed to find app '{}': {:?}", app_name.blue(), e);
        }
    }
}

pub async fn add_app(
    app_name: String,
    exe_name: String,
    params: Option<String>,
    search_term: String,
    search_method: cli::SearchMethod,
) {
    match db::add_app(&app_name, &exe_name, &params, &search_term, &search_method).await {
        Ok(_) => {
            let param_info = if Some(params.clone()).is_some() {
                format!(" Params '{}'", params.unwrap().magenta())
            } else {
                String::new()
            };

            info!(
                "Added App Name '{}', Exe Name '{}', Search Term '{}', Search Method '{}'{}",
                app_name.blue(),
                exe_name.magenta(),
                search_term.magenta(),
                search_method,
                param_info
            );
        }
        Err(error) => {
            error!("Error adding app '{}': {}", app_name.blue(), error);
        }
    }
}

pub async fn delete_app(app_name: &str) {
    match db::delete_app(app_name).await {
        Ok(_) => {
            info!("Deleted app '{}'", app_name.blue());
        }
        Err(error) => {
            error!("Error deleting app '{}': {}", app_name, error);
        }
    }
}

async fn update_app_path_for_list(apps: Vec<db::App>) {
    for app in apps {
        let app_path = finder::get_app_path(app.clone(), None);
        if !app_path.is_empty() {
            match db::update_app_path(app.id, &app_path).await {
                Ok(_) => {
                    info!(
                        "Updated app for app_path '{}' to '{}'",
                        app.app_name.blue(),
                        app_path.magenta()
                    );
                }
                Err(error) => {
                    error!(
                        "Error updating app_path for '{}': {}",
                        app.app_name.blue(),
                        error
                    );
                }
            }
        }
    }
}

pub async fn update_app(app_name: Option<String>) {
    match app_name {
        Some(app_name) => match db::get_app(&app_name).await {
            Ok(app) => {
                update_app_path_for_list(vec![app]).await;
            }
            Err(e) => {
                error!(
                    "Failed to find app '{}', unable to do update: {:?}",
                    app_name, e
                );
            }
        },
        None => {
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(
                    "Do you want to update the app path for all apps? This may take a while.",
                )
                .interact()
                .unwrap()
            {
                match db::get_apps().await {
                    Ok(apps) => {
                        update_app_path_for_list(apps).await;
                    }
                    Err(e) => {
                        error!("Failed to get apps, unable to do update: {:?}", e);
                    }
                }
            }
        }
    }
}

pub async fn list_app(app_name: Option<String>) {
    match app_name {
        Some(app_name) => match db::get_app(&app_name).await {
            Ok(app) => {
                info!("{}", tabled::Table::new(vec![app]).with(Style::modern()));
            }
            Err(e) => {
                error!(
                    "Failed to find app '{}', unable to do list: {:?}",
                    app_name, e
                );
            }
        },
        None => match db::get_apps().await {
            Ok(apps) => {
                info!(
                    "{}\n{}",
                    "App Listing".yellow(),
                    tabled::Table::new(apps)
                        .with(Modify::new(Rows::new(1..)).with(Width::wrap(30).keep_words()))
                        .with(Style::modern())
                );
            }
            Err(e) => {
                error!("Failed to get apps, unable to do list: {:?}", e);
            }
        },
    }
}

pub fn reset() {
    // Prompt the user for confirmation to delete the file
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to reset the database? All data will be deleted.")
        .interact()
        .unwrap()
    {
        match db::reset_db() {
            Ok(_) => info!("Database reset."),
            Err(e) => error!("Failed to reset database: {:?}", e),
        }
    } else {
        info!("Reset not confirmed.");
    }
}

pub fn testings() {
    // Could be any code calls, my WIP section
    info!("Testing!");
}

async fn open_process(app: db::App, app_path: &str) {
    #[cfg(target_os = "macos")]
    let result = Command::new("open").arg(app_path);
    #[cfg(target_os = "windows")]
    let mut cmd = Command::new(app_path);

    let mut flattened_params = String::new();
    if Some(app.params.clone()).is_some() {
        let args = paths::parse_arguments(&app.params.clone().unwrap());
        flattened_params = format!(" with params '{}'", args.join(" ").magenta());

        for arg in args {
            cmd.arg(arg);
        }
    }
    let result = cmd.spawn();

    match result {
        Ok(_) => {
            info!(
                "Opened '{}' in '{}'{}",
                &app.app_name.blue(),
                &app_path.magenta(),
                flattened_params
            );

            match db::update_last_opened(app.id).await {
                Ok(_) => {
                    debug!("Updated last_opened for app '{}'", app.app_name.blue());
                }
                Err(error) => {
                    error!(
                        "Error updating last_opened for app '{}': {}",
                        app.app_name.blue(),
                        error
                    );
                }
            }
        }
        Err(e) => error!("Failed to open '{}': {:?}", &app.app_name, e),
    }
}
