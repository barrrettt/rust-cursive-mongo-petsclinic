mod views;
mod settings;

use petsclinic_lib::DataBase;
use cursive::{Cursive, CursiveExt, event};
use settings::{App,AppSettings};
use views::{dialog_connect, menubar, list_customers, };

//MAIN
fn main() {
    //launch TUI
    launch_tui(); 
}

fn launch_tui(){
    //new cursive TUI
    let mut siv = Cursive::new();
    siv.set_window_title("PET CLINIC");

    //initial user data
    siv.set_user_data(
    App{
        settings: AppSettings{
            db_user: "admin".to_string(),
            db_pass:"admin".to_string(),
            db_url:"localhost".to_string(),
            db_port:"27017".to_string(),
        },
        database:None,
    });

    //'Esc' is global quit
    siv.add_global_callback(event::Key::Esc, |s| s.quit());
    
    //show connection pane
    dialog_connect::show(&mut siv);
    
    //create menubar
    menubar::show(&mut siv);
    
    //run
    siv.run();
}

fn show_all(siv: &mut Cursive){
    siv.set_autohide_menu(false);
    list_customers::show(siv);
}

// cosas de database
fn connect_database(mongo_url:&str)->Option<DataBase>{
    //println!("Connecting to mongodb...");
    match DataBase::connect(mongo_url){
        Ok(database)=>{
            database
        }
        Err(_)=>{
            //println!("Error when connecting to mongodb! {}",e);
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

