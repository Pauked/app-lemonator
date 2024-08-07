use std::{fs::File, io::Write};

use color_eyre::{eyre::Context, owo_colors::OwoColorize, Report, Result};
use dialoguer::{theme::ColorfulTheme, Confirm};
use log::{debug, error, info};
use tabled::{
    builder::Builder,
    settings::{object::Rows, Modify, Style, Width},
};

use crate::{
    data,
    db::{self},
    finder, paths, runner,
};

pub enum ListType {
    Full,
    Summary,
}

pub fn create_db() -> Result<bool, Report> {
    db::create_db()
}

pub fn open_app(app_name: &str) -> Result<String, Report> {
    let app = db::get_app(app_name).wrap_err("Unable to open app".to_string())?;

    let current_app_file_version = if let Some(app_path) = &app.app_path {
        Some(data::FileVersion::new(
            app.app_description.clone().unwrap_or_default(),
            app_path.to_string(),
            app.app_version.clone().unwrap_or_default(),
        ))
    } else {
        None
    };

    let update_app_file_version =
        finder::get_app_file_version(app.clone(), current_app_file_version)
            .wrap_err("Unable to open app".to_string())?;

    // Just getting the latest app path/file version regardless is not a quick process
    // so lets check if the app path exists first. If it doesn't, then we'll get the latest info.
    let update_app_file_version = if !paths::check_app_exists(&update_app_file_version.path) {
        finder::get_app_file_version(app.clone(), None)
            .wrap_err("Unable to open app".to_string())?
    } else {
        update_app_file_version
    };

    // Check the app exists, update last run date/time and file version information
    if paths::check_app_exists(&update_app_file_version.path) {
        db::update_app_file_version(app.id, &update_app_file_version).wrap_err(format!(
            "Error updating app_path for '{}'",
            app.app_name.blue(),
        ))?;
        debug!(
            "Updated app '{}' app_path from '{}' to '{}'",
            app.app_name.blue(),
            app.app_path.clone().unwrap_or_default().magenta(),
            update_app_file_version.path.magenta()
        );
    }

    let open_result = runner::open_process(app.clone(), &update_app_file_version.path)
        .wrap_err("Unable to open app".to_string())?;

    // FIXME: db::update_last_opened(app.id).await
    match db::update_last_opened(app.id) {
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

pub fn add_app(
    app_name: String,
    exe_name: String,
    params: Option<String>,
    search_term: String,
    search_method: data::SearchMethod,
    operating_system: data::OperatingSystem,
) -> Result<String, Report> {
    // If the app already exists, this is "OK". Report back the details of what is stored.
    if (db::get_app(&app_name)).is_ok() {
        let listing = match list_app(Some(app_name.clone()), ListType::Summary) {
            Ok(output) => output,
            Err(_) => "Unable to get listing".to_string(),
        };

        return Ok(format!(
            "Cannot add app '{}' as it already exists. Current details are:\n{}",
            app_name.blue(),
            listing
        ));
    }

    let new_app = data::App::new(
        app_name,
        exe_name,
        params,
        search_term,
        search_method,
        operating_system,
    );

    if let Err(error) = new_app.validate() {
        return Err(eyre::eyre!(
            "Error adding app, validation error - {:?}",
            error
        ));
    }

    db::add_app(&new_app).wrap_err("Error adding app".to_string())?;

    let app = db::get_app(&new_app.app_name)
        .wrap_err("Error adding app, error retrieving details after save")?;

    Ok(format!("Successfully added {}", app.to_description()))
}

pub fn edit_app(
    lookup_app_name: String,
    app_name: Option<String>,
    exe_name: Option<String>,
    params: Option<String>,
    search_term: Option<String>,
    search_method: Option<data::SearchMethod>,
) -> Result<String, Report> {
    let mut app = db::get_app(&lookup_app_name).wrap_err("Unable to edit app".to_string())?;

    debug!(
        "Before editing - lookup app name '{}', app record '{:?}'",
        lookup_app_name, app
    );
    app.app_name = app_name.unwrap_or(app.app_name);
    app.exe_name = exe_name.unwrap_or(app.exe_name);
    if let Some(params) = params {
        app.params = Some(params);
    }
    app.search_term = search_term.unwrap_or(app.search_term);
    app.search_method = search_method.unwrap_or(app.search_method);
    debug!(
        "After editing - lookup app name '{}', app record '{:?}'",
        lookup_app_name, app
    );

    if let Err(error) = app.validate() {
        return Err(eyre::eyre!(
            "Error editing app, validation error - {:?}",
            error
        ));
    }

    db::edit_app(&lookup_app_name, &app).wrap_err("Unable to edit app".to_string())?;

    Ok(format!("Successfully edited {}", app.to_description()))
}

pub fn delete_app(app_name: &str) -> Result<String, Report> {
    if (db::get_app(app_name)).is_err() {
        return Ok(format!(
            "App '{}' does not exist, so cannot be deleted",
            app_name.blue()
        ));
    }
    db::delete_app(app_name)?;
    Ok(format!("Successfully deleted app '{}'", app_name.blue()))
}

fn update_app_file_version_for_list(apps: Vec<data::App>) -> Result<String, Report> {
    let (success, failed) = {
        let mut success = 0;
        let mut failed = 0;

        for app in &apps {
            // I want this process to continue to run, even if one or more apps fail to update
            match finder::get_app_file_version(app.clone(), None) {
                Ok(app_file_version) => {
                    match db::update_app_file_version(app.id, &app_file_version) {
                        Ok(_) => {
                            info!(
                                "Updated app '{}' app_path from '{}' to '{}'",
                                app.app_name.blue(),
                                app.app_path.clone().unwrap_or_default().magenta(),
                                app_file_version.path.magenta()
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
                    }
                }
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
            if success == 1 {
                "Successfully updated app".green().to_string()
            } else {
                "Successfully updated all apps".green().to_string()
            }
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

pub fn update_app(app_name: Option<String>, force: bool) -> Result<String, Report> {
    let apps = match app_name {
        Some(app_name) => {
            vec![db::get_app(&app_name)
                .wrap_err("Unable to update app path for selected app".to_string())?]
        }
        None => {
            if !force
                && !Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(
                        "Do you want to update the app path for all apps? This may take a while.",
                    )
                    .interact()
                    .unwrap()
            {
                return Ok("Aborted app path update".to_string());
            }

            db::get_apps().wrap_err("Unable to update app path for all apps".to_string())?
        }
    };

    update_app_file_version_for_list(apps).wrap_err("Unable to update app path")
}

pub fn list_app(app_name: Option<String>, list_type: ListType) -> Result<String, Report> {
    match app_name {
        Some(app_name) => {
            let app =
                db::get_app(&app_name).wrap_err("Unable to generate app listing".to_string())?;
            Ok(format!(
                "\n{}",
                tabled::Table::new(vec![app]).with(Style::modern())
            ))
        }
        None => {
            let apps = db::get_apps().wrap_err("Unable to generate app listing".to_string())?;

            if apps.is_empty() {
                return Ok("No apps to list.".to_string());
            }

            let table = match list_type {
                ListType::Full => tabled::Table::new(apps)
                    .with(Modify::new(Rows::new(1..)).with(Width::wrap(30).keep_words()))
                    .with(Style::modern())
                    .to_string(),
                ListType::Summary => {
                    let mut builder = Builder::default();
                    builder.push_record(["App Name", "App Path", "Last Opened", "Last Updated"]);
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

            Ok(format!("\n{}", table))
        }
    }
}

pub fn reset(force: bool) -> Result<String, Report> {
    if !db::database_exists() {
        return Ok("Database does not exist, nothing to reset.".to_string());
    }

    // Prompt the user for confirmation to delete the file
    if force
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to reset the database? All data will be deleted.")
            .interact()
            .unwrap()
    {
        db::reset_db().wrap_err("Failed to reset database.")?;
        Ok("Successfully reset database.".green().to_string())
    } else {
        Ok("Database reset not confirmed.".to_string())
    }
}

pub fn export(file_out: Option<String>, force: bool) -> Result<String, Report> {
    let apps = db::get_apps().wrap_err("Unable to export".to_string())?;

    let file_checked: String = file_out.unwrap_or_default();
    let output_file = paths::get_export_file_name(
        &file_checked,
        dirs::document_dir().unwrap(),
        &paths::get_unique_export_file_name(),
    );

    if paths::file_exists(&output_file)
        && !force
        && !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "Export file '{}' already exists, do you want to overwrite it?",
                output_file
            ))
            .interact()
            .unwrap()
    {
        return Ok("Aborted export".to_string());
    }

    let serialized = serde_json::to_string(&apps)
        .wrap_err("Unable to export, error serializing apps to export:".to_string())?;
    let mut file = File::create(&output_file).wrap_err(format!(
        "Unable to export, error creating file to export: '{}'",
        output_file
    ))?;
    file.write_all(serialized.as_bytes()).wrap_err(format!(
        "Unable to export, error writing to file to export: '{}'",
        output_file
    ))?;

    Ok(format!(
        "{}",
        format!("Successfully exported apps to '{}'", output_file).green()
    ))
}

pub fn import(file_in: String) -> Result<String, Report> {
    let file = File::open(&file_in).wrap_err(format!(
        "Unable to import, error opening file '{}'",
        file_in
    ))?;
    let deserialized: Vec<data::App> = serde_json::from_reader(file).wrap_err(format!(
        "Unable to import, error deserializing file '{}'",
        file_in
    ))?;

    let mut success = 0;
    let mut skipped = 0;
    let mut failed = 0;

    for app in &deserialized {
        if let Err(error) = app.validate() {
            error!(
                "{} '{}', {} - {:?}",
                "Unable to import app".red(),
                app.app_name.blue(),
                "validation error".red(),
                error
            );
            failed += 1;
            continue;
        }

        if (db::get_app(&app.app_name)).is_ok() {
            info!("Skipped app '{}', already exists", app.app_name.blue());
            skipped += 1;
        } else {
            match db::add_app(app) {
                Ok(_) => {
                    info!("Successfully added {}", app.to_description());
                    success += 1;
                }
                Err(error) => {
                    error!("Unable to add app '{}': {:?}", app.app_name, error);
                    failed += 1;
                }
            }
        }
    }

    let message = {
        if success == deserialized.len() {
            "Successfully imported all apps".green().to_string()
        } else {
            format!(
                "{}\n{}\n{}",
                format!("Successfully imported {} apps", success).green(),
                format!("Skipped {} apps", skipped).yellow(),
                format!("Failed to import {} apps", failed).red()
            )
        }
    };

    Ok(message)
}
