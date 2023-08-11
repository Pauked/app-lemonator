use std::{fs::File, io::Write, process::Command};

use color_eyre::{eyre, eyre::Context, Report, Result};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm};
use log::{debug, error, info};
use tabled::{
    builder::Builder,
    settings::{object::Rows, Modify, Style, Width},
};

use crate::{
    cli,
    db::{self},
    finder, paths,
};

pub async fn create_db() {
    db::create_db().await;
}

pub async fn open_app(app_name: &str) -> Result<String, Report> {
    let app = db::get_app(app_name).await.wrap_err(format!(
        "Failed to find app in database '{}'",
        app_name.blue()
    ))?;
    let app_path = finder::get_app_path(app.clone(), app.app_path.clone()).await?;
    open_process(app.clone(), &app_path).await
    // let open_result = open_process(app.clone(), &app_path).await?;

    // FIXME: update app path (maybe move to get_app_path instead of open...)
    // match db::update_app_path(app.id, &app_path).await {
    //     Ok(_) => {
    //         debug!(
    //             "Updated app for app_path '{}' to '{}'",
    //             app.app_name.blue(),
    //             app_path.magenta()
    //         );
    //     }
    //     Err(error) => {
    //         error!(
    //             "Error updating app_path for '{}': {}",
    //             app.app_name.blue(),
    //             error
    //         );
    //     }
    // }

    // Ok(open_result)
    //db::update_app_path(app.id, &app_path).await?;
    //Ok(format!("Successfully opened '{}'", app_name.blue()))
    /*
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
    error!("Failed to find app '{}': {:?}", app_name.blue(), e);
    */
}

pub async fn add_app(
    app_name: String,
    exe_name: String,
    params: Option<String>,
    search_term: String,
    search_method: cli::SearchMethod,
) -> Result<String, Report> {
    if (db::get_app(&app_name).await).is_ok() {
        Err(eyre::eyre!(
            "Cannot add app '{}' as it already exists. Full details are:",
            app_name.blue()
        ))?;
        error!(
            "Cannot add app '{}' as it already exists. Full details are:",
            app_name.blue()
        );
        // FIXME: list_app
        //list_app(Some(app_name), false).await;
        //return;
    }

    let add_result = db::add_app(&app_name, &exe_name, &params, &search_term, &search_method)
        .await
        .wrap_err(format!("Error adding app '{}'", app_name.blue()))?;

    let param_info = if let Some(unwrapped_params) = params {
        format!(" Params '{}'", unwrapped_params.magenta())
    } else {
        String::new()
    };

    Ok(format!(
        "Added App Name '{}', Exe Name '{}', Search Term '{}', Search Method '{}'{}",
        app_name.blue(),
        exe_name.magenta(),
        search_term.magenta(),
        search_method,
        param_info
    ))

    /*
    match db::add_app(&app_name, &exe_name, &params, &search_term, &search_method).await {
        Ok(_) => {
            let param_info = if let Some(unwrapped_params) = params {
                format!(" Params '{}'", unwrapped_params.magenta())
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
    */
}

pub async fn delete_app(app_name: &str) -> Result<String, Report> {
    todo!("Delete not implemented yet.")
    /*
    match db::delete_app(app_name).await {
        Ok(_) => {
            info!("Deleted app '{}'", app_name.blue());
        }
        Err(error) => {
            error!("Error deleting app '{}': {}", app_name, error);
        }
    }
    */
}

async fn update_app_path_for_list(apps: Vec<db::App>) {
    todo!("Update not implemented yet.")
    /*
    for app in apps {
        let app_path = finder::get_app_path(app.clone(), None);
        if !app_path.is_empty() {
            match db::update_app_path(app.id, &app_path).await {
                Ok(_) => {
                    info!(
                        "Updated app for app path '{}' to '{}'",
                        app.app_name.blue(),
                        app_path.magenta()
                    );
                }
                Err(error) => {
                    error!(
                        "Error updating app path for '{}': {}",
                        app.app_name.blue(),
                        error
                    );
                }
            }
        }
    }
    */
}

pub async fn update_app(app_name: Option<String>) -> Result<String, Report> {
    todo!("Update not implemented yet.")
    /*
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
    */
}

pub async fn list_app(app_name: Option<String>, full: bool) -> Result<String, Report> {
    todo!("List not implemented yet.")
    /*
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
                if apps.is_empty() {
                    info!("No apps to list.");
                    return;
                }

                let table = match full {
                    true => tabled::Table::new(apps)
                        .with(Modify::new(Rows::new(1..)).with(Width::wrap(30).keep_words()))
                        .with(Style::modern())
                        .to_string(),
                    _ => {
                        let mut builder = Builder::default();
                        builder.set_header(["App Name", "App Path", "Last Opened", "Last Updated"]);
                        for app in apps {
                            builder.push_record([
                                app.app_name,
                                db::display_option_string(&app.app_path),
                                db::display_option_utc_datetime_to_local(&app.last_opened),
                                db::display_option_utc_datetime_to_local(&app.last_updated),
                            ]);
                        }
                        let mut table = builder.build();
                        table
                            .with(Modify::new(Rows::new(1..)).with(Width::wrap(50).keep_words()))
                            .with(Style::modern())
                            .to_string()
                    }
                };

                info!("{}\n{}", "App Listing".yellow(), table);
            }
            Err(e) => {
                error!("Failed to get apps, unable to do list: {:?}", e);
            }
        },
    }
    */
}

