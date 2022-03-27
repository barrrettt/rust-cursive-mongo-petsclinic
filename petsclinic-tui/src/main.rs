use petsclinic_lib as lib;

fn main() {
    println!("Run...");
    let db = match lib::connect(){
        Ok(db)=>{
            println!("OK");
            db
        }
        Err(e)=>{
            println!("Error {}",e);
            return;
        },
    };

    let _result = lib::find_customers(&db, "");
    
}
