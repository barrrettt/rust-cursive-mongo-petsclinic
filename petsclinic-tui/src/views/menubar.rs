use cursive::{menu, views::Dialog, Cursive};

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