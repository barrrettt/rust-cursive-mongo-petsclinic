mod views;
mod settings;

use petsclinic_lib::DataBase;
use cursive::{Cursive, CursiveExt};
use settings::{App,AppSettings};
use views::menubar::create_menu_bar;

//MAIN
fn main() {
    //launch TUI
    launch_tui(); 
}

fn launch_tui(){
    //new cursive TUI
    let mut siv = Cursive::new();

    //add user data
    siv.set_user_data(
    App{
        settings: AppSettings{database_url: "mongodb://admin:admin@localhost:27017".to_string(),},
        database:None,
    });

    //'q' is global quit
    siv.add_global_callback('q', |s| s.quit());
    
    //create menubar
    create_menu_bar(&mut siv);
    
    //run
    siv.run();
}


// cosas de database
fn connect_database()->Option<DataBase>{
    println!("Connecting to mongodb...");
    match DataBase::connect(){
        Ok(database)=>{
            database
        }
        Err(e)=>{
            println!("Error when connecting to mongodb! {}",e);
            None
        },
    }
}

fn _reset_database(database:&DataBase){
     //delete all
     println!("Deleting collecions...");
     database.delete_database();
 
     //repoblate
     println!("Creating mocks..");
     database.create_db_mocks();
}

fn _find_name_print(database:&DataBase){
    //find by name
    let result = database.find_like_name("Javier");
    if let Some(customers) = result{
        println!("Find by name result:");
        for customer in customers{
            println!("C:{:?}-{:?}",customer.name,customer.id);
        }
    }else{
        println!("result {:?}",result.expect("Error"));
    }
}

