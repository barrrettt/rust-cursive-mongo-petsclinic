
use cursive::{
    Cursive, views::{LinearLayout, Panel, EditView, SelectView, ResizedView, TextView},
    traits::{Resizable, Nameable, Scrollable}, 
    align::HAlign,
};

use crate::settings::App;

use super::detail_customer::update_customer;

// show panel 
pub fn new() -> ResizedView<Panel<LinearLayout>> {

    //EDITVIEW FOR SEARCHS
    let editview_searchs = EditView::new()
    .on_edit(update_customers_list)
    .fixed_width(30)
    .with_name("query");

    //SELECT VIEW FOR SELECT CUSTOMERS
    let selectview_result_list: SelectView<String> = SelectView::new()
    .on_submit(update_customer_detail)
    .h_align(HAlign::Center);
    //add name and scroll to selectview
    let scroll_selection_result = selectview_result_list
    .with_name("selectview_customers")
    .scrollable();

    //TEXTVIEW COUNT
    let textview_count = TextView::new("")
    .with_name("customers_count")
    .fixed_width(30);

    //panel 
    let panel = Panel::new(
        LinearLayout::vertical()
        // edit view 
        .child(editview_searchs)
        //lines results
        .child(scroll_selection_result)
        //count
        .child(textview_count)
    )
    //Title and sizes
    .min_size((30, 10));
    
    //result
    panel
}

pub fn poblate_list(siv: &mut Cursive, query: &str) {
    //user data, database and find by name
    let app = siv.user_data::<App>().unwrap();
    let database = app.database.as_ref().unwrap();
    let search_result = database.find_customers_like_name_sort_list(query);
    let mut list_count = 0;

    if let Some(result) = search_result{
        let (customers,count) = result;
        //get selectview and add results
        siv.call_on_name("selectview_customers", |selectview: &mut SelectView| {
            selectview.clear();
            //poblate
            for customer in customers{
                let label = customer.name;
                let id = customer.id.unwrap().to_hex();
                selectview.add_item(label,id);
                list_count+=1;
            }
        });

        siv.call_on_name("customers_count", |view: &mut TextView| {
            view.set_content(format!("Results {}/{}",list_count,&count));
        });
    }   
}

// Update results according to the query
fn update_customers_list(siv: &mut Cursive, query: &str, _cursor: usize) {
    poblate_list(siv,query);
}

//update customer details
fn update_customer_detail(siv: &mut Cursive, id: &str) {
    update_customer(siv, id);
}

