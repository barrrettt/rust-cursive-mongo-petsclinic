mod database;
mod datamodels;
use database::DB;

use std::error::Error;
use tokio;
//use std::env;

#[tokio::main]
pub async fn connect() -> Result<DB, Box<dyn Error>> {
    // Check connection and if no exist db create one with mocks data
    let db = DB::new().await.unwrap();
    let exist = db.ckeck_databases().await;
    if !exist {
        println!("New database...");
        db.create_db().await;
    }
    
    Ok(db)
}

pub async fn find_customers(db:&DB, query:&str) -> Result<(), Box<dyn Error>> {
    println!("customers");
    Ok(())
}
