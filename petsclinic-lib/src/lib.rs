pub mod datamodels;
mod util;

//use std::time::Duration;

use tokio;
use bson::{Document, doc, Bson, oid::ObjectId, from_document};
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::{
    options::{ResolverConfig, ClientOptions, FindOptions}, 
    Client
};
use util::create_mocks;
use crate::datamodels::{
    customer::Customer,
    pet::Pet
};

//const DEFAULT_URL:&str = "mongodb://admin:admin@localhost:27017";
const DATABASE_NAME:&str = "pet_clinic";
const COLLECTION_CUSTOMER:&str = "customers";
const COLLECTION_PETS:&str = "pets";

#[derive(Debug)]
pub struct DataBase {
    pub runtime:tokio::runtime::Runtime,
    pub client: Client,
}

impl DataBase{
    //TOKIO MAIN
    pub fn connect(mongo_url:&str) -> Result<Option<DataBase>, mongodb::error::Error> {
        
        //tokio runtime
        let runtime = tokio::runtime::Runtime::new().unwrap();
        
        //get client to connect with mongodb
        let client_result = runtime.block_on(async {
            let client_uri = mongo_url;
            let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;
            let client = match Client::with_options(options) {
                Ok(it) => it,
                Err(err) => return Err(err),
            };
            Ok(client)
        });
        
        match client_result {
            Ok(client) => {
                let database = DataBase {runtime, client};
                let mut create_new = false;
                //if empty insert some data
                database.runtime.block_on(async {
                    let ckeck_ok = database.ckeck_databases().await;
                    if !ckeck_ok {
                        println!("New database...");
                        create_new = true;
                    }
                });
                if create_new{
                    database.create_database();
                }
                return Ok(Some(database));
            },
            Err(e) => Err(e),
        }
    }

    //check if exists data
    async fn ckeck_databases(&self) -> bool{
        //println!("Databases:");
        let result = self.client.list_database_names(None, None).await;
        let db_pet_clinic_exists = match result {
            Ok(values)=>{
                let mut exists = false;
                for name in values{
                    println!("- {}", name);
                    if name.eq(DATABASE_NAME) {
                        exists = true;
                    }
                 }
                 exists
            },
            Err(e)=>{
                print!("Error:{}",e);
                false
            }
        };
        db_pet_clinic_exists
    }

    //new Database with minimal mocks
    pub fn create_database(&self){
        self.runtime.block_on(async {
            //2 new customers and pets
            let customer = Customer{
                id:None,
                name:"Javier Fernández Barreiro".to_owned(),
                note:"No note".to_owned(),
                contact:vec![],
                update_time:Utc::now(),
            };

            if let Some(owner) = self.add_customer(customer).await{
                //new pets
                let pet = Pet{
                    id:None,
                    customer_id:owner.id,
                    name:"Lua".to_owned(),
                    note:"".to_owned(),
                    pet_type:"cat".to_owned(),
                    update_time:Utc::now(),
                };
                self.add_pet(pet).await;
            }
            
            let customer = Customer{
                id:None,
                name:"Isiña García Novais".to_owned(),
                note:"No note".to_owned(),
                contact:vec![],
                update_time:Utc::now(),
            };
            
            if let Some(owner) = self.add_customer(customer).await{
                let pet = Pet{
                    id:None,
                    customer_id:owner.id,
                    name:"Xena".to_owned(),
                    note:"".to_owned(),
                    pet_type:"cat".to_owned(),
                    update_time:Utc::now(),
                };
                self.add_pet(pet).await;
            }            
        });
    }

    //new Database with mocks
    pub fn create_db_mocks(&self){
        create_mocks(&self,10000);
    }

    //delete db
    pub fn delete_database(&self) {
        self.runtime.block_on(async {
            //get collection
            let db = self.client.database(DATABASE_NAME);
            db.collection::<Document>(COLLECTION_CUSTOMER).drop(None).await.expect("cant delete customers");
            db.collection::<Document>(COLLECTION_PETS).drop(None).await.expect("cant delete pets");
        });
    }

