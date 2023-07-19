 
 //each variant in enum can have different types and amounts of associated data
 
 #[allow(dead_code)]
 fn enum_from_a_string(){
 
    enum IpAddrKind{
        V4(String),
        V6(String),
    }

    let _home = IpAddrKind::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrKind::V6(String::from("::1"));
 }

fn enum_with_different_types(){

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let _home = IpAddr::V4(127,0,0,1);
    let _loopback = IpAddr::V6(String::from("::1"));
}

//you can put any kind of data inside an enum variant: strings, numeric tyoes or structs, you can even include another enum


fn enums_from_structs(){
    struct Ipv4Addr {
        // --snip--
    }

    struct Ipv6Addr{
        // --snip--
    }
    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr)
    }
}

fn enums_with_multiple_types(){
    enum Message {
        Quit,                               //has no data associated with it at all
        Move { x: i32, y:i32},              //has named filds like struct does
        Write(String),                      //includes a Single string
        ChangeColor(i32,i32,i32),           //includes 3 i32 values
    }

    impl Message {
        fn call(&self){
            //method body would be here
        }
    }


    let m = Message::Write(String::from("hello"));
    m.call();

}

fn enum_option(){
    //option is a very important enum thats even included in the prelude, you dont need to bring it into scope explicitly
    

    let some_number = Some(5); //some_number is Option<i32> since we have specified a number value
    let some_char = Some('e'); //some_char is Option<char> since we have specified a char value
    let absent_number: Option<i32> = None;//absent number to be of type option<i32> since compiler cant infer the type that the corresponding some variant will hold by looking at a None Value only

    //we use option<T> because the compiler wont let us use an Option<T> value as if it were a valid value
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x+ y // will fail because they are of different types: an i8 and an option< i8>
    //when we have an i8 we can continue confidently because compiler will make sure we handle that case before using the value but am option<T> we have to confirm first
    //in other words we have to convert an Option< T> to a T before you can perform T operations with it
    //this helps catch one of the most common issues with null: assuming that something isnt null when it actually is
    //in general in order to use an option<T> value, you want to have code that will handle each variant, in order to get the T value out of a Some Variant


}



fn main(){
    enum_from_a_string();
    enum_with_different_types();
    enums_from_structs();
    enums_with_multiple_types();
    enum_option();

}