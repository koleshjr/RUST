

fn string_ownership_move(){
    let mut s = String::from("hello"); //string::from requests the memory it needs
    s.push_str(", world");
    let s1 = s;

    println! ("{s1}");
}

fn string_ownership_clone(){
    let mut s = String::from("hello"); //string::from requests the memory it needs
    s.push_str(", world");
    let s1 = s.clone();

    println! ("{s1}, {s}");
}

fn stack_only_data_copy(){
    let x = 5;
    let y = x;
    println! ("x = {}, y={}", x, y);
}




fn takes_ownership(some_string: String){
    println! ("{}", some_string);
}//some string goes out of scope

fn makes_copy(some_integer: i32){
    println! ("{}", some_integer);
}//some integer goes out of scope
fn take_ownership_concept(){
    let s = String::from("hello"); //s comes in scope, hence valid
    takes_ownership(s); // s 's value moves into the function hence is no longer valid heere, you cant use it afterwards
    // println!("{s}"); // this thrrows an error

    let x = 5;   //x comes into scope
    makes_copy(x); //x moves into the function  but this is a copy functionso its okay to use x going forward
    println!("{x}"); // this doesnt
}


fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string:String) -> String{
    a_string
}

fn ownership_in_return_values_and_scope(){

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);


}


fn calculate_length(s: String) -> (String, usize){
    let length = s.len();

    (s, length)
}
fn return_multiple_values_in_a_tuple(){

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println! ("The length of {s2} is {len}");
}



fn main() {
    println!("Ownership: set of rules that govern how Rust program manages memory");
    println!("In Rust memory is managed through a system of ownership with a set of rules that the compiler check, if any of the rules is violated the program won't compile");
    println!(" A stack: Last in First out, all values stored in a stack must have a known fixed size");
    println!(" A heap: Less organized , you request a certain amount of space, the memory allocator finds an empty spot on the heap that is big enough, marks it as being in use ");
    println!(" and returns a pointer(address of that location) , a process called allocation on the heap");
    println!(" Relationship of the heap and stack");
    println!("          Since the pointer to the heap is a known fixed size you can store the pointer on the stack but when you want actual data , you must follow the pointer");
    println!("Pushing to the stack is faster than allocating on the heap because the allocator does not need to search for a place to store data, that location is always at the top of the stack");
    println!("Pushing to the heap is slower because we have to find a big enough space to hold the data and then perform book keeping to prepare for the next allocation");
    println!("Accessing data in the heap is slower than the stack because you have to follow a pointer to get there");
    println!("When your code calls a function, the values passed into the function (including potentially pointers to data on the heap) and the function's local variables get pushed onto the stack");
    println!("When the function is over , those values get popped off the stack");
    println!("So what does onwership address?: in general managing heap data but the specifics below");
    println!("      keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap");
    println!("      cleaning uo unused data on the heap so you don't run out of space are all problems that ownership adresses");
    println! ("Ownership Rules");
    println!("      Eachh value in RUST has an owner");
    println!("      There can only be one owner at a time");    
    println!("      when the owner goes out of scope the value will be dropped, only valid within the scopeeeee");    

    println!();
    println!("The String type: A data type that is of no known fixed size, hence stored on the heappp");
    println!("          since it is of no fixed size, a string can change e.g hello-> hello world, we cannot assign it to a stack rather in a heap");
    println!("          thus, memory needs to be requested from the memory allocator at runtime( we do this ourselves) and we need a way of returning this memory to the allocator when we are done with our striing- RUST does this automatically once the var goes out of scope");
    println!("          copying data stored on the heap for example: let x = 'steve',, let y =x;, we dont copy the actuall data 'steve' stored on the heap , we copy the pointer, the length and the capacity, that are on the stack");
    println!("          So a problem, occurs when both x and y goes out of scope and they will try to free up the same memory - double free error, so once copied to y, rust forgets about y - this concept is called 'MOVE' ");
    println!("          If we want to deeply copy the heap data of the string , not just stack data, we can use a common method called clone");


    println!("Shallow copying ");
    string_ownership_move();//shallow copy , just the data stored on the stack, the pointer, lenght and capacity-cheap
    println!();
    println!("deeep copying");
    string_ownership_clone();// deep copy, the data stored on the heap, the data value themselves - expesive
    println!();
    println!("For data types with a fixed known size the values are stored entirely on the stack, hence copying, copies the value itself not the pointer, len , capacity as in data stored in heaps");
    stack_only_data_copy();

    println!();
    println!("Rust copy annotation: this trait can be placed on types that are stored on the stack, that trivially copies a value, making them still valid after assignment to another variable");
    println!();

    println!("Take ownership and make copy concepts in functions");
    ownership_in_return_values_and_scope();    
    println!();

    println! ("Return multiple values in Rust using a Tuple");
    return_multiple_values_in_a_tuple();



}


