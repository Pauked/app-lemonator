use std::process::Command;

use colored::Colorize;
use eyre::Context;

use crate::{data, paths};

pub async fn open_process(app: data::App, app_path: &str) -> Result<String, eyre::Report> {
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

    Ok(format!(
        "Successfully opened '{}' in '{}'{}",
        &app.app_name.blue(),
        &app_path.magenta(),
        flattened_params
    ))
}
