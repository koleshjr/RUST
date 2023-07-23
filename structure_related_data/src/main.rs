// here we are using structs to structure related data

struct Person {
    name: String,
    age: u32,
    city: String
}

impl Person{
    fn new(name: &str, age: u32, city: &str)-> Person{
        Person{
            name: name.to_string(), //String::from(name),
            age,
            city: city.to_string()
        }
    }

    fn print_info(&self){
        println! ("Name {}", self.name);
        println! ("Age {}", self.age);
        println! ("City {}", self.city);
    }
}

fn main(){
    //using the constructor
    let person_1 = Person::new("stephen", 22, "Ongata Rongai");
    //using instantiation
    let person_2 = Person{
        name: "Caroline".to_string(),
        age: 21,
        city: "Nairobi".to_string()
    };

    // print information
    person_1.print_info();
    person_2.print_info();
}