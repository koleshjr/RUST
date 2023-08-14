/*to make an API request to the /v2/cryptocurrency/quotes/latest  endpoint thatt returns the latest market quote for one or more cryptocurrencies
 you will have to define the structs for the data you want to extract from the API and specify the attributes with serdes derive trait*/

//  here is the struct that retrieves the id, name, symbol and quote data of the specified cryptocurrencies
//The api retunes daata that may be more than you need but with serde crate you can specify the exct data you need as shown below
use serde::{Serialize, Deserialize};
use std::env;
use dotenv::dotenv;


#[derive(Debug, Deserialize, Serialize)]
struct ApiResponse {
    data: Data
}

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    // Add fields that you need from the data object
    #[serde(rename = "1")]
    crypto_1: Cryptocurrency,

    #[serde(rename = "2")]
    crypto_2: Cryptocurrency,

    #[serde(rename = "3")]
    crypto_3: Cryptocurrency,

    #[serde(rename = "4")]
    crypto_4: Cryptocurrency,


}

#[derive(Debug , Deserialize, Serialize)]
struct Cryptocurrency{
    id:u32,
    name: String,
    symbol: String,
    quote: Quote
}

#[derive(Debug, Deserialize, Serialize)]
struct Quote {
    USD: QuoteDetails
}

#[derive(Debug, Deserialize, Serialize)]
struct QuoteDetails{
    price: f64,
    volume_24h: f64
}

// set up the API to call the endpoint using reqwest

use reqwest::Client;
use reqwest::Error;

// tokio enabled assynchronous function that requests the endpoint with client.get() after creating a client instance with the client::new mwthod
pub async fn crypto(cryptos: Vec<&str>, convert: &str) -> Result<(), Error> {
    //  load the environment variables

    dotenv().ok();
    let coinmarketcap_api = env::var("COINMARKET_API").expect("I have not seen the Coin Market API");

    let client = Client::new();
    
    let url = " https://pro-api.coinmarketcap.com/v2/cryptocurrency/quotes/latest ";
    // let params = [
    //     ("id", "1,2,3,4"),
    //     ("convert", "USD"), //CONVERT MARKET VALUES TO USD
    // ];

    let params = [

        ("id", &cryptos.join(",")),
        ("convert", &convert.to_string()),
    ];

    println!("{:?}", params);

    let response = client.get(url)
    
    // header function call on the request builder instance takes in your API key, the query function takes in the parameters and send function sends the request
        .header("X-CMC_PRO_API_KEY", coinmarketcap_api)
        .query(&params)
        .send().await?;
    

    // deserializes the JSON response with serde-json's from_str method that takes in a JSON string
    let result: ApiResponse = serde_json::from_str(&*response.text().await?).unwrap();

    // we then prinnt the result of the deserialization operation in the console
    println!("{:#?}", result);
    Ok(())
}