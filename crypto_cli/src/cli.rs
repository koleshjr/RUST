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
            SubCommand::with_name("crypto")
                .about("Handles cryptocurrency commands")
                .arg(
                    Arg::with_name("cryptos")
                        .required(true)
                        .index(1)
                        .help("Comma-separated cryptocurrency IDs (e.g., 12,13,14)"),
                )
                .arg(
                    Arg::with_name("convert")
                        .required(true)
                        .index(2)
                        .default_value("USD")
                        .help("Conversion currency"),
                )
        )
        .get_matches();

    // Check for the "crypto" subcommand
    if let Some(matches) = matches.subcommand_matches("crypto") {
        let cryptos = matches.value_of("cryptos").unwrap();
        let convert = matches.value_of("convert").unwrap();
        
        let cryptos: Vec<&str> = cryptos.split(',').collect();
        println!("{:?}, {}", cryptos, convert);
        crypto(cryptos, convert).await.unwrap();
    } else {
        println!("Please use the correct subcommand");
    }
}

