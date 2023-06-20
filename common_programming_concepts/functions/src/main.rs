fn main() {
    another_function(6);
    println!("Hello, world!");
    print_labeled_measurements(7, 'h');
    statement_expression();
    
    
}


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


