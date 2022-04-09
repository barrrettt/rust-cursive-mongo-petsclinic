
use cursive::{views::{Panel, LinearLayout, ResizedView, EditView, TextView, TextArea}, traits::{Resizable, Nameable}, Cursive};
use mongodb::bson::oid::ObjectId;
use petsclinic_lib::datamodels::customer::Customer;
use crate::{settings::App};

//show panel
pub fn new() -> ResizedView<Panel<LinearLayout>> {
    
    let fixed_with = 30;

    //panel 
    let panel = Panel::new(
        LinearLayout::vertical()
        // ID
        .child(LinearLayout::horizontal()
            .child(TextView::new("ID "))
            .child(TextView::new("-").with_name("customer_id"))
        )

        // name
        .child(LinearLayout::horizontal()
            .child(TextView::new("Names  "))
            .child(EditView::new().with_name("customer_names").fixed_width(fixed_with))
        )

        // NOTE
        .child(LinearLayout::horizontal()
            .child(TextView::new("Notes  "))
            .child(TextArea::new().with_name("customer_note").min_height(4).fixed_width(fixed_with))
        )

        // Contact type and edit text
        .child(LinearLayout::horizontal()
            .child(TextView::new("Contact type  "))
            .child(TextView::new("-").with_name("customer_contact"))
        )

        // Time
        .child(LinearLayout::horizontal()
            .child(TextView::new("Time  "))
            .child(TextView::new("-").with_name("customer_time"))
        )

    )
    //Title and sizes
    .title("Customer")
    .min_size((30, 10));
    
    panel
}

pub fn update_customer(siv: &mut Cursive, id: &str){
    //siv.add_layer(Dialog::info(format!("ID {}",id.to_string())));
    //user data, database and get id
    let app = siv.user_data::<App>().unwrap();
    let database = app.database.as_ref().unwrap();

    let oid = &ObjectId::parse_str(id).unwrap();
    
    let customer:Customer;
    if let Some(c) = database.get_customer_by_id(oid){
        customer = c;
    }else{
        return;
    }
  
    // customer data model to view
    siv.find_name::<TextView>("customer_id").unwrap().set_content(
        customer.id.unwrap().to_hex()
    );
    
    siv.call_on_name("customer_names", |t: &mut EditView| {
        t.set_content(customer.name);
    });

    siv.find_name::<TextArea>("customer_note").unwrap().set_content(customer.note);
    siv.find_name::<TextView>("customer_time").unwrap().set_content(customer.update_time.to_string());

}