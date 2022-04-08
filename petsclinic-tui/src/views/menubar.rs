use cursive::{menu, views::Dialog, Cursive};

use crate::{settings::App, show_all};

use super::dialog_connect;


//MENU BAR
pub(crate) fn show(siv: &mut Cursive){
    
    siv.menubar()
    //FILE
        .add_subtree(
            "File",
            menu::Tree::new()
                //DATABASE
                .leaf("Connection", |s| { 
                    dialog_connect::show(s);
                    s.set_autohide_menu(true);

                })
                //------
                .delimiter()
                //QUIT
                .leaf("Quit", |s| s.quit()
            )
        )
    //DDL
        .add_subtree(
            "Database",
            menu::Tree::new()
                .leaf("Create mocks", move |s| { 
                    s.pop_layer();
                    //Mocks
                    let app = s.user_data::<App>().unwrap();
                    let database = app.database.as_ref().unwrap();
                    database.create_db_mocks();
                    show_all(s);
                    s.add_layer(Dialog::info("Mocks created!"));
                })

                .leaf("Delete all", move |s| { 
                    s.pop_layer();
                    //delete all
                    let app = s.user_data::<App>().unwrap();
                    let database = app.database.as_ref().unwrap();
                    database.delete_database();
                    show_all(s);
                    s.add_layer(Dialog::info("Deleted collections!"));
                })
        )

    //HELP
        .add_subtree(
            "Help",
            menu::Tree::new()
                .leaf("Resources", move |s| { 
                    s.add_layer(Dialog::info(RESOURCES_TEXT));
                })

                .leaf("About", move |s| { 
                    s.add_layer(Dialog::info(ABOUT_TEXT));
                })
        )
    ;//END

    //visible?
    //siv.set_autohide_menu(false);    
}


//const text 1
const ABOUT_TEXT:&str = r#"
PET CLINIC 
------------------------
rust + mongodb + cursive

Practice proyect by Barrrettt

https://github.com/barrrettt"#;

//const text 2
const RESOURCES_TEXT:&str = r#"
CARGO
------------------------
mongodb #connector
futures #callbacks
bson # for mongo crate
tokio # threads for mongo crate
chrono #DateTimes
serde # Structs serialization/des
rand #randoms
cursive #Text user interface
"#;