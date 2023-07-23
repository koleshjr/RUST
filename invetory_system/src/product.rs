#[derive(Debug)]
//Define a struct representing a product
pub struct Product{
    pub name: String,
    price: f64,
    quantity: u32,
}

impl Product{
    //constructor to create a new product 
    pub fn new(name: &str, price: f64, quantity: u32)-> Product{
        Product{
            name: String::from(name),
            price,
            quantity
        }
    }


}