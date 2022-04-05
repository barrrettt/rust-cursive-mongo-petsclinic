use cursive::{
    Cursive, 
    views::{Dialog, EditView, LinearLayout, TextView}, 
    traits::{Resizable, Nameable}
};

use crate::settings::App;

//show dialog with text views and edit text and button to connect
pub fn show_dialog_settings(siv:&mut Cursive){
    
    //mongo url
    let url = siv.user_data::<App>().unwrap().settings.database_url.to_string();

    //dialog
    let dialog = Dialog::around(
        LinearLayout::vertical()
            .child(TextView::new("Current mongo url connection:"))
            .child(EditView::new().content(url).with_name("editview_mongourl"))
            .fixed_width(50),
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
    // default value
    let url_default = siv.user_data::<App>().unwrap().settings.database_url_default.to_string();
    // Get handles for each view.
    let mut editview = siv.find_name::<EditView>("editview_mongourl").unwrap();
    editview.set_content(url_default);
}

//Try connect
fn on_connect(siv: &mut Cursive) {
    // Get handles for each view.
    let editview = siv.find_name::<EditView>("editview_mongourl").unwrap();
    siv.pop_layer();//hide last layer
    siv.add_layer(Dialog::info(format!("Current value: {}", editview.get_content() )));

}
