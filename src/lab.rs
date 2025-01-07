use rand::Rng;
use std::collections::{HashSet, VecDeque};
use std::process::Command;

#[derive(Clone)]
pub struct LabTile {
    walls: [bool; 4], // [top, right, bottom, left]
}

#[derive(Clone)]
pub struct Lab {
    tiles: Vec<LabTile>,
    size: usize,
}

impl Lab {
    pub fn new(size: usize) -> Lab {
        Lab {
            tiles: vec![LabTile { walls: [true; 4] }; size * size],
            size,
        }
    }

    pub fn print(&self) {
        for y in 0..self.size {
            // Print the top walls of the row
            for x in 0..self.size {
                let tile = &self.tiles[y * self.size + x];
                print!("+");
                if tile.walls[0] {
                    print!("---"); // Top wall
                } else {
                    print!("   "); // No top wall
                }
            }
            println!("+");

            // Print the left walls and the cell content
            for x in 0..self.size {
                let tile = &self.tiles[y * self.size + x];
                if tile.walls[3] {
                    print!("|"); // Left wall
                } else {
                    print!(" "); // No left wall
                }
                print!("   "); // Cell content (empty)
            }
            // Print the rightmost wall
            println!("|");
        }

        // Print the bottom border of the last row
        for _ in 0..self.size {
            print!("+---");
        }
        println!("+");
    }
}

fn print_labyrinth_with_path(lab: &Lab, visited: &HashSet<usize>, entrance: usize, exit: usize) {
    let size = lab.size;
    for y in 0..size {
        // Print the top walls of the row
        for x in 0..size {
            let tile = &lab.tiles[y * size + x];
            print!("+");
            if tile.walls[0] {
                print!("---"); // Top wall
            } else {
                print!("   "); // No top wall
            }
        }
        println!("+");

        // Print the left walls and the cell content
        for x in 0..size {
            let tile = &lab.tiles[y * size + x];
            if tile.walls[3] {
                print!("|"); // Left wall
            } else {
                print!(" "); // No left wall
            }

            let cell = y * size + x;

            // Highlight entrance (green)
            if cell == entrance {
                print!("\x1b[32m E \x1b[0m"); // Green "E" for entrance
            }
            // Highlight exit (red)
            else if cell == exit {
                print!("\x1b[31m X \x1b[0m"); // Red "X" for exit
            }
            // Highlight visited cells (blue)
            else if visited.contains(&cell) {
                print!("\x1b[34m · \x1b[0m"); // Blue dot for visited cells
            } else {
                print!("   "); // Empty cell
            }
        }
        // Print the rightmost wall
        println!("|");
    }

    // Print the bottom border of the last row
    for _ in 0..size {
        print!("+---");
    }
    println!("+");
}

fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/c", "cls"])
            .status()
            .expect("Failed to clear terminal");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear terminal");
    }
}

pub fn fill_labyrinth(lab: &mut Lab) {
    let size = lab.size;
    let mut rng = rand::thread_rng();

    // Create an entrance (remove top wall of a random cell in the first row)
    let entrance_cell = rng.gen_range(0..size);
    lab.tiles[entrance_cell].walls[0] = false; // Remove top wall

    // Create an exit (remove bottom wall of a random cell in the last row)
    let exit_cell = (size - 1) * size + rng.gen_range(0..size);
    lab.tiles[exit_cell].walls[2] = false; // Remove bottom wall
    println!("Entrance: ({}, {})", entrance_cell % size, entrance_cell / size);
    println!("Exit: ({}, {})", exit_cell % size, exit_cell / size);

    // Generate the main path from entrance to exit
    generate_main_path(lab, entrance_cell, exit_cell);

    // Generate dead-end paths
    // generate_dead_end_paths(lab, entrance_cell, exit_cell);
}

