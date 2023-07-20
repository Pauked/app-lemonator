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
        cli::Action::Testings {} => {
            println!("Testing!");
        }
    }

    /*
    let matches = cli::get_cli().get_matches();

    db::create_db().await;

    match matches.subcommand() {
        Some((cli::ACTION_OPEN, sub_matches)) => {
            let action = cli::get_action_open(sub_matches);
            let app = db::get_app(&action.app).await;
            runner::run_app(app);
        }
        Some((cli::ACTION_ADD, sub_matches)) => {
            let action = cli::get_action_add(sub_matches);

            db::add_app(
                &action.app,
                &action.exe_name,
                &action.search_term,
                &action.search_method,
            )
            .await;
        }
        Some((cli::ACTION_DELETE, sub_matches)) => {
            let action = cli::get_action_delete(sub_matches);
            db::delete_app(&action.app).await;
        }
        Some((cli::ACTION_TESTINGS, _sub_matches)) => {
            println!("Testing!");
            /*
            let root_folder = runner::get_local_app_data_folder();
            //let root_folder = r#"C:\Users\paul\AppData\Local\JetBrains\Toolbox\apps\Rider\ch-0"#;

            println!("Root folder: {}", root_folder);
            let result = runner::find_file_in_folders(&root_folder, "rider64.exe");
            match result {
                Ok(_) => println!("Found file"),
                Err(e) => eprintln!("Failed to find file: {:?}", e),
            }

            */
        }
        _ => unreachable!(),
    }
    */
}
