mod lab;
mod render_3d;

use lab::{Lab, generate_main_path, generate_dead_end_paths};
use render_3d::render_labyrinth_3d;

fn main() {
    // Create a new labyrinth of size 20x20
    let size = 20;
    let mut labyrinth = Lab::new(size);

    // Define entrance and exit positions
    let entrance = 0; // Top-left corner
    let exit = size * size - 1; // Bottom-right corner

    // Fill the labyrinth with walls and paths
    generate_main_path(&mut labyrinth, entrance, exit);

    // Generate dead-end paths
    generate_dead_end_paths(&mut labyrinth);

    // Render the labyrinth in 3D
    render_labyrinth_3d(&labyrinth);
}