//Declare that this file is the main entry point to the program

mod shapes;
mod utils;

use shapes::{Shape, ShapeType};
use utils::get_user_input;

fn main(){
    println! ("Shapes Project");

    let shape_type = get_user_input();
    let shape = Shape::new(shape_type);

    println! ("Area: {:.2}", shape.area());
}