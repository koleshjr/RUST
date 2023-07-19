use std::io;

fn fibonacci(n: u32)-> u32{
    //if n is either 0 and 1 return the same number 
    //else retun fib(n-1) + fib(n -2)

    if n == 0 {
        0
    }else if n ==1 {
        1
    }else {
        fibonacci(n-1) + fibonacci(n-2)
    }

    
}

fn main(){
    println! (" Fibonacci Program");

    loop {
        println! ("Choose an unsigned number (type 'exit' to quit) ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        if choice.trim().eq_ignore_ascii_case("exit"){
            println! ("Exiting the program: Goodbye");
            break;
        }

        let number: u32 = match choice.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println! ("Invalid input: Please use a valid number");
                continue;
            }
        };

        let answer = fibonacci(number);
        println! ("The fibonacci of {} is {}", number, answer);




    }
}