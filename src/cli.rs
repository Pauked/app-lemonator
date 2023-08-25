use std::env;

use clap::Parser;

use crate::actions;
use crate::constants;
use crate::data;

// https://github.com/clap-rs/clap/blob/master/examples/git-derive.rs
// https://github.com/glotlabs/gdrive/blob/main/src/main.rs
#[derive(Parser, Debug, PartialEq)]
#[clap(about, author, name = constants::CRATE_NAME, version)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Parser, Debug, PartialEq)]
pub enum Action {
    /// Opens an app.
    #[clap(short_flag = 'o', long_flag = "open")]
    Open { app_name: String },

    /// Adds an app to the database.
    #[clap(short_flag = 'a', long_flag = "add")]
    Add {
        /// Nice name for app.
        app_name: String,
        /// Executable to find and run. For Shortcuts, can be full path and exe.
        exe_name: String,
        /// Search text for app.
        search_term: String,
        /// Search method to find app.
        #[clap(value_enum)]
        search_method: data::SearchMethod,
        /// Parameters to pass to app.
        #[arg(long)]
        params: Option<Vec<String>>,
    },

    /// Deletes the app from the database.
    #[clap(short_flag = 'd', long_flag = "delete")]
    Delete { app_name: String },

    /// Update the running folder for selected apps. No app named means all in database.
    #[clap(short_flag = 'u', long_flag = "update")]
    Update {
        /// App name to update.
        app_name: Option<String>,

        /// Force update of all apps in datanase and skip confirmation prompt.
        #[arg(long, default_value = "false")]
        force: bool,
    },

    /// Lists all apps in the database.
    #[clap(short_flag = 'l', long_flag = "list")]
    List {
        /// Individual app name to list.
        app_name: Option<String>,

        /// Show full details of apps.
        #[arg(long, default_value = "false")]
        full: bool,
    },

    /// Resets the database.
    #[clap(short_flag = 'r', long_flag = "reset")]
    Reset {
        /// Force database reset and skip confirmation prompt.
        #[arg(long, default_value = "false")]
        force: bool,
    },

    /// Exports the database to a JSON file.
    #[clap(short_flag = 'e', long_flag = "export")]
    Export {
        /// File name to export to. Can be left blank, app will save to Documents folder.
        file_out: Option<String>,

        /// Force export to overwrite existing file and skip confirmation prompt.
        #[arg(long, default_value = "false")]
        force: bool,
    },

    /// Imports a JSON file to the database. Existing records will be skipped.
    #[clap(short_flag = 'i', long_flag = "import")]
    Import {
        /// File name to import from.
        file_in: String,
    },
}

pub async fn run_cli_action(args: Args) -> Result<String, eyre::Report> {
    // If we are not resetting the database, make sure it exists and is ready to use
    match args.action {
        Action::Reset { force: _ } => {}
        _ => {
            actions::create_db().await?;
        }
    }

    match args.action {
        Action::Open { app_name } => Ok(actions::open_app(&app_name).await?),
        Action::Add {
            app_name,
            exe_name,
            params,
            search_term,
            search_method,
        } => Ok(actions::add_app(
            app_name,
            exe_name,
            params.map(|p| p.join(" ")),
            search_term,
            search_method,
            get_operating_system(),
        )
        .await?),
        Action::Delete { app_name } => Ok(actions::delete_app(&app_name).await?),
        Action::Update { app_name, force } => Ok(actions::update_app(app_name, force).await?),
        Action::List { app_name, full } => {
            let list_type = match full {
                true => actions::ListType::Full,
                false => actions::ListType::Summary,
            };
            Ok(actions::list_app(app_name, list_type).await?)
        }
        Action::Reset { force } => Ok(actions::reset(force).await?),
        Action::Export {
            file_out: file,
            force,
        } => Ok(actions::export(file, force).await?),
        Action::Import { file_in: file } => Ok(actions::import(file).await?),
    }
}

fn get_operating_system() -> data::OperatingSystem {
    match env::consts::OS {
        constants::OS_WINDOWS => data::OperatingSystem::Windows,
        constants::OS_MACOS => data::OperatingSystem::MacOS,
        _ => data::OperatingSystem::Unknown,
    }
}
