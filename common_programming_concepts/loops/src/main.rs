// we have loop, while and for


//loop : - executes a block of code over and over again forever or until you explicitly tell it to stop
fn running_a_forever_loop() {
    //can be broken when a condition is met using break to break the whole code and continue to skip to the next iteration
    let mut counter = 5;
    loop {
        println! ("again!");
        counter -= 1;
        if counter == 0 {
            break;
    }

    }
}


//Returning Values from Loops
fn returning_values_from_loops() {
    let mut counter = 0;
    let result = loop {
        counter +=1;

        if counter == 10 {
            break counter * 2;
        }
    }; //ending it because its a statement just like let x = 34;

    println! (" The result is {result}")
}

//loop labels to diambiguate between multiple loops
//we can specify a loop label on a loop that you can use with break or continue to specify that those keywords apply to the labelled loop instead of using break or ccontinue
//they must  begine with a single quote

fn using_loop_labels(){
    let mut count = 0;
    //outer loop with the label counting up
    'counting_up: loop {
        println! ("count = {count}");
        let mut remaining = 10;

        //inner loop
        loop {
            println! ("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -=1;
            
        }
        count += 1;
        println! ("End remaining = {remaining}");
    }
    println! ("End count = {count}");
    
}

//conditional loops with while 

fn using_a_while_loop_esample_one() {
    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5 {
        println! (" The value is: {}", a[index]);

        index +=1;
    }
}

fn using_a_while_loop_esample_two() {
    let mut number = 3;
    while number != 0 {
        println! ("{number}!");

        number -= 1;

    }

    println! ("LIFTOFF!!!")
}

// using a for loop
// helps increase the safety of our code as we cannot go past the array limit

fn using_a_for_loop(){
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println! ("the value is: {element}");
    }
}

//adding a range to our for loop

fn liftoff_with_for_and_range(){
    for number in (1..4).rev(){
        println! ("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn main(){
    println! ();
    println! ("Running a forever loop");
    println! ();
    running_a_forever_loop();
    println! ();
    println! ("Returning values from loops");
    println! ();
    returning_values_from_loops();
    println! ();
    println! ("using loop labels to break");
    println! ();
    using_loop_labels();
    println! ();
    println! ("using a while loop example 1");
    println! ();
    using_a_while_loop_esample_one();
    println! ();
    println! ("using a while loop example 2");
    println! ();
    using_a_while_loop_esample_two();
    println! ();
    println! ("using a for loop to print out elements");
    println! ();
    using_a_for_loop();
    println! ();
    println! ("uses for for safety and range reversed i.e in (1..4).rev()");
    println! ();
    liftoff_with_for_and_range();
}