fn generate_main_path(lab: &mut Lab, entrance: usize, exit: usize) {
    let mut rng = rand::thread_rng();
    let mut visited = HashSet::new();

    // Start from the entrance
    let mut current = entrance;
    visited.insert(entrance);

    while !visited.contains(&exit) {
        let (x, y) = (current % lab.size, current / lab.size);

        // Collect all valid neighbors (up, down, left, right)
        let mut neighbors = Vec::new();

        // Check top neighbor
        if y > 0 && !visited.contains(&(current - lab.size)) {
            neighbors.push((current - lab.size, 0, 2)); // Top -> Bottom
        }
        // Check right neighbor
        if x < lab.size - 1 && !visited.contains(&(current + 1)) {
            neighbors.push((current + 1, 1, 3)); // Right -> Left
        }
        // Check bottom neighbor
        if y < lab.size - 1 && !visited.contains(&(current + lab.size)) {
            neighbors.push((current + lab.size, 2, 0)); // Bottom -> Top
        }
        // Check left neighbor
        if x > 0 && !visited.contains(&(current - 1)) {
            neighbors.push((current - 1, 3, 1)); // Left -> Right
        }

        if neighbors.is_empty() {
            // If no valid neighbors, find the cell closest to the exit and restart from there
            let (exit_x, exit_y) = (exit % lab.size, exit / lab.size);
            let closest_cell = *visited
                .iter()
                .min_by_key(|&&cell| {
                    let (cell_x, cell_y) = (cell % lab.size, cell / lab.size);
                    ((cell_x as isize - exit_x as isize).abs() + (cell_y as isize - exit_y as isize).abs()) as usize
                })
                .unwrap();
            current = closest_cell;
            continue;
        }

        // Randomly choose a neighbor
        let (next_cell, wall_idx, opposite_wall_idx) = neighbors[rng.gen_range(0..neighbors.len())];

        // Remove the wall between current and next_cell
        lab.tiles[current].walls[wall_idx] = false;
        lab.tiles[next_cell].walls[opposite_wall_idx] = false;

        // Move to the next cell
        current = next_cell;
        visited.insert(current);

        // Clear the terminal and print the current state of the labyrinth
        clear_terminal();
        print_labyrinth_with_path(&lab, &visited, entrance, exit);
        std::thread::sleep(std::time::Duration::from_millis(10)); // Add a small delay for visualization
    }
}

fn generate_dead_end_paths(lab: &mut Lab, entrance: usize, exit: usize) {
    let mut rng = rand::thread_rng();
    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    // Start from the entrance
    stack.push(entrance);
    visited.insert(entrance);

    while let Some(current) = stack.pop() {
        let (x, y) = (current % lab.size, current / lab.size);
        let mut neighbors = Vec::new();

        // Check top neighbor
        if y > 0 && !visited.contains(&(current - lab.size)) {
            neighbors.push((current - lab.size, 0, 2)); // Top -> Bottom
        }
        // Check right neighbor
        if x < lab.size - 1 && !visited.contains(&(current + 1)) {
            neighbors.push((current + 1, 1, 3)); // Right -> Left
        }
        // Check bottom neighbor
        if y < lab.size - 1 && !visited.contains(&(current + lab.size)) {
            neighbors.push((current + lab.size, 2, 0)); // Bottom -> Top
        }
        // Check left neighbor
        if x > 0 && !visited.contains(&(current - 1)) {
            neighbors.push((current - 1, 3, 1)); // Left -> Right
        }

        if neighbors.is_empty() {
            continue;
        }

        // Pick a random neighbor
        let (next_cell, wall_idx, opposite_wall_idx) = neighbors[rng.gen_range(0..neighbors.len())];

        // Remove the wall between current and next_cell
        lab.tiles[current].walls[wall_idx] = false;
        lab.tiles[next_cell].walls[opposite_wall_idx] = false;

        // Move to the next cell
        stack.push(next_cell);
        visited.insert(next_cell);
    }
}