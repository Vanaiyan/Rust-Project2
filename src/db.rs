use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");

    
    let full_url = format!("{}/{}", database_url, db_name);
    let db = Database::connect(&full_url).await?;
    Ok(db)
}