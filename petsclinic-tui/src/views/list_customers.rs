
use cursive::{
    Cursive, views::{LinearLayout, Panel, EditView, SelectView, ResizedView},
    traits::{Resizable, Nameable, Scrollable}, 
    align::HAlign,
};

use crate::settings::App;

use super::detail_customer::update_customer;

// show panel 
pub fn new() -> ResizedView<Panel<LinearLayout>> {

    //EDITVIEW FOR SEARCHS
    let editview = EditView::new()
    .on_edit(on_edit)
    .fixed_width(30)
    .with_name("query");

    //SELECT VIEW FOR SELECT CUSTOMERS
    let selectview: SelectView<String> = SelectView::new()
    .on_submit(on_submit)
    .h_align(HAlign::Center);
    //add name and scroll to selectview
    let scroll_selection = selectview
    .with_name("selectview_customers")
    .scrollable();

    //panel 
    let panel = Panel::new(
        LinearLayout::vertical()
        // edit view 
        .child(editview)
        //lines results
        .child(scroll_selection)
    )
    //Title and sizes
    .min_size((30, 10));
    
    panel
}

pub fn poblate_list(siv: &mut Cursive, query: &str) {
    //user data, database and find by name
    let app = siv.user_data::<App>().unwrap();
    let database = app.database.as_ref().unwrap();
    let search_result = database.find_customers_like_name(query);

    //get selectview and add results
    siv.call_on_name("selectview_customers", |selectview: &mut SelectView| {
        selectview.clear();
         //poblate
        if let Some(customers) = search_result{
            for customer in customers{
                let label = customer.name;
                let id = customer.id.unwrap().to_hex();
                selectview.add_item(label,id);
            }
        }
    });
}

// Update results according to the query
fn on_edit(siv: &mut Cursive, query: &str, _cursor: usize) {
    //siv.add_layer(Dialog::info(format!("Query {}",query.to_string())));
    poblate_list(siv,query);
}

fn on_submit(siv: &mut Cursive, id: &str) {
    //siv.add_layer(Dialog::info(format!("SUmmit ID {}",id.to_string())));
    update_customer(siv, id);
}

