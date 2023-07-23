//telling rust this is the entry point of our program
#[allow(dead_code)]
mod product;
mod inventory;

use product::Product;
use inventory::Inventory;

fn main(){
    let mut inventory = Inventory::new();

    let product_1 = Product::new("Apple", 0.50, 100);
    let product_2 = Product::new("Banana", 0.46, 70);
    let product_3 = Product::new("Apple", 0.20, 150);

    inventory.add_product(product_1);
    inventory.add_product(product_2);
    inventory.add_product(product_3);

    inventory.print_inventory();

    let search_product = "Apple";

    match inventory.get_product(search_product){
        Some(product) => println! ("Product found: {:?}", product),
        None => println! ("Product not found"),
    }
}