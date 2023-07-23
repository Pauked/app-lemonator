use chrono::{NaiveDate, Utc};
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
    pub id: i32,
    pub app_name: String,
    pub exe_name: String,
    pub search_term: String,
    pub search_method: String,
    pub found_path: Option<String>,
    pub last_run: Option<NaiveDate>,
    pub last_update: Option<NaiveDate>,
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

pub async fn add_app(
    app_name: &str,
    exe_name: &str,
    search_term: &str,
    search_method: SearchMethod,
) {
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
                app_name.green(),
                exe_name,
                search_term,
                search_method
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

pub async fn update_app_found_path(app: &str, id: i32, found_path: &str) {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let update_result = sqlx::query(
        "UPDATE apps SET found_path = $1, last_update = $2 WHERE id=$3 COLLATE NOCASE",
    )
    .bind(found_path)
    .bind(Utc::now().date_naive())
    .bind(id)
    .execute(&db)
    .await;

    match update_result {
        Ok(_) => {
            println!("Updated app '{}'", app.blue());
        }
        Err(error) => {
            panic!("error: {}", error);
        }
    }
}

pub async fn update_last_run(app: &str, id: i32) {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let update_result = sqlx::query("UPDATE apps SET last_run = $1 WHERE id=$2 COLLATE NOCASE")
        .bind(Utc::now())
        .bind(id)
        .execute(&db)
        .await;

    match update_result {
        Ok(_) => {
            println!("Updated app '{}'", app.blue());
        }
        Err(error) => {
            panic!("error: {}", error);
        }
    }
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
