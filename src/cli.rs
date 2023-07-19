use clap::{arg, ArgMatches, Command};

pub const ACTION_OPEN: &str = "open";
pub const ACTION_ADD: &str = "add";
pub const ACTION_DELETE: &str = "delete";
pub const ACTION_TESTINGS: &str = "testings";

pub const PARAM_APP: &str = "APP";
pub const PARAM_EXE_NAME: &str = "EXE_NAME";
pub const PARAM_SEARCH_TERM: &str = "SEARCH_TERM";
pub const PARAM_SEARCH_METHOD: &str = "SEARCH_METHOD";

pub const MSG_REQUIRED: &str = "This is a required parameter";

pub struct ActionAdd {
    pub app: String,
    pub exe_name: String,
    pub search_term: String,
    pub search_method: String,
}

pub struct ActionOpen {
    pub app: String,
}

// TODO: Add a "delete" action

pub fn get_cli() -> Command {
    Command::new("win-app-runner")
        .about("A wrapper for running difficult to find Windows Store apps")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new(ACTION_OPEN)
                .about("open an app")
                .arg(arg!(<APP> "The app to open"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new(ACTION_ADD)
                .about("add an app to the database")
                .arg(arg!(<APP> "Nice name for app"))
                .arg(arg!(<EXE_NAME> "Exe to run"))
                .arg(arg!(<SEARCH_TERM> "Text to search for app on"))
                .arg(arg!(<SEARCH_METHOD> "Method to search against"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new(ACTION_DELETE)
                .about("delete an app from the database")
                .arg(arg!(<APP> "The app to delete"))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new(ACTION_TESTINGS))
}

pub fn get_action_open(arg_matches: &ArgMatches) -> ActionOpen {
    let app_param = arg_matches
        .get_one::<String>(PARAM_APP)
        .map(|s| s.as_str())
        .expect(MSG_REQUIRED);

    ActionOpen {
        app: app_param.to_owned(),
    }
}

pub fn get_action_add(arg_matches: &ArgMatches) -> ActionAdd {
    let app = arg_matches
        .get_one::<String>(PARAM_APP)
        .map(|s| s.as_str())
        .expect(MSG_REQUIRED);
    let exe_name = arg_matches
        .get_one::<String>(PARAM_EXE_NAME)
        .map(|s| s.as_str())
        .expect(MSG_REQUIRED);
    let search_term = arg_matches
        .get_one::<String>(PARAM_SEARCH_TERM)
        .map(|s| s.as_str())
        .expect(MSG_REQUIRED);
    let search_method = arg_matches
        .get_one::<String>(PARAM_SEARCH_METHOD)
        .map(|s| s.as_str())
        .expect(MSG_REQUIRED);

    ActionAdd {
        app: app.to_owned(),
        exe_name: exe_name.to_owned(),
        search_term: search_term.to_owned(),
        search_method: search_method.to_owned(),
    }
}
