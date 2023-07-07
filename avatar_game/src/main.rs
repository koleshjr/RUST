//Avatar game inspired by cyndie kamau

//libraries
use std::io; //prelude, if a type you want is not in the prelude you have to bring that type into scope explicitly using the "use" statement

//helpers
fn welcome() {
    println!("Hey there ! What is your name?");

    let mut your_name: String= String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut your_name)

        /*returns a  result value, either Ok or Err, Ok indicates Operation was successful, Err failed
        if "Ok", you will get back the value the user typed if ERR you will get an error messages that you defined in the expect
        if you dont call expect you will get a warning
        */
        .expect("Failed to read line"); 
    println!("Welcome to the game {your_name}");

    

    
}

fn choose_sanurai() {
    let samurais: Vec<&str>  = vec!["Ada", "Vita", "Uzi", "Doa"];
    let samurais_ages: Vec<i32> = vec![30, 25, 19, 27];
    let about_samurais: Vec<&str> = vec![

        r#"Ada is the squad leader.She is a fierce fighter known as the queen of death"#,
        r#"Vita was once the squad leader , but got kicked out. She is now a lone wolf and a legendary mercenary"#,
        r#"Uzi grew up, trained on the streets, she is called the queen of the underworld"#,
        r#"Doa lives and breathesin the shadows. She strikes when no one is watching"#,
    ];

    loop{
        println!();
        println!("There are 4 samurai warriors currently. Choose one to learn about them");
        println! ("Type exit to quit");

        for (index, samurai) in samurais.iter().enumerate(){
            println! ("{}, {samurai}", index + 1);
        }

        let mut your_choice = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut your_choice).expect("Failed to read line");
        if your_choice.trim() == "exit"{
            println! ("See you again, budddy!");
            break;
        }

        match your_choice.trim().parse::<usize>(){
            Ok(n) if n >0 && n<= samurais.len()=> {
                println! ("You chose {}: Age {}, About: {}", samurais[n-1], samurais_ages[n-1], about_samurais[n-1]);
            },

            _ => println!("Invalid choice! Please enter a number between 1 and {}", samurais.len())
        }
    }

    
}


fn main() {
    welcome();
    choose_sanurai();

}
