use std::fmt::{self, Display};

use petsclinic_lib::DataBase;

//User data
#[derive(Debug)]
pub struct App{
    pub settings: AppSettings,
    pub database: Option<DataBase>,
}
impl Display for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.settings)
    }
}

//settings "mongodb://admin:admin@localhost:27017"
#[derive(Debug)]
pub struct AppSettings{
    pub db_url: String,
    pub db_port: String,
    pub db_user: String,
    pub db_pass: String,
}
impl Display for AppSettings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.get_mongo_url_connector())
    }
}
impl AppSettings{
    pub fn get_mongo_url_connector(&self)->String{
        format!("mongodb://{}:{}@{}:{}",self.db_user,self.db_pass,self.db_url,self.db_port)
    }
}