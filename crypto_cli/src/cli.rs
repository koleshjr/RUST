//clap
use clap::{App, Arg, SubCommand};

// import the crypto function
use crate::api::crypto;

pub async fn cli (){
    // //cli retrieves all arguments from the cli with the env::args().colect()
    // let args: Vec<String> = env::args().collect();

    // // checks if there is an additional argument "crypto" truue: runs the crypto function, else return an error message
    // if args.len()> 1 && args[1] == "crypto"{
    //     crypto().await.unwrap();
    // }else{
    //     println! ("Invalid command. Usage: cargo run crypto");
    // }

    let matches = App::new("Crypto CLI")

        .subcommand(
            SubCommand::with_name("list")
                .about("Handles cryptocurrency list commands")
                .subcommand(
                    SubCommand::with_name("crypto")
                        .about("list crypto currency data from coin market capp")
                        .arg(
                            Arg::with_name("cryptos")
                                .required(true)
                                .index(1)
                                .help("Comma separated crypto currency Ids (example: 1,2,3,4")
                        )
                        .arg(
                            Arg::with_name("convert")
                                .required(true)
                                .index(2)
                                .default_value("USD")
                                .help("Conversion currency")
                        ),
                ),
        )
        .get_matches();
        /*  
        Check for the "crypto" subcommand and get the positional arguments in our case:
            1: cryptos: comma separated crypto currency ids 
            2: convert: String: possible values USD, KES, EUR 
        
        */
    if let Some(list_matches) = matches.subcommand_matches("list") {
        if let Some(crypto_matches) = list_matches.subcommand_matches("crypto"){
            let cryptos = crypto_matches.value_of("cryptos").unwrap();
            let convert = crypto_matches.value_of("convert").unwrap();
            
            let cryptos: Vec<&str> = cryptos.split(',').collect();
            println!("{:?}, {}", cryptos, convert);
            crypto(cryptos, convert).await.unwrap();

        } else {
            println!("Subcommand 'crypto' is missing")
        }

    } else {
        println!("Please use the correct subcommand");
    }
}

