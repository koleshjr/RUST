#[derive(Debug)]
pub enum AnimalType{
    Carnivore,
    Herbivore,
    Omnivore,
}

pub struct Animal{
    pub name: String,
    pub animal_type: AnimalType
}

impl Animal{
    pub fn new(name: &str, animal_type: AnimalType)-> Self{
        Animal{
            name: name.to_string(),
            animal_type,
        }
    }

    pub fn eat(&self){
        match self.animal_type {
            AnimalType::Carnivore => println! {"{} is a carnivore and eats meat", self.name},
            AnimalType::Herbivore => println! {"{} is a Herbivore and eats plants", self.name},
            AnimalType::Omnivore => println! {"{} is a Omnivore and eats both plants and meat", self.name},

        }
    }
}