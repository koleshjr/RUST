fn references_borrowing_notes(){
    println!("Reference: a pointer in thats its an address that we can follow to access the data stored in that address, that data is owned by some other variable");
    println!("Unlike a pointer, a reference is guaranteed to point to aa valid value of a particular type for the life of that reference ");
    println!("Helps to refeer to a value , without taking ownership of it ");
    println!("when s goes out of scope 'if referenced' its not dropped since it does not have ownership of what it refers to  ");
    println!("Achieved by using the '&' character, the opposite is dereferencing achieved using '*' character ");
    println!("This whole process can be explained as borrowing, you cannot change something you don't own so references are immutable by default but can be changed by adding &mut String as type ");
}

fn calculate_length_helper(s: &String)-> usize {
    s.len()
}
fn calculate_length(){
    let s1 = String::from("hello");
    let len = calculate_length_helper(&s1);
    println! ("The length of {s1} is {len}");
}

fn change_something_helper(some_string: &mut String){  //This makes it very clear that the change  something helper function will mutate the value it borrows.
    some_string.push_str(", world");
}
fn change_something(){
    let mut s = String::from("Hello"); //set this to a mutable variable
    change_something_helper(&mut s); //make sure you add the mut to after the reference symbol/char
    println! ("{s}");
}

fn reference_mutability_inmultiple_scopes(){
    let mut s = String::from("hello");

    let r1 = &s;//no problem
    let r2 = &s;//no problem

    println! ("{r1}, {r2}"); //variables r1 and r2 will not be used after this point

    let r3 = &mut s; //no problem
    println! ("{r3}");
}


fn main() {
    println! (" Notes on Referencing and Borrowing");
    println! ("------------------------------------------------------------------------------------------------------------------------------------------------------------------------------");
    references_borrowing_notes();

    println! (" Example showing referencing");
    println! ("------------------------------------------------------------------------------------------------------------------------------------------------------------------------------");    
    calculate_length();

    println! (" Implementing, references, mutable references");
    println! (" Mutable references have one limitation; there can only be one reference to that value in the scope of interest, you can use curly brackets to create a new scope, allowing for multiple references");
    
    println! ("------------------------------------------------------------------------------------------------------------------------------------------------------------------------------");    
    change_something();

    println! (" We also cannot have a mutable and immutable reference to the same value, but once the scope changes we can");
    println! (" Rust prevents dangling references, each reference must have a value borrowed from some variable");
    println! ("------------------------------------------------------------------------------------------------------------------------------------------------------------------------------");    
    reference_mutability_inmultiple_scopes();

    println! (" Final Rules: we can have either one mutable reference, or any number of immutable references at any given point in time, References must always be valid")
}