pub fn reset() -> Result<String, Report> {
    todo!("Reset not implemented yet.")
    /*
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
    */
}

pub fn testings() -> Result<String, Report> {
    // Could be any code calls, my WIP section
    todo!("Testing!");
}

async fn open_process(app: db::App, app_path: &str) -> Result<String, Report> {
    #[cfg(target_os = "macos")]
    let mut cmd = Command::new("open");
    #[cfg(target_os = "macos")]
    cmd.arg(app_path);

    #[cfg(target_os = "windows")]
    let mut cmd = Command::new(app_path);

    // Double check we can see the app before running
    if !paths::check_app_exists(app_path) {
        return Err(eyre::eyre!(
            "App path does not exist for app '{}' and path '{}",
            app.app_name,
            app_path
        ));
    }

    let mut flattened_params = String::new();
    if let Some(app_params) = app.params {
        let args = paths::parse_arguments(&app_params);
        flattened_params = format!(" with params '{}'", args.join(" ").magenta());

        for arg in args {
            cmd.arg(arg);
        }
    }
    cmd.spawn()
        .wrap_err(format!("Failed to open '{}'", &app.app_name))?;

    // FIXME: db::update_last_opened(app.id).await
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

    Ok(format!(
        "Successfully opened '{}' in '{}'{}",
        &app.app_name.blue(),
        &app_path.magenta(),
        flattened_params
    ))
}

pub async fn export(file_out: Option<String>) -> Result<String, Report> {
    todo!("Export not implemented yet.")
    /*
    match db::get_apps().await {
        Ok(apps) => {
            let file_checked: String = match file_out {
                Some(file) => file,
                None => String::new(),
            };
            let output_file = paths::get_export_file_name(
                &file_checked,
                dirs::document_dir().unwrap(),
                &paths::get_unique_export_file_name(),
            );

            if paths::file_exists(&output_file)
                && !Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(format!(
                        "Export file '{}' already exists, do you want to overwrite it?",
                        output_file
                    ))
                    .interact()
                    .unwrap()
            {
                info!("Export cancelled.");
                return;
            }

            match serde_json::to_string(&apps) {
                Ok(serialized) => match File::create(&output_file) {
                    Ok(mut file) => match file.write_all(serialized.as_bytes()) {
                        Ok(_) => {
                            info!("Exported apps to '{}'", output_file);
                        }
                        Err(error) => {
                            error!("Error writing to file to export: {}", error);
                        }
                    },
                    Err(error) => {
                        error!("Error creating file to export: {}", error);
                    }
                },
                Err(error) => {
                    error!("Error serializing apps to export: {}", error);
                }
            }
        }
        Err(error) => {
            error!("Error getting apps to export: {}", error);
        }
    }
    */
}

pub async fn import(file_in: String) -> Result<String, Report> {
    todo!("Import not implemented yet.")
    /*
    match File::open(&file_in) {
        Ok(file) => {
            let deserialized: Vec<db::App> = match serde_json::from_reader(file) {
                Ok(deserialized) => deserialized,
                Err(error) => {
                    error!(
                        "Error deserializing file '{}' to import: {}",
                        file_in, error
                    );
                    return;
                }
            };

            let mut imported_count = 0;

            for app in deserialized {
                let search_method: cli::SearchMethod = app.search_method.parse().unwrap();

                if (db::get_app(&app.app_name).await).is_ok() {
                    info!("App '{}' already exists, skipping.", app.app_name.blue());
                } else {
                    match db::add_app(
                        &app.app_name,
                        &app.exe_name,
                        &app.params,
                        &app.search_term,
                        &search_method,
                    )
                    .await
                    {
                        Ok(_) => {
                            info!("Imported app '{}'", app.app_name.blue());
                            imported_count += 1;
                        }
                        Err(error) => {
                            error!("Error importing app '{}': {}", app.app_name.blue(), error);
                        }
                    }
                }
            }

            if imported_count == 0 {
                info!("No apps imported from '{}'", file_in);
                return;
            }

            info!(
                "Successfully imported {} apps from '{}'",
                imported_count, file_in
            );
        }
        Err(error) => {
            error!("Error opening file '{}' to import: {}", file_in, error);
        }
    }
    */
}
