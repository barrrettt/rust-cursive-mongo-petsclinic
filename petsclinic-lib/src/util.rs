use chrono::Utc;
use rand::{seq::SliceRandom, Rng};
use lipsum::lipsum;

use crate::datamodels::{customer::Customer, pet::Pet};

fn get_random_personames(amount_names:i128) -> Vec<String> {

    let mut rng = rand::thread_rng();

    //read resource files
    let mut names_female = Vec::new();
    for line in include_str!("res/names_female.txt").lines(){
        names_female.push(line.to_ascii_lowercase());
    }
    
    let mut names_male = Vec::new();
    for line in include_str!("res/names_male.txt").lines(){
        names_male.push(line.to_ascii_lowercase());
    }
    
    let mut surnames = Vec::new();
    for line in include_str!("res/surnames.txt").lines(){
        surnames.push(line.to_ascii_lowercase());
    }

    let mut result = Vec::new();
    //exec n times
    for _i in 0..amount_names{
        let name;
        let male = rng.gen_ratio(1, 2);
        if male{
            name = names_male.choose(&mut rng).unwrap();
        }else{
            name = names_female.choose(&mut rng).unwrap();
        }
        let surname1 = surnames.choose(&mut rng).unwrap();
        let surname2 = surnames.choose(&mut rng).unwrap();

        let names = format!("{} {} {}",name,surname1,surname2);
        result.push(names);
    }
    //result
    result
}

fn get_random_petname(amount_names:i128) -> Vec<String> {
    let mut rng = rand::thread_rng();

    //read resource files
    let mut names = Vec::new();
    for line in include_str!("res/pet_names.txt").lines(){
        names.push(line.to_ascii_lowercase());
    }
    let mut result = Vec::new();
    //exec n times
    for _i in 0..amount_names{
        let petname = names.choose(&mut rng).unwrap();
        result.push(petname.to_string());
    }
    //result
    result
}

fn get_random_note(aount_notes:i128)-> Vec<String> {
    let mut result = Vec::new();
    //exec n times
    for _i in 0..aount_notes{
        let lenght = rand::thread_rng().gen_range(0..10);
        let lorem = lipsum(lenght);
        result.push(lorem);
    }
    //result
    result
}

pub(crate) fn create_mocks(db: &crate::DataBase, instances: i128){
    db.runtime.block_on(async {
        //n customers
        let names = get_random_personames(instances);
        let mut iter_names = names.iter();

        //n*2 pets
        let pet_names = get_random_petname(instances*2);
        let mut iter_pet_names = pet_names.iter();

        //notes
        let notes = get_random_note(instances);
        let mut iter_notes = notes.iter();

        //n creations
        for _i in 0..instances {
            let name = iter_names.next().unwrap();
            let note = iter_notes.next().unwrap();
            let pet1name = iter_pet_names.next().unwrap();
            let pet2name = iter_pet_names.next().unwrap();

            //new customer
            let customer = Customer{
                id:None,
                name:name.to_owned(),
                note:note.to_owned(),
                contact:vec![],
                update_time:Utc::now(),
            };

            //2 pets
            if let Some(owner) = db.add_customer(customer).await{
                let pet = Pet{
                    id:None,
                    customer_id:owner.id,
                    name:pet1name.to_owned(),
                    note:"".to_owned(),
                    pet_type:"cat".to_owned(),
                    update_time:Utc::now(),
                };
                db.add_pet(pet).await;

                let pet = Pet{
                    id:None,
                    customer_id:owner.id,
                    name:pet2name.to_owned(),
                    note:"".to_owned(),
                    pet_type:"cat".to_owned(),
                    update_time:Utc::now(),
                };
                db.add_pet(pet).await;
            }
        }
    });
}