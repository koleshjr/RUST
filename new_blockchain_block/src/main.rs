/* A tutorial inspired by cyndie kamau */

//Import system time and unix epoch from the std::time module
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest}; //has to be defined in the cargo toml file
use std::io::{stdin};

#[derive(Debug)]
#[allow(dead_code)]

//struct of our block
struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    current_hash: String,
    previous_hash: String,

}

//implementing associated functions
impl Block {
    fn new_block(index: u32, timestamp: u64, data: String, previous_hash: String)-> Block {
        let current_hash = Block::calculate_hash(index, timestamp, &data, &previous_hash);
        Block {
            index, 
            timestamp,
            data,
            current_hash,
            previous_hash,
        }

    }

    fn calculate_hash(index: u32, timestamp: u64, data: &String, previous_hash: &String) -> String{
        let input = format!("{} {} {} {}", index, timestamp, data, previous_hash); //combines the input string into a single string called input
        //calcuating the hash 
        let mut s = Sha256::new(); // a new sha256 instance is created
        s.update(input); //pass the formatted input into the hash function
        let result = s.finalize(); //finalize computes the hash and stores it in result

        format! ("{:x}", result) //the result is then converted to a hexadecimal string using the format! macro
    }
}

fn main(){
    let mut blocks = Vec::new();

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let previous_hash = "0".to_string();
    let first_block = Block::new_block(0, timestamp, "FIRST_BLOCK".to_string(), previous_hash);
    blocks.push(first_block);

    println! ("{:?}", blocks[0]);

    for i in 1..10 {
        println! ("Type 'exit' to quit");
        println! ("Please enter datta for block number {}: ", i);

        //to handle user input
        let mut data = String::new();
        stdin().read_line(&mut data).expect("Unable to read your input");
        let data = data.trim().to_string();

        if data == "exit"{
            println! ("See yah later");
            break;
        }

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let previous_hash = blocks[i-1].current_hash.clone(); //deep copying

        let new_block = Block::new_block(i.try_into().unwrap(), timestamp, data, previous_hash);//converts usize to i32
        blocks.push(new_block);

        println!("{:?}", blocks[i]);

    }
}