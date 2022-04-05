use cursive::{
    Cursive, 
    views::{Dialog, ListView, EditView, TextArea, Checkbox, LinearLayout, TextView, SelectView}, 
    traits::{Resizable, Nameable, Scrollable}, 
    With
};

use crate::settings::App;

//show dialog with text views and edit text and button to connect
pub(crate) fn show_dialog_settings(s:&mut Cursive){
    let value = s.user_data::<App>().unwrap().settings.database_url.to_string();
    let text_conn = format!("Current mongo url connection:\n {}", value);

    s.add_layer(
        Dialog::new()
            .title("Connection")
            .button("Connect", |s| s.quit())
            .content(
                ListView::new()
                    // Each child is a single-line view with a label
                    .child("Name", EditView::new().fixed_width(10))
                    .child("Presentation", TextArea::new().min_height(4))
                    .child(
                        "Receive spam?",
                        Checkbox::new().on_change(|s, checked| {
                            // Enable/Disable the next field depending on this checkbox
                            for name in &["email1", "email2"] {
                                s.call_on_name(name, |view: &mut EditView| {
                                    view.set_enabled(checked)
                                });
                                if checked {
                                    s.focus_name("email1").unwrap();
                                }
                            }
                        }),
                    )
                    .child(
                        "Email",
                        // Each child must have a height of 1 line,
                        // but we can still combine multiple views!
                        LinearLayout::horizontal()
                            .child(
                                EditView::new()
                                    .disabled()
                                    .with_name("email1")
                                    .fixed_width(15),
                            )
                            .child(TextView::new("@"))
                            .child(
                                EditView::new()
                                    .disabled()
                                    .with_name("email2")
                                    .fixed_width(10),
                            ),
                    )
                    // Delimiter currently are just a blank line
                    .delimiter()
                    .child(
                        "Age",
                        // Popup-mode SelectView are small enough to fit here
                        SelectView::new()
                            .popup()
                            .item_str("0-18")
                            .item_str("19-30")
                            .item_str("31-40")
                            .item_str("41+"),
                    )
                    .with(|list| {
                        // We can also add children procedurally
                        for i in 0..50 {
                            list.add_child(
                                &format!("Item {}", i),
                                EditView::new(),
                            );
                        }
                    })
                    .scrollable(),
            ),
    );
    
}