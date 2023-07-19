use std::io; //bringing the prelude into scope

fn main(){


    println! ("Temperature Converter");
    //initiate a forever loop: because you want the program to run until the user decides to break

    loop {

        //list down the choices available to the user
        println! ("Choose an Option");
        println! ("Choose 1 to Convert Celsius to Fahrenheit");
        println! ("Choose 2 Convert Fahrenheit to Celsius");
        println! ("Choose 3 to Quit");

        //create a mutable variable to store the choice
        let mut choice = String::new();

        //use a mutable reference instead of cloning the whole string- a reference is a pointer
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        
        //shadowing
        let choice: u32 = match choice.trim().parse(){

            Ok(num) => num,
            Err(_) => {
                println! ("Invalid input. Please enter a number");
                continue;
            }

        };

        match choice{

            //should be exhaustive , currently we have choices 1,2,3 we have to handle the rest tooo
            1 => {
                println! ("Enter temperature in Celsius");
                let celsius = read_float_input();
                let fahrenheit = celsius_to_fahrenheit(celsius);
                println!("{}°C is equal to {}°F", celsius, fahrenheit);
            }

            2 => {
                println! ("Enter temperature in Fahrenheit");
                let fahrenheit = read_float_input();
                let celsius = fahrenheit_to_celsius(fahrenheit);
                println!("{}°F is equal to {}°C",  fahrenheit, celsius);
            }

            3 => {
                println! ("Quiting");
                break;
            }

            _ => {
                println! ("Invalid Option: Please choose a valid option");
            }
        }


        fn read_float_input()-> f64 {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            input.trim().parse().expect("Invalid input. Please enter a valid number")
        }

        fn celsius_to_fahrenheit(celsius: f64)-> f64{

            let fahrenheit = (celsius * 9.0/5.0) + 32.0;
            fahrenheit
        }

        fn fahrenheit_to_celsius(fahrenheit: f64)-> f64{
            let celsius = (fahrenheit -32.0) * 5.0/9.0;
            celsius

        }








    }
}