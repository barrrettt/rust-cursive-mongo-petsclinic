use petsclinic_lib as lib;

fn main() {
    println!("Run...");
    let db = match lib::DataBase::connect(){
        Ok(db)=>{
            println!("OK");
            db
        }
        Err(e)=>{
            println!("Error {}",e);
            return;
        },
    };

    let result = db.find_customers("Javier");
    println!("result {:?}",result);
}
