#[derive(Debug, Clone)] // Implement the Clone trait for CharacterClass

// Enum to represent different character classes
enum CharacterClass {
    Warrior,
    Mage,
    Archer,
}

// Struct to represent a Character
struct Character {
    name: String,
    class: CharacterClass,
    health: i32,
    attack: i32,
}


impl Character {
    // Method to create a new character: constructor - doesn't take self
    fn new(name: &str, class: CharacterClass, health: i32, attack: i32) -> Character {
        Character {
            name: String::from(name),
            class,
            health,
            attack,
        }
    }

    // Method to attack an enemy and reduce their health
    fn attack_enemy(&self, enemy: &mut Enemy) {
        println!(
            "{} attacks {} with {} damage!",
            self.name, enemy.name, self.attack
        );

        enemy.health -= self.attack;
    }
}



struct CharacterAttributes {
    health: i32,
    attack: i32
}

impl CharacterAttributes{
    fn get_attributes(class: &CharacterClass)-> CharacterAttributes{
        match class {
            CharacterClass::Warrior => CharacterAttributes{health: 100, attack: 20},
            CharacterClass::Mage => CharacterAttributes{health: 50, attack: 50},
            CharacterClass::Archer => CharacterAttributes{health: 40, attack: 60},
        }
    }
}

struct Enemy {
    name: String,
    health: i32,
    attack: i32,
}

impl Enemy {
    // Method to create a new enemy: Constructor
    fn new(name: &str, health: i32, attack: i32) -> Enemy {
        Enemy {
            name: String::from(name),
            health,
            attack,
        }
    }

    // Method to attack the player character and reduce their health
    fn attack_player(&self, player: &mut Character) {
        println!(
            "{} attacks {} with {} damage!",
            self.name, player.name, self.attack
        );

        player.health -= self.attack;
    }
}

fn format_class(class: &CharacterClass) -> &'static str {
    // Work with a borrowed reference to avoid ownership errors instead of the string
    match class {
        CharacterClass::Warrior => "Warrior",
        CharacterClass::Mage => "Mage",
        CharacterClass::Archer => "Archer",
    }
}



fn main() {
    // Create a Player Character
    let list: Vec<(&str, &CharacterClass)> = vec![
        ("Kolesh", &CharacterClass::Warrior),
        ("Kolesh", &CharacterClass::Mage),
        ("Kolesh", &CharacterClass::Archer),
    ];

    for &(name, char_class) in &list {
        println!("The {} class playing is: {:?}", name, char_class);
        println!();
        let attributes = CharacterAttributes::get_attributes(char_class);
        let mut player = Character::new(name, char_class.clone(), attributes.health, attributes.attack);

        // Create an enemy
        let mut enemy = Enemy::new("keziah", 50, 10);

        // Game loop
        loop {
            // Display enemy and player stats
            println!(
                "Player: {} ({}) - Health: {}, Attack: {}",
                player.name,
                format_class(&player.class),
                player.health,
                player.attack
            );

            println!(
                "Enemy: {} ({}) - Health: {}, Attack: {}",
                enemy.name, "Goblin", enemy.health, enemy.attack
            );

            // Players turn to attack
            player.attack_enemy(&mut enemy);

            // Check if the enemy is defeated
            if enemy.health <= 0 {
                println!("You defeated the {}!", enemy.name);
                break;
            }

            // Enemy's turn to attack
            enemy.attack_player(&mut player);

            // Check if the player is defeated
            if player.health <= 0 {
                println!("Game over! The {} defeated you.", enemy.name);
                break;
            }
        }
    }
}
