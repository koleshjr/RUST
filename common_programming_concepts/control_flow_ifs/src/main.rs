// fn main() {
//     let number = 7;

//     //condition must be a bool otherwise, we will get an error
//     if number < 5 {
//         println! ("condition was true");
//     } else {
//         println!(" condition was false");
//     }
// }


// handling multiple conditions
// only executes the block for the first true condition
// fn main() {
//     let number = 6;


//     if number % 4 == 0 {
//         println! ("Number is divisible by 4");
//     } else if number % 3 == 0{
//         println! ("Number is divisible by 3");
//     } else if number % 2 == 0 {
//         println! ("Number is divisble by 2");
//     } else {
//         println! ("Number is not divisible by 4, 3 or 2")
//     }
// }

//using an if in a let statement
fn main() {
    let condition = false;
    //should be of the same data type(int and string would fail)
    let number = if condition {5} else {6};

    println! (" The value of number is: {number}");
}