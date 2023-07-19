use std::io;

//recursive function to call factorial

fn recursive_func(n: u32) -> u32{
    if n ==0{
        //base case
        1
    }else {

        //recurive case: factorial of n is n* factorial(n-1)
        n* recursive_func(n-1)
    }
}

fn main(){
    println! ("Factorial Calculator");

    loop {
        println! ("Enter a non negative integer (or 'exit' to quit')");

        let mut choice = String::new();
        io::stdin().read_line(& mut choice).expect("Failed to read line");

        if choice.trim().eq_ignore_ascii_case("exit"){
            println! (" Exiting the program: Goodbye");
            break;

        }

        let number: u32 = match choice.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println! ("Invalid choice: please enter a valid number");
                continue;
            }
        };

        let answer = recursive_func(number);
        println!("Factorial of {} is: {}", number, answer);



    }
}