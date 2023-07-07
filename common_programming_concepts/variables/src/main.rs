// immutability
// fn main() {
//   let x = 5;
//    println! (" The value of x is {x}");
//    x = 6;
//    println! (" The value of x is : {x}");

// }


//mutability

fn mutability(){

    println! ( "we declare mutable variables using: let mut");
    let mut x = 5;
    println! ("The value of x is: {x}");
    x = 6;
    println! (" The value of x is: {x}");
}

fn declaring_constants(){
    println! ("constants are always immutable and are declared suing the CONST keyword, use UPPERCASE with underscore btwn words");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 30;
    println! ("{THREE_HOURS_IN_SECONDS}");
}


fn shadowing_concept() {
    println! ("Shadowing is declaring a new variable with the same name as previous variable, 1st var shadowed by 2nd var");
    println! (" Achieved by using let and using the same variable name");
    println! (" Advantage of using shadowing over mut is because we can change the data type of first var with shadowing but not possible with mut");
    let x = 5;

    //shadowing
    let x = x+ 1;


    // inner scope shadowing remains in the inner scope
    {
        let x = x* 2;
        println! (" The value of x in the inner scope is: {x}");

    }
    // the global scope shadowing still remains after the inner shadow ends

    println! (" The value of x is: {x}");

}

//differences between mut and shadowing
// this passes successfully - so shadowing you should use let again 
//fn main() {let spaces = "  ";
//let spaces = spaces.len();
//}

//using mut fails- mut you don't need to use let again- mismatched types 
fn main(){
    println! ();
    println! ("Mutability concept");
    println! ();
    mutability();
    println! ();
    println! ("Constants concept");
    println! ();
    declaring_constants();
    println! ();
    println! ("shadowing concept");
    println! ();
    shadowing_concept();
    println! ();


}

