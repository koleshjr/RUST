use crate::ecosystem::animal::{Animal};

pub struct Habitat {
    pub name: String,
    pub animals: Vec<Animal>
}

impl Habitat {
    pub fn new(name: &str)-> Self{
        Habitat {
            name: name.to_string(),
            animals: Vec::new()
        }
    }

    //I guess based on what I have experienced so far , we clone when we post , we reference when we get
    pub fn add_animal(&mut self, animal: Animal){
        self.animals.push(animal);
    }

    pub fn list_animals(&self){
        println! ("Animals in {}: ", self.name);
        for animal in &self.animals{
            println! ("{} ({:?})", animal.name, animal.animal_type);
        }
    }
}