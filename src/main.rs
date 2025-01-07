mod lab;

use lab::{Lab, generate_main_path, generate_dead_end_paths, find_shortest_path, print_labyrinth_with_path, clear_terminal};
use std::collections::HashSet;

fn main() {
    // Create a new labyrinth of size 20x20
    let size = 20;
    let mut labyrinth = Lab::new(size);

    // Define entrance and exit
    let entrance = 0; // Top-left corner
    let exit = size * size - 1; // Bottom-right corner

    // Fill the labyrinth with walls and paths
    generate_main_path(&mut labyrinth, entrance, exit);

    // Generate dead-end paths
    generate_dead_end_paths(&mut labyrinth, entrance, exit);

    // Find the shortest path from entrance to exit
    let shortest_path = find_shortest_path(&labyrinth, entrance, exit);

    // Print the labyrinth with the shortest path highlighted
    clear_terminal();
    print_labyrinth_with_path(&labyrinth, &HashSet::new(), entrance, exit, None, shortest_path.as_ref());
}