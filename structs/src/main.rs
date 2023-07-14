/* structs are similar to tuples in that they both hold multiple related values, the prics of a struct can be of different types 



*/

//creating a Struct, its like a class in oop
struct User {
    active:bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


//function to build a user from a struct
fn build_user(username: String, email: String)-> User{
    User{
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

//function  that actually builds a class
fn define_struct() {

    let mut user1 = User{
        active:true,
        username: String::from("someusername123"),
        email: String::from("someuseremail@example.com"),
        sign_in_count: 1,
    };
    
    user1.email = String::from("anotheremail@example.com");

    //I had to move this here , because apparently, when a new struct copies data from another struct(especially String attributes),
    // the original struct isnt usable anymore(ownership)-pointers and heaps and bla bla bla, but if we were copying ints, bools only then it would be usable becuase of stack copying
    println!("{}",user1.active);
    println!("{}",user1.username);
    println!("{}",user1.sign_in_count);
    println!("{}",user1.email);
    println!();    

    let user2 = build_user(String::from("stephen"), String::from("steve123@koko.com"));
    let user3 = User{
        email: String::from("user3@email.com"),
        ..user1
    };




    println!("User 2");
    println!();
    println!("{}",user2.active);
    println!("{}",user2.username);
    println!("{}",user2.sign_in_count);
    println!("{}",user2.email);    

    println!("User 3");
    println!();
    println!("{}",user3.active);
    println!("{}",user3.username);
    println!("{}",user3.sign_in_count);
    println!("{}",user3.email);       


}


// Tuple Structs ---------------------------------------------------------------------------------------
//struct tuples dont have names associated with the attributes
fn tuple_structs(){
    struct Color(i32, i32, i32);
    struct Point(i32,i32,i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    println!("{}", black.0);
    println!("{}", origin.0);
}

//Unit structs : structs that don't have any field, just like unit tuples

struct AlwaysEqual;
fn unit_stuct(){
    let subject = AlwaysEqual;
}


//introducing lifetimes; ensures that the data referenced by a struct is valid for as long as the struct is



fn main() {
    define_struct();
    tuple_structs();

}
