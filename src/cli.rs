use std::env;

use clap::Parser;

use crate::actions;
use crate::constants;
use crate::data;

#[derive(Parser, Debug, PartialEq)]
#[command(name = constants::CRATE_NAME)]
#[command(author = constants::CRATE_AUTHORS)]
#[command(version = constants::CRATE_VERSION)]
#[command(
    help_template = "{about-section}Version : {version}\nAuthor  : {author} \n\n{usage-heading} {usage} \n\n{all-args} {tab}"
)]
#[command(about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Parser, Debug, PartialEq)]
pub enum Action {
    /// Opens an app.
    #[clap(short_flag = 'o')]
    Open {
        /// Name of app to open.
        app_name: String,
        /// Will always attempt to update the app path before opening. Handy for apps that have regular updates.
        #[arg(long, default_value = "false")]
        always_update: bool,
    },

    /// Adds an app to the database.
    #[clap(short_flag = 'a')]
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

    /// Edits individual properties of an app in the database.
    #[clap(short_flag = 'e')]
    Edit {
        /// Look up on nice name of app to edit.
        #[arg(requires = "edit")]
        lookup_app_name: String,
        /// Nice name for app.
        #[arg(long, group = "edit")]
        app_name: Option<String>,
        /// Executable to find and run. For Shortcuts, can be full path and exe.
        #[arg(long, group = "edit")]
        exe_name: Option<String>,
        /// Search text for app.
        #[arg(long, group = "edit")]
        search_term: Option<String>,
        /// Search method to find app.
        #[clap(value_enum)]
        #[arg(long, group = "edit")]
        search_method: Option<data::SearchMethod>,
        /// Parameters to pass to app.
        #[arg(long, group = "edit")]
        params: Option<Vec<String>>,
    },

    /// Deletes the app from the database.
    #[clap(short_flag = 'd')]
    Delete { app_name: String },

    /// Update the running folder for selected apps. No app named means all in database.
    #[clap(short_flag = 'u')]
    Update {
        /// App name to update.
        app_name: Option<String>,

        /// Force update of all apps in datanase and skip confirmation prompt.
        #[arg(long, default_value = "false")]
        force: bool,
    },

    /// Lists all apps in the database.
    #[clap(short_flag = 'l')]
    List {
        /// Individual app name to list.
        app_name: Option<String>,

        /// Show full details of apps.
        #[arg(long, default_value = "false")]
        full: bool,
    },

    /// Resets the database.
    #[clap(short_flag = 'r')]
    Reset {
        /// Force database reset and skip confirmation prompt.
        #[arg(long, default_value = "false")]
        force: bool,
    },

    /// Exports the database to a JSON file.
    #[clap(short_flag = 'x')]
    Export {
        /// File name to export to. Can be left blank, app will save to Documents folder.
        file_out: Option<String>,

        /// Force export to overwrite existing file and skip confirmation prompt.
        #[arg(long, default_value = "false")]
        force: bool,
    },

    /// Imports a JSON file to the database. Existing records will be skipped.
    #[clap(short_flag = 'i')]
    Import {
        /// File name to import from.
        file_in: String,
    },
}

pub fn run_cli_action(args: Args) -> Result<String, eyre::Report> {
    // If we are not resetting the database, make sure it exists and is ready to use
    match args.action {
        Action::Reset { force: _ } => {}
        _ => {
            actions::create_db()?;
        }
    }

    match args.action {
        Action::Open {
            app_name,
            always_update,
        } => Ok(actions::open_app(&app_name, always_update)?),
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
        )?),
        Action::Edit {
            lookup_app_name,
            app_name,
            exe_name,
            params,
            search_term,
            search_method,
        } => Ok(actions::edit_app(
            lookup_app_name,
            app_name,
            exe_name,
            params.map(|p| p.join(" ")),
            search_term,
            search_method,
        )?),
        Action::Delete { app_name } => Ok(actions::delete_app(&app_name)?),
        Action::Update { app_name, force } => Ok(actions::update_app(app_name, force)?),
        Action::List { app_name, full } => {
            let list_type = match full {
                true => actions::ListType::Full,
                false => actions::ListType::Summary,
            };
            Ok(actions::list_app(app_name, list_type)?)
        }
        Action::Reset { force } => Ok(actions::reset(force)?),
        Action::Export {
            file_out: file,
            force,
        } => Ok(actions::export(file, force)?),
        Action::Import { file_in: file } => Ok(actions::import(file)?),
    }
}

fn get_operating_system() -> data::OperatingSystem {
    match env::consts::OS {
        constants::OS_WINDOWS => data::OperatingSystem::Windows,
        constants::OS_MACOS => data::OperatingSystem::MacOS,
        _ => data::OperatingSystem::Unknown,
    }
}
