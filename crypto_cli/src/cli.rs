// used to retrive the argument from the command line
use std::env;
// import the crypto function
use crate::api::crypto;

pub async fn cli (){
    //cli retrieves all arguments from the cli with the env::args().colect()
    let args: Vec<String> = env::args().collect();

    // checks if there is an additional argument "crypto" truue: runs the crypto function, else return an error message
    if args.len()> 1 && args[1] == "crypto"{
        crypto().await.unwrap();
    }else{
        println! ("Invalid command. Usage: cargo run crypto");
    }
}