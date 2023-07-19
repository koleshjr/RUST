
use std::io;    //prelude
use rand::Rng;  //Import the random number generator trait

fn main(){
    println! ("Welcome to the Guessing game!");

    //generate a random number between 0-100 inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println! ("Please enter your guess, You can choose a number from 1 to 100 inclusive, to exit type exit");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");


        if guess.trim().eq_ignore_ascii_case("exit"){
            println! ("Exiting the game, Goodbye");
            break;
        }

        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println! ("Invalid input, Please enter a valid number");
                continue;
            }
        };

        println! ("You guessed, {}",guess );

        //Check if the guess is correct

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println! ("Too small! Try Again"),
            std::cmp::Ordering::Greater => println! ("Too Big! Try Again"),
            std::cmp::Ordering::Equal => {
                println! ("Congratulations! You guessed the correct number! {}", secret_number);
                break;
            }
        }

        
    }
}