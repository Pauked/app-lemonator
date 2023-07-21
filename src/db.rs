use colored::Colorize;
use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    FromRow, Sqlite, SqlitePool,
};

use crate::cli::SearchMethod;

const DB_URL: &str = "sqlite://sqlite.db";

static MIGRATOR: Migrator = sqlx::migrate!(); // this will pick up migrations from the ./migrations directory

#[derive(Clone, FromRow, Debug)]
pub struct App {
    pub app_name: String,
    pub exe_name: String,
    pub search_term: String,
    pub search_method: String,
}

pub async fn create_db() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        //println!("Database already exists");
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let migration_results = MIGRATOR.run(&db).await;
    match migration_results {
        Ok(_) => {
            //println!("Migration success");
        }
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    //println!("migration: {:?}", migration_results);
}

pub async fn add_app(app_name: &str, exe_name: &str, search_term: &str, search_method: SearchMethod) {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    // TODO: Check app doesn't exist already

    let result = sqlx::query(
        "INSERT INTO apps (app_name, exe_name, search_term, search_method) VALUES (?,?,?,?)",
    )
    .bind(app_name)
    .bind(exe_name)
    .bind(search_term)
    .bind(search_method.to_string())
    .execute(&db)
    .await;

    match result {
        Ok(_) => {
            println!(
                "Added appname '{}', exename '{}', searchterm '{}', searchmethod '{}'",
                app_name.green(), exe_name, search_term, search_method
            );
        }
        Err(error) => {
            panic!("error: {}", error);
        }
    }
}

pub async fn get_app(app: &str) -> Result<App, sqlx::Error> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    sqlx::query_as::<_, App>("SELECT * FROM apps WHERE app_name = ? COLLATE NOCASE")
        .bind(app.to_lowercase())
        .fetch_one(&db)
        .await
}

pub async fn delete_app(app: &str) {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let delete_result = sqlx::query("DELETE FROM apps WHERE app_name=$1 COLLATE NOCASE")
        .bind(app.to_lowercase())
        .execute(&db)
        .await;

    match delete_result {
        Ok(_) => {
            println!("Deleted app '{}'", app.blue());
        }
        Err(error) => {
            panic!("error: {}", error);
        }
    }
}

pub async fn get_apps() -> Vec<App> {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let result = sqlx::query_as::<_, App>("SELECT * FROM apps")
        .fetch_all(&db)
        .await;

    match result {
        Ok(apps) => return apps,
        Err(error) => {
            panic!("error: {}", error);
        }
    };
}

