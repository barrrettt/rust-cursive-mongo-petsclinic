use cursive::{Cursive, views::{Panel, Button}, traits::Resizable, XY};

use crate::{settings::App};

//show panel
pub fn _show(siv: &mut Cursive) -> cursive::views::ResizedView<Panel<Button>>{

    // app user data
    let _app = siv.user_data::<App>().unwrap();

    // panel
    let panel = Panel::new(
        Button::new("Quit", |s| s.quit())
    )
    .title("Customers")
    .fixed_size( XY::new(40, 10));

    //dialog
    panel
}