fn area (width: u32, height: u32) -> u32{
    width * height
}

fn rectangles(){
    let width = 30;
    let height = 45;

    println! ("The area of 30 * 45 is {} pixels", 
    area(width, height));
}


//refactoring with tuples
fn tuple_area(dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
}


fn rectangles_tuples(){
    let rect1 = (50, 60);

    println! ("The area of a rectangle is {} pixels", tuple_area(rect1));
}



//refactoring with structs

#[derive(Debug)]// adding the attribute to derive the Debug trait
struct Rectangle {
    width: u32,
    height: u32,
}
fn struct_area(rectangle: &Rectangle) -> u32{ //borrow the struct not taking ownership
    rectangle.width * rectangle.height
}
fn struct_rectangle(){
    let scale = 2;
    let rect1 = Rectangle{
        width: dbg!(100 * scale),//dbg returns ownership of the expressions value, hence width will still have the same value as if dbg was not there 
        height: 98
    };

    println!("The rectangle is: {:#?}", rect1); //pretty print
    println! ("Area of the rectangle is: {} ", struct_area(&rect1));
    dbg!(&rect1); //prints it out on the stderr, instead of stdout like println, we dont want ownership here so we use a reference
}


fn main(){
    println! ("Raw rectangle area calculation");
    rectangles();
    println! ("tuple rectangle area calculation");
    rectangles_tuples();
    println! ("struct rectangle area calculation");
    struct_rectangle();
}