use color_eyre::{eyre::Context, owo_colors::OwoColorize, Report, Result};
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm};
use log::{debug, error, info};
use tabled::{
    builder::Builder,
    settings::{object::Rows, Modify, Style, Width},
};

use crate::{
    cli, data,
    db::{self},
    finder, paths, runner,
};

pub async fn create_db() -> Result<bool, Report> {
    db::create_db().await
}

pub async fn open_app(app_name: &str) -> Result<String, Report> {
    let app = db::get_app(app_name)
        .await
        .wrap_err("Unable to open app".to_string())?;
    let app_path = finder::get_app_path(app.clone(), app.app_path.clone())
        .await
        .wrap_err("Unable to open app".to_string())?;

    if paths::check_app_exists(&app_path) {
        // FIXME Move update app path to here? Should Database code be in here?
        db::update_app_path(app.id, &app_path)
            .await
            .wrap_err(format!(
                "Error updating app_path for '{}'",
                app.app_name.blue(),
            ))?;
        debug!(
            "Updated app for app_path '{}' to '{}'",
            app.app_name.blue(),
            app_path.magenta()
        );
    }

    let open_result = runner::open_process(app.clone(), &app_path)
        .await
        .wrap_err("Unable to open app".to_string())?;

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

    Ok(open_result)
}

pub async fn add_app(
    app_name: String,
    exe_name: String,
    params: Option<String>,
    search_term: String,
    search_method: cli::SearchMethod,
) -> Result<String, Report> {
    // If the app already exists, this is "OK". Report back the details of what is stored.
    if (db::get_app(&app_name).await).is_ok() {
        let listing = match list_app(Some(app_name.clone()), false).await {
            Ok(output) => output,
            Err(_) => "Unable to get listing".to_string(),
        };

        return Ok(format!(
            "Cannot add app '{}' as it already exists. Current details are:\n{}",
            app_name.blue(),
            listing
        ));
    }

    db::add_app(&app_name, &exe_name, &params, &search_term, &search_method).await?;

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
}

pub async fn delete_app(app_name: &str) -> Result<String, Report> {
    db::delete_app(app_name).await?;
    Ok(format!("Successfully deleted app '{}'", app_name.blue()))
}

async fn update_app_path_for_list(apps: Vec<data::App>) -> Result<String, Report> {
    let (success, failed) = {
        let mut success = 0;
        let mut failed = 0;

        for app in &apps {
            // I want this process to continue to run, even if one or more apps fail to update
            match finder::get_app_path(app.clone(), None).await {
                Ok(app_path) => match db::update_app_path(app.id, &app_path).await {
                    Ok(_) => {
                        info!(
                            "Updated app for app path '{}' to '{}'",
                            app.app_name.blue(),
                            app_path.magenta()
                        );
                        success += 1;
                    }
                    Err(error) => {
                        error!(
                            "Error updating app path for '{}': {:?}",
                            app.app_name, error
                        );
                        failed += 1;
                    }
                },
                Err(error) => {
                    error!("Error getting app path for '{}': {:?}", app.app_name, error);
                    failed += 1;
                }
            }
        }

        (success, failed)
    };

    let message = {
        if success == apps.len() {
            "All apps updated successfully".green().to_string()
        } else {
            format!(
                "{}\n{}",
                format!("Successfully updated {} apps", success).green(),
                format!("Failed to update {} apps", failed).red()
            )
        }
    };
    Ok(message)
}

pub async fn update_app(app_name: Option<String>) -> Result<String, Report> {
    let apps = match app_name {
        Some(app_name) => {
            vec![db::get_app(&app_name)
                .await
                .wrap_err("Unable to update app path for selected app".to_string())?]
        }
        None => {
            if !Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(
                    "Do you want to update the app path for all apps? This may take a while.",
                )
                .interact()
                .unwrap()
            {
                return Ok("Aborted app path update".to_string());
            }

            db::get_apps()
                .await
                .wrap_err("Unable to update app path for all apps".to_string())?
        }
    };

    update_app_path_for_list(apps)
        .await
        .wrap_err("Unable to update app path")
}

pub async fn list_app(app_name: Option<String>, full: bool) -> Result<String, Report> {
    match app_name {
        Some(app_name) => {
            let app = db::get_app(&app_name)
                .await
                .wrap_err("Unable to create app listing".to_string())?;
            Ok(format!(
                "{}",
                tabled::Table::new(vec![app]).with(Style::modern())
            ))
        }
        None => {
            let apps = db::get_apps()
                .await
                .wrap_err("Unable to create app listing".to_string())?;

            if apps.is_empty() {
                return Ok("No apps to list.".to_string());
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
                            data::display_option_string(&app.app_path),
                            data::display_option_utc_datetime_to_local(&app.last_opened),
                            data::display_option_utc_datetime_to_local(&app.last_updated),
                        ]);
                    }
                    let mut table = builder.build();
                    table
                        .with(Modify::new(Rows::new(1..)).with(Width::wrap(50).keep_words()))
                        .with(Style::modern())
                        .to_string()
                }
            };

            Ok(format!("{}\n{}", "App Listing".blue(), table))
        }
    }
}

pub fn reset() -> Result<String, Report> {
    // Prompt the user for confirmation to delete the file
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to reset the database? All data will be deleted.")
        .interact()
        .unwrap()
    {
        db::reset_db().wrap_err("Failed to reset database.")?;
        Ok("Database reset.".to_string())
    } else {
        Ok("Database reset not confirmed.".to_string())
    }
}

pub fn testings() -> Result<String, Report> {
    // Could be any code calls, my WIP section
    todo!("Testing!");
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
