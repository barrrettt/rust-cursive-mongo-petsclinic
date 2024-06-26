use std::thread;

use cursive::{
    Cursive, 
    views::{Dialog, EditView, TextView, ListView}, 
    traits::{Resizable, Nameable}
};
use petsclinic_lib::DataBase;

use crate::{settings::App,show_all};

//show dialog with text views and edit text and button to connect
pub fn show(siv: &mut Cursive){

    // app user data
    let app = siv.user_data::<App>().unwrap();

    //dialog
    let dialog = Dialog::new()
        .title("MONGO DB CONNECTION")
        .content(ListView::new()
            .child("User",EditView::new().content(&app.settings.db_user).with_name("edit_user"))
            .child("Pass",EditView::new().content(&app.settings.db_pass).with_name("edit_pass"))
            .child("Url ",EditView::new().content(&app.settings.db_url).with_name("edit_url"))
            .child("Port",EditView::new().content(&app.settings.db_port).with_name("edit_port"))
            .fixed_width(50)
        )
        .button("Default",on_default)
        .button("Connect",|s|{
            on_connect(s);
        });

    //dialog
    siv.add_layer(dialog);
}

//set default connection url
fn on_default(siv: &mut Cursive) {
    // Get handles for each view and add defaults.
    siv.find_name::<EditView>("edit_user").unwrap().set_content("admin");
    siv.find_name::<EditView>("edit_pass").unwrap().set_content("admin");
    siv.find_name::<EditView>("edit_url").unwrap().set_content("localhost");
    siv.find_name::<EditView>("edit_port").unwrap().set_content("27017");
}

//Try connect
fn on_connect(siv: &mut Cursive) {
    // Get handles for each view and get data.
    let str_user = siv.find_name::<EditView>("edit_user").unwrap().get_content().to_string();
    let str_pass = siv.find_name::<EditView>("edit_pass").unwrap().get_content().to_string();
    let str_url  = siv.find_name::<EditView>("edit_url").unwrap().get_content().to_string();
    let str_port = siv.find_name::<EditView>("edit_port").unwrap().get_content().to_string();

    //to settings
    let app = siv.user_data::<App>().unwrap();
    app.settings.db_user = str_user;
    app.settings.db_pass = str_pass;
    app.settings.db_url  = str_url;
    app.settings.db_port = str_port;

    //show dialog 
    let str_mongo_url = app.settings.get_mongo_url_connector();
    siv.pop_layer();
    siv.add_layer(Dialog::around(
        TextView::new(format!("Connecting... {}", str_mongo_url ))
    ));

    //try connect to mongodb
    let cb = siv.cb_sink().clone();
    thread::spawn(move || {
        if let Ok(result) = DataBase::connect(&str_mongo_url) {
            cb.send(Box::new(|s|{
                try_done(s,result);
            })).unwrap();

        }else{
            cb.send(Box::new(|s|{
                s.pop_layer();
            })).unwrap();
        }
    });
}

//check result
fn try_done(s: &mut Cursive, result: Option<DataBase>){
    
    //database to user data
    let app = s.user_data::<App>().unwrap();    
    app.database = result;

    //show result to user
    match &app.database {
        Some(_d) => {
            s.pop_layer();
            show_all(s);
        }
        None => {
            s.pop_layer();
            s.add_layer(Dialog::info("ERROR ON CONNECTION"));
        }
    };
}
