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

//settings
#[derive(Debug)]
pub struct AppSettings{
    pub database_url: String,
}
impl Display for AppSettings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.database_url)
    }
}