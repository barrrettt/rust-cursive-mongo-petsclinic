use bson::Document;
use chrono::Utc;
use mongodb::{
    options::{ResolverConfig, ClientOptions}, 
    Client
};
use crate::datamodels::{
    customer::Customer,
    pet::Pet
};

//default url mongo
const DEFAULT_URL:&str  = "mongodb://admin:admin@localhost:27017";

//collections names
const DATABASE_NAME:&str = "pet_clinic";
const COLLECTION_CUSTOMER:&str = "customers";
const COLLECTION_PETS:&str = "pets";

//database model
#[derive(Clone, Debug)]
pub struct DB {
    client: Client,
}

impl DB {
    //Instancing..
    pub async fn new() -> Result<Self,Box<dyn std::error::Error>> {
        let client_uri = DEFAULT_URL;
        let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;
        Ok(Self {
            client: Client::with_options(options)?,
        })
    }

    //check if exists data
    pub async fn ckeck_databases(&self) -> bool{
        //println!("Databases:");
        let result = self.client.list_database_names(None, None).await;
        let db_pet_clinic_exists = match result {
            Ok(values)=>{
                let mut exists = false;
                for name in values{
                    //println!("- {}", name);
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

    //new data
    pub async fn create_db(&self){
        let owner = self.add_customer_with_name("Javier Fernández Barreiro").await;
        let owner = owner.unwrap();
        self.add_pet_by_name_and_owner("Lua",&owner).await;
        self.add_pet_by_name_and_owner("Jazz",&owner).await;
        self.add_pet_by_name_and_owner("Ned",&owner).await;

        let owner = self.add_customer_with_name("Isiña Garcia Novais").await;
        let owner = owner.unwrap();
        self.add_pet_by_name_and_owner("Xena",&owner).await;
        self.add_pet_by_name_and_owner("Mut",&owner).await;
        self.add_pet_by_name_and_owner("Vlad",&owner).await;
    }

    //CRUD: Customer++
    pub async fn add_customer_with_name(&self, customer_name: &str) -> Option<Customer> {
        let db = self.client.database(DATABASE_NAME);
        let customers = db.collection::<Document>(COLLECTION_CUSTOMER);
        
        //new object
        let mut instance = Customer{
            id:None,
            name:customer_name.to_owned(),
            note:"".to_owned(),
            contact:vec![],
            update_time:Utc::now(),
        };

        // Convert struct to document
        if let Ok(bson) = bson::to_bson(&instance) {
            let document = bson.as_document().unwrap();
            //inset
            match customers.insert_one(document, None).await {
                Err(_) => {
                    //println!("Error insert {}",e);
                    return None
                }
                Ok(inserted) =>{
                    //println!("+customer:{}",instance.name);
                    instance.id = inserted.inserted_id.as_object_id();
                    return Some(instance)
                },
            };
        };
        None
    }

    //CRUD: Pet++
    pub async fn add_pet_by_name_and_owner(&self, pet_name:&str, customer_owner:&Customer) -> Option<Pet> {
        let db = self.client.database(DATABASE_NAME);
        let pets = db.collection::<Document>(COLLECTION_PETS);
        
        //new object
        let mut instance = Pet{
            id:None,
            customer_id:customer_owner.id,
            name:pet_name.to_owned(),
            note:"".to_owned(),
            pet_type:"cat".to_owned(),
            update_time:Utc::now(),
        };
        
        // Convert struct to document
        let serialized = bson::to_bson(&instance);
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
                    instance.id = inserted.inserted_id.as_object_id();
                    return Some(instance)
                },
            };
        }
        None
    }

}