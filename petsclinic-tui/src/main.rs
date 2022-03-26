use petsclinic_lib as pets;

fn main() {
    println!("Run...");
    match pets::connect(){
        Ok(_)=>print!("OK"),
        Err(e)=>print!("Error {}",e),
    };
}
