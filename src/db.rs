use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    FromRow, Sqlite, SqlitePool,
};

const DB_URL: &str = "sqlite://sqlite.db";

static MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Clone, FromRow, Debug)]
pub struct App {
    pub app: String,
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
        println!("Database already exists");
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let migration_results = MIGRATOR.run(&db).await;

    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    println!("migration: {:?}", migration_results);
}

pub async fn add_app(app: &str, exe_name: &str, search_term: &str, search_method: &str) {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    // TODO: Check app doesn't exist already

    let result = sqlx::query(
        "INSERT INTO apps (app, exe_name, search_term, search_method) VALUES (?,?,?,?)",
    )
    .bind(app)
    .bind(exe_name)
    .bind(search_term)
    .bind(search_method)
    .execute(&db)
    .await
    .unwrap();

    println!("Query result: {:?}", result);
    println!(
        "Added app {} with exename {} with searchterm {} using searchmethod {}",
        app, exe_name, search_term, search_method
    );
}

pub async fn get_app(app: &str) -> App {
    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let result = sqlx::query_as::<_, App>("SELECT * FROM apps WHERE app = ? COLLATE NOCASE")
        .bind(app.to_lowercase())
        .fetch_one(&db)
        .await
        .unwrap();

    println!("Query result: {:?}", result);

    result
}
