mod lab;

use lab::{Lab, fill_labyrinth};

fn main() {
    // Create a new labyrinth of size 20x20
    let mut labyrinth = Lab::new(20);

    // Fill the labyrinth with walls and paths
    fill_labyrinth(&mut labyrinth);

    // Print the labyrinth to the console
    // labyrinth.print();
}