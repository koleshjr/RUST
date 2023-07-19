//match allows you to compare a value against a series of patterns and then execute code based on which pattern matches




//<<---------------------------------------Match Flow Constructs --------------------------------------------------------------------------->>>>>

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}


//<<---------------------------------------Patterns that Bind to Values --------------------------------------------------------------------------->>>>>
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}



fn value_in_cents(coin: Coin) -> u8{

    //when the match expression executes, it compares the resultant values against the pattern of each arm in order
    match coin{                 //this varies with If becuase in if we get a boolean value, but here it can be any type, thetype of coin here is a coin enum
        Coin::Penny => {
            println! ("Lucky penny!");
            1
        },       //an arm has a pattern and code to run separated by a => operator, different arms separated by a comma


        //we dont use curly brackets if the match arm is short for example: Coin::Nickel => {5}
        Coin::Nickel => 5, 
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println! ("State quarter from {:?}!", state);
            25
        }
    }
}


// Matching with Option<T>
//lets say we want to write a function that takes an Option<i32> and if there is a value inside , adds 1 to that value, if no value the function should return a None Value and not attempt
//to perform any operations

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}


//catch all patterns and the _placeholder


fn dice_roll(){
    let dice_roll = 9;
    match dice_roll{
        3=> add_fancy_hat(), //if 3 add fancy hat
        7=> remove_fancy_hat(), //if 7 remove fancy hat
        // other => move_player(other) //if other move player other times, catch all should be placed last because the order matter
        //_ is a special pattern that matches any value and does not bind to that value if we don not want to use the catch all above
        _ => reroll(), //if not 3 and 7 reroll, we are explicitly ignoring all other values in the last arm
        //-=>(), //nothing happens
    }

    fn add_fancy_hat(){}
    fn remove_fancy_hat(){}
    fn move_player(num_spaces: u8){}
    fn reroll(){}
}
    
fn main(){

    //match control flow construct
    let coin1 = Coin::Penny;
    let coin2 = Coin::Nickel;
    let coin3 = Coin::Dime;

    //Patterns that Bind to Values
    let coin4 = Coin::Quarter(UsState::Alabama); 
    let coin5 = Coin::Quarter(UsState::Alaska); 

    //call the function
    value_in_cents(coin1);
    value_in_cents(coin2);
    value_in_cents(coin3);
    value_in_cents(coin4);
    value_in_cents(coin5);

    //matching with optio<T> - combining match and enums is useful in many situations
    //matches are exhaustive, we must exhaust every last possibility in order for the code to be valid, hence making the billion dollar mistake of Nulls is impossible

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println! ("{:?}, {:?}", six, none);


}

