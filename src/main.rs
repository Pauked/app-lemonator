use clap::Parser;
use colored::Colorize;

mod cli;
mod db;
mod paths;
mod runner;

/*
    TODO: Save last run path to speed up app launch. Either tag on apps table or create new table.
    TODO: Add method to update last run path periodically (or after each run?).
    TODO: Add Tokio tracing for logging.
    TODO: Improve console output. Make consistent. Make tidy. Show app name!
    TODO: Change name to something less naff? Stream Deck Lemonator?
    TODO: Stats/Usage table. What has been run and when.
    TODO: Add "shortcut" option. Run app with no search term. May need to tweak input arguments.
    TODO: Add a reset option. Nukes the DB and recreates it.
    TODO: Add params field to apps table. Allows for passing of params to exe.
*/

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();
    //println!("{:?}", args);

    db::create_db().await;

    match args.action {
        cli::Action::Open { app_name } => {
            match db::get_app(&app_name).await {
                Ok(app) => {
                    runner::run_app(app);
                }
                Err(_) => {
                    eprintln!("App '{}' not found", app_name.red());
                }
            }
        }
        cli::Action::Add {
            app_name,
            exe_name,
            search_term,
            search_method,
        } => {
            db::add_app(&app_name, &exe_name, &search_term, search_method).await;
        }
        cli::Action::Delete { app_name } => {
            db::delete_app(&app_name).await;
        }
        cli::Action::List {} => {
            let apps = db::get_apps().await;
            println!("{}", "App Listing".yellow());
            for app in apps {
                println!("App_name '{}', exe_name '{}', searchterm '{}', searchmethod '{}'", app.app_name.green(), app.exe_name, app.search_term, app.search_method);
            }
        },
        cli::Action::Testings {} => {
            println!("Testing!");
        }
    }
}
