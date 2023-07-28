use std::process::Command;

use crate::{db, finder};

pub async fn run_app(app: db::App) {
    let found_path = finder::get_app_path(app.clone());

    if !found_path.is_empty() {
        return;
    }

    open_process(app.clone(), &found_path).await;
    db::update_app_found_path(&app.app_name, app.id, &found_path).await;
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
