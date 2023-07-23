use std::io;
use crate::ShapeType; //call ShapeType enum from shapes.rs
//Function to read a f64 value from a user input

pub fn read_f64(prompt: &str)-> f64{
    loop {
        println! ("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse(){
            Ok(value) => return value,
            Err(_)=> println! ("Invalid input. Please enter a valid number")
        }
        
    }

}

pub fn get_user_input() -> ShapeType{
    loop{
        println! ("Select a Shape");
        println! ("1. Circle");
        println! ("2. Rectangle");
        println! ("3. Triangle");

        let choice = read_f64("Enter your choice (1 or 2 or 3)");

        match choice {
            1.0 => return ShapeType::Circle,
            2.0 => return ShapeType::Rectangle,
            3.0 => return ShapeType::Triangle,
            _ => println! (" Invalid Choice. Please select a valid shape."),
        }


    }
}