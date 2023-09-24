use std::{ffi::OsStr, process::Command};

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

    // Add in additional arguments
    add_arguments_to_command(&mut cmd, app.params);

    // Run the app
    cmd.spawn()
        .wrap_err(format!("Failed to open '{}'", &app.app_name))?;

    Ok(format!(
        "Successfully opened '{}' from '{}'{}",
        &app.app_name.blue(),
        &app_path.magenta(),
        get_display_args(&cmd)
    ))
}

fn get_display_args(cmd: &Command) -> String {
    let cmd_args: Vec<&OsStr> = cmd.get_args().collect();
    let result: String = cmd_args
        .iter()
        .filter_map(|s| s.to_str()) // Convert each &OsStr to Option<&str>
        .collect::<Vec<_>>() // Collect to Vec<&str>
        .join(" ");
    format!(" with params '{}'", result.magenta())
}

fn add_arguments_to_command(cmd: &mut Command, additional_arguments: Option<String>) {
    if let Some(additional_arguments_unwrapped) = additional_arguments {
        let args = paths::parse_arguments(&additional_arguments_unwrapped);
        for arg in args {
            cmd.arg(arg);
        }
    }
}
