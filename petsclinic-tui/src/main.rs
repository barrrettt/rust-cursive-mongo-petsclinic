use petsclinic_lib as lib;

fn main() {
    println!("Running...");
    let database = match lib::DataBase::connect(){
        Ok(data)=>{
            data.unwrap()
        }
        Err(e)=>{
            println!("Error {}",e);
            return;
        },
    };

    //find by name
    let result = database.find_like_name("Javier");
    println!("result {:?}",result.expect("Error"));

    //add 
}
