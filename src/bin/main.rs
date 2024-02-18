// The io module in Rust provides functionality for input and output operations,
// including reading from and writing to files,
// handling standard input and output streams and more. 
use std::io;

// Every player will have a name and a score
struct Player {
    name: String,
    score: u32,
}

// NOTE: What is a trait?

// Think of traits as a set of rules,
// or a blueprint for behavior, that types can follow

// Define a trait to represent printable objects
trait Printable {
    fn to_string(&self) -> String;
}

// Implement the Printable trait for the Player struct
impl Printable for Player {
    
    // Here the function is actually being defined
    fn to_string(&self) -> String {
        format!("{} ({})", self.name, self.score)
    }

}

// This is "collect input" function,
// will collect the user's input from the command line.

// In order to work, this function takes a single arguement "prompt",
// which is a string slice.
// This will be used to prompt the user for input.

fn collect_input<T: std::str::FromStr>(prompt: &str) -> T {

    // This loop will continue to run until the user enters valid information.
    loop {
        println!("{}", prompt);
        
        // A variable called "input" is created ahead of time,
        // to store the user's input.
        // Since the user has not entered their input yet, 
        // this variable is set to an empty String.
        // That is what "String::new()" means
        // The keyword "mut" means mutable.
        // Mutable means that the value of "input" can be changed.
        // This makes sense because when the user enters the input,
        // this will replace the empty String.
        let mut input = String::new();

        // Read the user's input and modify the variable "input",
        // without taking ownership of the variable
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        // .trim() Remove any whitespace from the input,
        // .parse() Attempt to convert the trimmed string into another type.
        // In this context, type T
        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => continue,
        }

    }

}

fn collect_players() -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();
    let mut num_players = 0;

    loop {
        num_players = collect_input::<u32>("How many players (≥ 2)?");
        if num_players < 2 {
            println!("Invalid. Please try again");
            continue;
        } 

        break;
    }
    
    for i in 1..=num_players {
        let name:String = collect_input(format!("Player {} name:", i).as_str());
        players.push(Player {name, score: 0});
    }
    players

}


// NOTE: Ownership and references in Rust

// The default behavior of Rust,
// is that functions and methods,
// take ownership of variables that are passed into them.

// To override this behavior, you need to explicitly tell Rust, 
// that you only want to reference the variable,
// in the task that you are trying to perform.

// 1. .read_line(input) means that .read_line,
// will take ownership of the variable.

// 2. .read_line(&input) means that .read_line,
// will use a read-only copy of the variable.

// 3.  .read_line(&mut input) means that .read_line,
// will be allowed to modify the variable (mut),
// BUT the ownership of the variable will NOT be transferred to .readline

fn main() {
    
}
