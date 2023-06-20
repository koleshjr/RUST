fn main() {
    //integer overflow
    // if you have a varibale with type u8(0-255) , if you try to change it to 256, an integer overflow happenas since its outside the valid range
    //Running it in debug modem, integer overflow causes the program to panic at runtime
    //Running it in release mode rust doesnt include checks for integer overlfow, it performs two complement wrapping instead
    //the val 256 becomes 0 and 257 becomes 1 and so on and so on
    //To handle overflow yyou can use the families of methods provided by the std library for primitive numeric types: Researchj


    let _x = 2.0; // f64
    println!("the value of _x is: {_x}");
    let _y: f32 = 3.0; //f32
    let _sum = 5+ 10;
    let _difference = 95.5 - 4.3;
    let _product = 4*30;
    let _quotient = 56.7 / 32.2;
    let _truncated = -5/ 3; // results in -1
    let _remainder = 43 %5;

    //booleans either true or false
    let _t = true;
    let _f:bool = false; //with explicit type annotation

    //characters- use string literals, 4 bytes in size, unicode scalar type
    let _c = 'z';
    let _z: char = 'Z'; //with explicit type annotation

    //compund types- groups multiple values into one type
    //tuple - have a fixed length: once declared , they cannot grow or shrink in size, can store multiple types

    //tuple destructuring
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z)= tup;
    println!("The value of y is {_y}");
    
    //tuple indexing
    let tup_2: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = tup_2.0;
    let _six_point_four = tup_2.1;
    let _one = tup_2.2;

    println!("The value of one is: {_one}");


    //the array type - unlike tuples cannot store multiple types: just elements of the same type and they have a fixed length
    //useful when you want your data to be stored on the stack rather than the heap'
    //vector is allowed to grow or shrink in size, hence that's the difference between an array and a vector
    //array is for those things you are sure of the length like months, weekdays 
    let _array_a = ["January","February","March","APril", "May"];
    let _array_b: [i32;5] = [1,2,3,4,5];
    let _array_c = [3; 6];
    let _first_month = _array_a[0];
    let _second_month = _array_a[1];
    let _fourth_month = _array_a[3];
    println!( "The first month is {_first_month}, the second is {_second_month} and the fourth is {_fourth_month}");


    //invalid access
    use std::io;
    println! ("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = _array_b[index];

    //if you provide an  invalid index that does not exist rust panicks and immediately exists preventing access to an invalid memory
    println! (" The value of the element at index {index} is: {element}");
    

}
