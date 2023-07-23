mod ecosystem;

use ecosystem::animal::{Animal, AnimalType};
use ecosystem::habitat::Habitat;

fn main(){
    
    let lion = Animal::new("Lion", AnimalType::Carnivore);
    let deer = Animal::new("Deer", AnimalType::Herbivore);
    let rabbit = Animal::new("Rabbit", AnimalType::Herbivore);
    let bear = Animal::new("Bear", AnimalType::Omnivore);

    let mut forest = Habitat::new("Forest", ); //we have not instantiated the animals vec yet

    forest.add_animal(lion);
    forest.add_animal(deer);
    forest.add_animal(rabbit);
    forest.add_animal(bear);

    forest.list_animals();

    //simulate animal interactions

    for animal in &forest.animals{
        animal.eat();
    }


}