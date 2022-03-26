use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::options::ResolverConfig;
use mongodb::{options::ClientOptions, Client, Collection};

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn new() -> Result<Self,Box<dyn std::error::Error>> {
        let client_uri = "mongodb://admin:admin@localhost:27017";
        let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;
        Ok(Self {
            client: Client::with_options(options)?,
        })
    }

    pub async fn print_databases(&self) {
        println!("Databases:");
        let result = self.client.list_database_names(None, None).await;
        match result {
            Ok(values)=>{
                for name in values{
                    println!("- {}", name);
                 }
            },
            Err(e)=>print!("Error:{}",e),
            _=>(),
        };
        
    }
}