use clap::{arg, Command};

mod db;
mod runner;

fn cli() -> Command {
    Command::new("win-app-runner")
        .about("A wrapper for running difficult to find Windows Store apps")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("open")
                .about("open an app")
                .arg(arg!(<APP> "The app to open"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("add")
                .about("add an app to the database")
                .arg(arg!(<APP> "Nice name for app"))
                .arg(arg!(<EXE_NAME> "Exe to run"))
                .arg(arg!(<SEARCH_TERM> "Text to search for app on"))
                .arg(arg!(<SEARCH_METHOD> "Method to search against"))
                .arg_required_else_help(true),
        )
}

#[tokio::main]
async fn main() {
    // What I'm trying to achieve
    // -Command line params to "add" or "delete" an app
    //   - The "add" puts an entry in a local SQLLite database
    // -Command line params to "open" an app

    let matches = cli().get_matches();

    db::create_db().await;

    match matches.subcommand() {
        Some(("open", sub_matches)) => {
            let app_param = sub_matches
                .get_one::<String>("APP")
                .map(|s| s.as_str())
                .expect("required");

            let app = db:: get_app(app_param).await;
            runner::run_app(app);
        }
        Some(("add", sub_matches)) => {
            let app = sub_matches
                .get_one::<String>("APP")
                .map(|s| s.as_str())
                .expect("required");
            let exe_name = sub_matches
                .get_one::<String>("EXE_NAME")
                .map(|s| s.as_str())
                .expect("required");
            let search_term = sub_matches
                .get_one::<String>("SEARCH_TERM")
                .map(|s| s.as_str())
                .expect("required");
            let search_method = sub_matches
                .get_one::<String>("SEARCH_METHOD")
                .map(|s| s.as_str())
                .expect("required");

            db::add_app(app, exe_name, search_term, search_method).await;

            println!(
                "Adding app {} with exename {} with searchterm {} using searchmethod {}",
                app, exe_name, search_term, search_method
            );
        }
        _ => unreachable!(),
    }
}

