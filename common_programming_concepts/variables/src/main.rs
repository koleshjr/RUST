//immutability
//fn main() {
//   let x = 5;
//    println! (" The value of x is {x}");
//    x = 6;
//    println! (" The value of x is : {x}");
//
//}


//mutability

//fn main(){
//    let mut x = 5;
//    println! ("The value of x is: {x}");
//    x = 6;
//    println! (" The value of x is: {x}");
//};

//declaring consts
//const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;

//f/n main() {
//    let x = 5;
//
//    //shadowing
//    let x = x+ 1;


    //inner scope shadowing remains in the inner scope
//    {
//        let x = x* 2;
//        println! (" The value of x in the inner scope is: {x}");
//
//    }
    // the global scope shadowing still remains after the inner shadow ends

//    println! (" The value of x is: {x}");
//
//}

//differences between mut and shadowing
// this passes successfully - so shadowing you should use let again 
//fn main() {let spaces = "  ";
//let spaces = spaces.len();
//}

//using mut fails- mut you don't need to use let again- mismatched types 
fn main(){
    let mut spaces = "  ";
    spaces = spaces.len();
}

