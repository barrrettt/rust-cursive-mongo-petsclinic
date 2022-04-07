
use cursive::{Cursive, views::{LinearLayout, Panel, Button}, traits::Resizable, XY};

use crate::{settings::App};

//show panel
pub fn show(siv: &mut Cursive){

    // app user data
    let app = siv.user_data::<App>().unwrap();

    // panel
    let panel = 
    LinearLayout::vertical()
        .child(
            Panel::new(Button::new("Quit", |s| s.quit()))
                .title("Customers")
                //.fixed_width(40)
                .fixed_size( XY::new(40, 10)),
        );

    //dialog
    siv.add_layer(panel);
}