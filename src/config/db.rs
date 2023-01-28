use std::env;

use sea_orm::*;

pub async fn set_up() -> Result<DatabaseConnection, DbErr> {
    let db_connection_string = match env::var("DB_CONNECTION_STRING") {
        Ok(val) => val,
        Err(_e) => "DB_CONNECTION_STRING is undefined".to_string(),
    };

    let db = Database::connect(db_connection_string).await?;

    println!("DB connection has been established...");

    Ok(db)
}
