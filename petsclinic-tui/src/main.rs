mod views;
use petsclinic_lib::DataBase;

use cursive::{Cursive, CursiveExt};
use views::menubar::create_menu_bar;

fn main() {
    

    //tests database
    let database = match connect_database(){
        Some(d) => d,
        None => {
            println!("No connect to database!");
            return
        },
    };

    
    find_name_print(&database);
    //reset_database(&database);

    //launch TUI
    launch_tui();

}

fn launch_tui(){
    //new cursive TUI
    let mut siv = Cursive::new();
    siv.add_global_callback('q', |s| s.quit());
    
    //menubar
    create_menu_bar(&mut siv);
    
    //run
    siv.run();
}

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

fn reset_database(database:&DataBase){
     //delete all
     println!("Deleting collecions...");
     database.delete_database();
 
     //repoblate
     println!("Creating mocks..");
     database.create_db_mocks();
}

fn find_name_print(database:&DataBase){
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

