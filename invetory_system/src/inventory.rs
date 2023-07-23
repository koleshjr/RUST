

use crate::product::Product;// use the Peroduct Struct from the product module
use std::collections::HashMap;



// define a struct representing an inventory
pub struct Inventory{
    products: HashMap<String, Product>,


}

impl Inventory {
    pub fn new()-> Inventory{
        Inventory{
            products: HashMap::new(),
        }
    }

    
    //Method to add a product in the Inventory 

    pub fn add_product(&mut self, product: Product){
        self.products.insert(product.name.clone(), product);
        
    }    

    //method to retrieve a product from the inventory by name
    pub fn get_product(&self, name: &str) -> Option<&Product> {
        self.products.get(name)
    }

    //Method to print the entire inventory

    pub fn print_inventory(&self){
        println! ("Inventory: ");

        for (_, product) in &self.products{
            println! ("{:?}", product);
        }
    }
}