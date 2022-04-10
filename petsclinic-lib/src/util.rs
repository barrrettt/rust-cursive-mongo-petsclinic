use chrono::Utc;
use fake::{faker::{lorem::en::Words, internet::en::FreeEmail, phone_number::en::PhoneNumber, address::en::{CountryName, CityName, StreetName, SecondaryAddress}, name::en::{Name, FirstName}}, Fake};
use rand::Rng;

use crate::datamodels::{customer::{Customer, ContactType, Address}, pet::Pet};

fn get_random_note()-> String {
    let lenght = rand::thread_rng().gen_range(2..10);
    let words: Vec<String> = Words(1..lenght).fake();
    words.join(" ")
}

fn get_random_contacts() -> Vec<ContactType> {
    let ammount_contacts = rand::thread_rng().gen_range(1..4);
    let mut result:Vec<ContactType> = Vec::new();
    
    for _n in 0..ammount_contacts {
        let rnd_type = rand::thread_rng().gen_range(0..3);
        let contact:ContactType;
        if rnd_type == 0{
            let value = FreeEmail().fake();
            contact = ContactType::Email(value);
        }else if rnd_type == 1{
            let value = PhoneNumber().fake();
            contact = ContactType::Telephone(value);
        }else{
            let address = Address{
                line1: CountryName().fake(),
                line2: CityName().fake(),
                line3: StreetName().fake(),
                line4: SecondaryAddress().fake(),
            };
            contact = ContactType::Address(address);
        }
        result.push(contact);
    }
    
    result
}

pub(crate) fn create_mocks(db: &crate::DataBase, instances: i128){
    db.runtime.block_on(async {

        //n creations
        for _i in 0..instances {
            //new customer
            let customer = Customer{
                id:None,
                name: Name().fake(),
                note:get_random_note().to_owned(),
                contact: get_random_contacts(),
                update_time:Utc::now(),
            };

            //2 pets
            if let Some(owner) = db.add_customer(customer).await{
                let pet = Pet{
                    id:None,
                    customer_id:owner.id,
                    name: FirstName().fake(),
                    note: get_random_note().to_owned(),
                    pet_type:"cat".to_owned(),
                    update_time:Utc::now(),
                };
                db.add_pet(pet).await;

                let pet = Pet{
                    id:None,
                    customer_id:owner.id,
                    name: FirstName().fake(),
                    note: get_random_note().to_owned(),
                    pet_type:"cat".to_owned(),
                    update_time:Utc::now(),
                };
                db.add_pet(pet).await;
            }
        }
    });
}