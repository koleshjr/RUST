

//you must declare the type of each parameter
fn another_function(x: i32){
    println!("The value of x is {x}");
}

fn print_labeled_measurements(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

// statements - instructions that perform some action and do not return a value
//expressions- evaluate to a resultant  value.

fn statement_expression(){
    let _y =6; //statement
    let _z = _y+ 2; // expression
    let _q = {
        let x =3;
        x+1         // expressions do not include semicolons, including semicolons turns them into statements hence won't return a value
    };
    println!("The value of q is: {_q}");
    println!("The value of y is: {_y}");
    println!("The value of z is: {_z}");


}

//functions with return values

fn five() -> i32 {
    5
}

fn main() {

    println! ("Functions: should use Snake case, lowercase and underscores separate words");
    println! ("You can pass in arguments in functions sunch as fn me(age: u32).....");
    println! ("Statements are instructions that perform some action and do not return a value: e.g let y =6;");
    println! ("Expressions are instructions that evaluate to a value: e.g count += 1 , they should not end with a semi colon becuase that would turn them to statements not returning a value");
    
    another_function(6);
    println!("Hello, world!");
    print_labeled_measurements(7, 'h');
    statement_expression();
    five();
    
    
}


