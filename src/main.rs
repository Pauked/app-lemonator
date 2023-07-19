mod cli;
mod db;
mod runner;

#[tokio::main]
async fn main() {
    // What I'm trying to achieve
    // -Command line params to "add" or "delete" an app
    //   - The "add" puts an entry in a local SQLLite database
    // -Command line params to "open" an app

    // "C:\Users\paul\AppData\Local\JetBrains\Toolbox\apps\Rider\ch-0\223.8617.53\bin\rider64.exe"

    let matches = cli::get_cli().get_matches();

    db::create_db().await;

    match matches.subcommand() {
        Some((cli::ACTION_OPEN, sub_matches)) => {
            let action_open = cli::get_action_open(sub_matches);
            let app = db::get_app(&action_open.app).await;
            runner::run_app(app);
        }
        Some((cli::ACTION_ADD, sub_matches)) => {
            let action_add = cli::get_action_add(sub_matches);

            db::add_app(
                &action_add.app,
                &action_add.exe_name,
                &action_add.search_term,
                &action_add.search_method,
            )
            .await;
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
}
