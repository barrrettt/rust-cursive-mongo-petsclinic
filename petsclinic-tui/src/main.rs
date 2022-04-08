mod views;
mod settings;

use petsclinic_lib::DataBase;
use cursive::{Cursive, CursiveExt, event, views::LinearLayout};
use settings::{App,AppSettings};
use views::{dialog_connect, menubar, list_customers, detail_customer, };

//MAIN 
fn main() { 

    //launch TUI 
    launch_tui(); 

    //helpers
    //_reset_data();
    //_create_mocks();
} 

fn launch_tui(){
    //new cursive TUI
    let mut siv = Cursive::new();
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
    
    //title
    siv.set_window_title("PET CLINIC");

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
    //menu visible
    siv.set_autohide_menu(false);

    //main view
    let panel_main = LinearLayout::horizontal()
    .child(list_customers::new())
    .child(detail_customer::new(siv))
    //.child(list_pets::new(siv))
    //.child(detail_pet::new(siv))
    ;
    
    //add layer
    siv.add_layer(panel_main);

    //poblate all customers
    list_customers::poblate_list(siv,"");
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


//UTILS
fn _reset_data(){
    //to settings
    match DataBase::connect("mongodb://admin:admin@localhost:27017") {
        Ok(r) => r.unwrap().delete_database(),
        Err(_) => (),
    }
}

fn _create_mocks(){
    match DataBase::connect("mongodb://admin:admin@localhost:27017") {
        Ok(r) => r.unwrap().create_db_mocks(),
        Err(_) => (),
    }
}