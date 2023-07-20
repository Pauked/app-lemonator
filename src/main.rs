use clap::Parser;

mod cli;
mod db;
mod paths;
mod runner;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();
    println!("{:?}", args);

    db::create_db().await;

    match args.action {
        cli::Action::Open { app } => {
            let app = db::get_app(&app).await;
            runner::run_app(app);
        }
        cli::Action::Add {
            app,
            exe_name,
            search_term,
            search_method,
        } => {
            db::add_app(&app, &exe_name, &search_term, search_method).await;
        }
        cli::Action::Delete { app } => {
            db::delete_app(&app).await;
        }
        cli::Action::List {} => {
            let apps = db::get_apps().await;
            for app in apps {
                println!("App {}, exe {}, searchterm {}, searchmethod {}", app.app, app.exe_name, app.search_term, app.search_method);
            }
        },
        cli::Action::Testings {} => {
            println!("Testing!");
        }
    }
}
