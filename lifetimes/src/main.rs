//the returned refernce will be valid for as long as both parameters are valid
// the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y
// the returned reference will also be valid for the lenght of the smaller of the lifetimes of x and y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x 
    } else {
        y
    }
}

fn main() {
    let string_1 = String::from("abcd");
    let string_2 = "xyz";

    let result = longest(string_1.as_str(), string_2);
    println! ("The longest string is {}", result);

    ////The  below fails
    // let string_3 = String::from("dan");
    // let result;
    // {
    //     let string_4 = String::from("daniel");
    //     result = longest(string_3.as_str(), string_4.as_str());
    // }
    // println!("The longest string is {}", result);
}