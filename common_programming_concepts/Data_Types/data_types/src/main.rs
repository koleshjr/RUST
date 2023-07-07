fn main() {
    //integer overflow
    // if you have a varibale with type u8(0-255) , if you try to change it to 256, an integer overflow happenas since its outside the valid range
    //Running it in debug modem, integer overflow causes the program to panic at runtime
    //Running it in release mode rust doesnt include checks for integer overlfow, it performs two complement wrapping instead
    //the val 256 becomes 0 and 257 becomes 1 and so on and so on
    //To handle overflow yyou can use the families of methods provided by the std library for primitive numeric types: Researchj
    println! (" Rust is a Statically Typed Language which means that it must know the types of all variables at compile time");
    println! ();

    println!("Scalar types: represent a single value: integers(signed-i and unsigned-u)u32, i32 ...., floats: f32, f64, booleans, characters ");
    println!();

    println! ("Integers operations");

    let _x = 2.0; // f64
    println!("the value of _x is: {_x}");
    let _y: f32 = 3.0; //f32
    let _sum = 5+ 10;
    println!("the sum of 5 + 10 is {_sum}");
    let _difference = 95.5 - 4.3;
    println!("the difference of 95.5 - 4.3 is {_difference}");
    let _product = 4*30;
    println!("the product of 4 * 30 is {_product}");
    let _quotient = 56.7 / 32.2;
    println!("the quotient of 56.7 / 32.2 is {_quotient}");
    let _truncated = -5/ 3; // results in -1
    println!("the truncation of -5 / 3 is {_truncated}");
    let _remainder = 43 %5;
    println! ("the modulus of 43 % 5 is {_remainder}");

    //booleans either true or false
    let _t = true;
    let _f:bool = false; //with explicit type annotation

    //characters- use string literals, 4 bytes in size, unicode scalar type
    let _c = 'z';
    let _z: char = 'Z'; //with explicit type annotation

    println! ("compund types: can group multiple values together: tuples,array, vector");
    println!();
    println! ("Tuples: have a fixed length size once declared and can store multiple types e.g let z: (i32,f64,char) = (500, 0.4, 'z')");
    println! ("Tuples destructuring: let(y,x,z) = z;.....tuple indexing z.0, z.1");
    println!();

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

    println!();
    println!("Arrays: cannot store multiple types like tuples, have a fixed length, hence useful when you know the no of elements wont chnage like: months");
    println!("Useful when you want to store data in a stack rather than a heap");
    println!("let a: [i32:5] = [1,2,3,4,5];,let a = [3;5];....,");
    println!("Array Indexing: a[1], a[2],");
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

    println!();
    println!("Vectors: similar to array data type , only that it can change in size hence useful for when you are not sure of the array size if its constant");
    

}
