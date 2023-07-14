#[derive(Debug)]
struct Rectangle{
    width: u32,
    height:u32,
}

//implementation block: everything within this block will be associated with the rectangle type
//Associated functions are all functions that come after the impl block, since they are all associated with the type that comes after the key word impl
//associated functions that arent methods are often used for constructors that will return a new instance of the struct, often called new, but new isnt a built in word
impl Rectangle {

    //self represents the rectangle struct, we use a reference because we dont want to take ownership, we could use &mut if we wanted to change what we have borrowed
    //you can use self mutably instead of the reference but it is very rare only when we want to transform the original instance to something else
    //we can also choose the same name as our fields as the method name, e.g we can make a new function called width

    fn area(&self) -> u32{         
        self.width * self.height
    }


    //methods with the same name as fields are known as getters, and RUST does not implement them automatically for struct fields as some other languages
    fn width(&self) -> bool{
        self.width > 0
    }

    fn can_hold(&self, rectangle: &Rectangle )->bool {
        self.width > rectangle.width && self.height > rectangle.height}

    //to call this associated function we use the :: syntax with the struct name e.g Rectangle::square(3), String::from("zae")
    fn square(size: u32)-> Self{
        Self {                      //these self keyword and the one above are aliases for the type that appears after the impl keyword, which in this case is Rectangle
            width: size,
            height: size,
        }
    }
}

fn area_rectangle (){
    let rect1 = Rectangle{
        width: 43,
        height: 25,
    };

    println!("The area of {:?} is {}", rect1, rect1.area());
    if rect1.width() {
        println! ("The rectangle has a non zero width; it is {}", rect1.width);
    }

}


fn can_fit(){
    let rect1 = Rectangle{
        width: 30,
        height:50,
    };

    let rect2 = Rectangle{
        width: 10,
        height:40,
    };

    let rect3 = Rectangle{
        width: 60,
        height: 45,
    };

    println! ("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println! ("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn associated_funcs(){
    let rect1 = Rectangle::square(5);
    println! ("This is an associated function {:?}",rect1 );
}




fn main(){
    area_rectangle();
    can_fit();
    associated_funcs();

}