    //Count customers
    pub fn count_customers(&self) -> u64 {
        let result = self.runtime.block_on(async {
            //get collection
            let db = self.client.database(DATABASE_NAME);
            let customers = db.collection::<Document>(COLLECTION_CUSTOMER);
            //execute query
            if let Ok(some) = customers.estimated_document_count(None).await {
                some
            }else{
                0
            }
           
        });
        result
    }

    //customers
    pub fn find_customers_like_name_sort_list(&self, name:&str) -> Option<(Vec<Customer>, u64)>   {
        let result = self.runtime.block_on(async {
            //get collection
            let db = self.client.database(DATABASE_NAME);
            let customers = db.collection::<Document>(COLLECTION_CUSTOMER);

            // Query the customers in the collection with a filter to find with like.
            let regex = bson::Regex{pattern:name.to_owned(), options:"".to_owned()};
            let filter = doc!{"name":regex};
            //options
            let sort = doc!{"name":1};
            let options = FindOptions::builder()
            //.max_await_time(Duration::from_millis(128))        
            .limit(1000)
            .sort(sort).build();
            
            //execute query
            if let Result::Ok(mut cursor) = customers.find(filter, options).await{

                let mut customers_found: Vec<Customer> = Vec::new();
                
                while let Result::Ok(Some(doc)) = cursor.try_next().await{
                    if let Result::Ok(customer) = from_document(doc){
                        //println!("Doc {:?}",customer);
                        customers_found.push(customer)
                    }
                }

                //count all 
                if let Ok(count) = customers.estimated_document_count(None).await{
                    //result OK
                    return Some((customers_found,count));
                }
            }
            //bad finally
            None
        });
        result
    }

    //CRUD customer 
    pub fn get_customer_by_id(&self, id:&ObjectId) -> Option<Customer> {
        let result = self.runtime.block_on(async {
            //get collection
            let db = self.client.database(DATABASE_NAME);
            let customers = db.collection::<Document>(COLLECTION_CUSTOMER);

            // Query the customers in the collection with a filter to find with like.
            let filter = doc! {"_id":id};
            let options = None;
            //execute query
            if let Ok(some) = customers.find_one(filter, options).await {
                if let Some(d) = some{
                    //deserialize
                    if let Ok(c) = bson::from_bson(Bson::Document(d)) {
                        return Some(c)
                    }
                }
            };
            None
        });
        result
    }

    //CRUD: Customer++
    async fn add_customer(&self, mut customer: Customer) -> Option<Customer> {
        let db = self.client.database(DATABASE_NAME);
        let customers = db.collection::<Document>(COLLECTION_CUSTOMER);
        
        // Convert struct to document
        if let Ok(bson) = bson::to_bson(&customer) {
            let document = bson.as_document().unwrap();
            //inset
            match customers.insert_one(document, None).await {
                Err(_) => {
                    //println!("Error insert {}",e);
                    return None
                }
                Ok(inserted) =>{
                    //println!("+customer:{}",instance.name);
                    customer.id = inserted.inserted_id.as_object_id();
                    return Some(customer)
                },
            };
        };
        None
    }

    //CRUD: Pet++
    async fn add_pet(&self, mut pet: Pet) -> Option<Pet> {
        let db = self.client.database(DATABASE_NAME);
        let pets = db.collection::<Document>(COLLECTION_PETS);
        
        // Convert struct to document
        let serialized = bson::to_bson(&pet);
        if let Ok(bson) = serialized {
            let document = bson.as_document().unwrap();
            //inset
            match pets.insert_one(document, None).await {
                Err(_) => {
                    //println!("Error insert {}",e);
                    return None
                }
                Ok(inserted) =>{
                    //println!("+pet:{}",instance.name);
                    pet.id = inserted.inserted_id.as_object_id();
                    return Some(pet)
                },
            };
        }
        None
    }

}
