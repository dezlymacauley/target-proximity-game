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

fn main() {
    
}
