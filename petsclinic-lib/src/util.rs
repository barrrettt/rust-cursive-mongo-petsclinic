use rand::{seq::SliceRandom, Rng};

pub(crate) fn get_random_personames(amount_names:i128) -> Vec<String> {

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

pub(crate) fn get_random_petname(amount_names:i128) -> Vec<String> {

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