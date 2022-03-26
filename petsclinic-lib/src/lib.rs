
use std::error::Error;
use tokio;
mod database;
use database::DB;
//use std::env;

#[tokio::main]
pub async fn connect() -> Result<(), Box<dyn Error>> {
    // Check connection
    let db = DB::new().await.unwrap();
    db.print_databases().await;

    Ok(())
}
