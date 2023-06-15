//code that gets a guess from a user and prints it
//to obtain user input and then print the result as output we need to bring the io input/output library into scope
//io library comes from the standard library known as std
//by default rust has a set of items defined in the std library that it brings into the scope of every program

use std::io; // helps you accept user input
use rand::Rng; // defines methods that random number generators implement, and this trait must be in scope for us to use it
use std::cmp::Ordering;


fn main() {
    //here we are printing macros as we had learnt in some previous lessons
    println!("Guess the number !");


    //we call the rand::thread_rng func that calls a particular rand num generator that is local to the current thread of execution and is seeded by the OS
    //then we call the gen_range func on the random num generator defined by the rng trait we brought into scope , takes a range expression  as the argument
    //then generates a random number in that range start..=end which is inclusive of the start and end
    let secret_number = rand::thread_rng().gen_range(1..=100);


    //println!("The secret number is: {secret_number}");

    loop{
        println!("Please input in your guess.");

        //here we are defining/creating a mutable variable using the let 
        //String::new returns a new instance of a String- UTF - 8 encoded bit of text
        // new makes a new val of some kind

        let mut guess = String::new(); //create a mutable var guess and bind it to a new instance of a string

        //we will now use the io module from std lib to accept user input using stdin func
        io::stdin()

            //read line func gets input from the user, we pass the mut guess var to tell it what string to store the user input in
            //& indicates that this argument is a reference which gives you a way to let multpiple parts of your code access one piece of data
            //without needing to copy it into memory ,multiple times
            //references are also immutable so you have to type mut infront of it
            .read_line(&mut guess) //take whatever the user types into stdin input and puts that into a new string without overwritting its contents

            //read_line also returns a Result value(an enumeration or inshort enum) a type that can be in many states (variants)- ok and Err
            //ok indicates success and inside it is the successfully generated value, err contains info whhy/how the operation failed
            //the values of the Result type, like values of any types, has methods you can call e.g expect 
            //if the instance of Result is an err value, ecpect will cause the program to crash and display the message that you passed as argument to expect
            //an err would likely be the result of an error coming from the underlying OS
            //If the instace of the result is OK which indicates success, expect will take the return val that OK is holding and return
            //just that value so you can use it. In this case is the no of bytes in the user's input
            //if you don't call expect the progrm will compile but you'll get a warning that you havent used the result value returned from read_line
            // this indicates that the program has not handled a possible error


            .expect("Failed to read line");

        // we define a new variable called guess that shadows the prev guess value with a new one
        
        //let guess: u32 = guess.trim().parse().expect("Please type a number!! ");

        //handling invalid input by continuing instead of quitting
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //the {} acts as a place holder like f strings in python
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    

}
