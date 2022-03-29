mod database;
mod datamodels;
use database::DB;
use datamodels::customer::Customer;

use std::error::Error;
use tokio;


pub struct DataBase {
    pub runtime:tokio::runtime::Runtime,
    pub db:DB,
}

impl DataBase{

    //TOKIO MAIN
    pub fn connect() -> Result<DataBase, Box<dyn Error>> {
        
        let rt = tokio::runtime::Runtime::new().unwrap();
        
        let db = rt.block_on(async {
            // Check connection and if no exist db create one with mocks data
            let db = DB::new().await.unwrap();
            let exist = db.ckeck_databases().await;
            if !exist {
                println!("New database...");
                db.create_db_mocks().await;
            }
            db
        });

        let database = DataBase { runtime:rt, db };
        Ok(database)
    }
    
    //PUBLIC FUNCTIONS
    pub fn find_customers(&self, query:&str) -> Option<Customer>{
        let result = self.runtime.block_on(async {
            let result = self.db.find_like_name(query).await;
            result
        });
        
        result
    }
}