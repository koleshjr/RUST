//Import required module from another file
use crate::utils::read_f64;//starts at the root, finds a utils.rs , finds readf64 function

// define an enum representing differeent types and set it to public since items of a module are private by default
pub enum ShapeType {
    Circle,
    Rectangle,
    Triangle
}

//define a struct representing a shape with two attributes, shape type (enum), and dimensions a vec<f64>

pub struct Shape {
    shape_type: ShapeType,
    dimensions: Vec<f64>
}

//constructor to create a new shape
impl Shape{
    pub fn new(shape_type: ShapeType )-> Shape{
        let dimensions = match shape_type{
            ShapeType::Circle => vec![read_f64("Enter Radius: ")],
            ShapeType::Rectangle => vec![read_f64("Enter Length: "), read_f64("Enter width: ")],
            ShapeType::Triangle => vec![read_f64("Enter side a: "),read_f64("Enter side b: "),read_f64("Enter side c: ")],
        };

        Shape{
            shape_type,
            dimensions
        }
    }

    pub fn area(&self)-> f64{
        match self.shape_type {
            ShapeType::Circle => 3.142 * self.dimensions[0]* self.dimensions[0],
            ShapeType::Rectangle => self.dimensions[0] * self.dimensions[1],
            ShapeType::Triangle => {

                let (a,b,c) = (self.dimensions[0], self.dimensions[1], self.dimensions[2]);
                let s = (a+b+c) /2.0;
                (s * (s-a)* (s-b)* (s-c)).sqrt()
            }
        }
    }
}