// A rust program to show how to use Enums and pattern matching

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64,f64),
}

impl Shape{
    fn area(&self)-> f64{
        match self {
            Shape::Circle(radius) => {
                let area = 3.14 * radius * radius;
                area
            },

            Shape::Rectangle(width, height) => width * height,
        
            Shape::Triangle(a, b, c) => {
                let s = (a+b+c)/2.0;
                (s * (s-a)* (s-b)* (s-c)).sqrt()
            }
        }

    }
}

fn main(){
    let circle_1 = Shape::Circle(7.0);
    let rect_1 = Shape::Rectangle(4.0,5.0);
    let tri_1 = Shape::Triangle(3.0,4.0,6.0);

    println! ("The area of {:?} is {}", circle_1, circle_1.area());
    println! ("The area of {:?} is {}", rect_1, rect_1.area());
    println! ("The area of {:?} is {}", tri_1, tri_1.area());
}