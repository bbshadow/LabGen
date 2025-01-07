use rand::Rng;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct LabTile {
    walls: [bool; 4],
    neighbors: Vec<Option<LabTile>>
}

impl LabTile {
    pub fn new() -> LabTile {
        LabTile {
            walls: [false; 4],
            neighbors: Vec::new()
        }
    }
}

#[derive(Clone)]
pub struct Lab {
    tiles: Vec<LabTile>
}

impl Lab {
    pub fn new(size: usize) -> Lab {
        Lab {
            tiles: (0..(size*size)).map(|_| LabTile::new()).collect()
        }
    }
}

pub fn fill_labyrinth(lab: &mut Lab) {
    let size = (lab.tiles.len() as f32).sqrt() as usize;
    let mut rng = rand::thread_rng();

    // Initialize all walls as present
    for tile in &mut lab.tiles {
        tile.walls = [true; 4]; // Walls are initially closed
    }

    // Start with a random cell
    let start = rng.gen_range(0..lab.tiles.len());
    let mut visited = HashSet::new();
    let mut walls_to_check = Vec::new();

    visited.insert(start);

    // Add walls of the starting cell to the list
    let (x, y) = (start % size, start / size);
    if x > 0 {
        walls_to_check.push((start, start - 1, 3)); // Left wall
    }
    if x < size - 1 {
        walls_to_check.push((start, start + 1, 1)); // Right wall
    }
    if y > 0 {
        walls_to_check.push((start, start - size, 0)); // Top wall
    }
    if y < size - 1 {
        walls_to_check.push((start, start + size, 2)); // Bottom wall
    }

    while !walls_to_check.is_empty() {
        // Pick a random wall
        let idx = rng.gen_range(0..walls_to_check.len());
        let (cell_a, cell_b, wall_idx) = walls_to_check.remove(idx);

        if !visited.contains(&cell_b) {
            // Remove the wall between cell_a and cell_b
            lab.tiles[cell_a].walls[wall_idx] = false;
            let opposite_wall = match wall_idx {
                0 => 2, // Top -> Bottom
                1 => 3, // Right -> Left
                2 => 0, // Bottom -> Top
                3 => 1, // Left -> Right
                _ => panic!("Invalid wall index"),
            };
            lab.tiles[cell_b].walls[opposite_wall] = false;

            // Mark cell_b as visited
            visited.insert(cell_b);

            // Add walls of cell_b to the list
            let (x, y) = (cell_b % size, cell_b / size);
            if x > 0 {
                walls_to_check.push((cell_b, cell_b - 1, 3)); // Left wall
            }
            if x < size - 1 {
                walls_to_check.push((cell_b, cell_b + 1, 1)); // Right wall
            }
            if y > 0 {
                walls_to_check.push((cell_b, cell_b - size, 0)); // Top wall
            }
            if y < size - 1 {
                walls_to_check.push((cell_b, cell_b + size, 2)); // Bottom wall
            }
        }
    }

    // Create an entrance (remove top wall of a random cell in the first row)
    let entrance_cell = rng.gen_range(0..size);
    lab.tiles[entrance_cell].walls[0] = false; // Remove top wall

    // Create an exit (remove bottom wall of a random cell in the last row)
    let exit_cell = (size - 1) * size + rng.gen_range(0..size);
    lab.tiles[exit_cell].walls[2] = false; // Remove bottom wall

    // Add extra walls while ensuring the entrance and exit remain connected
    let mut attempts = 0;
    while attempts < size * size {
        let cell = rng.gen_range(0..lab.tiles.len());
        let wall_idx = rng.gen_range(0..4);

        // Check if adding this wall would block the path between entrance and exit
        let mut lab_copy = lab.clone();
        lab_copy.tiles[cell].walls[wall_idx] = true;

        // Calculate the opposite cell and wall
        let opposite_cell = match wall_idx {
            0 => {
                if cell >= size {
                    cell - size // Top -> Bottom
                } else {
                    continue; // Skip if the cell is in the first row
                }
            }
            1 => {
                if cell % size < size - 1 {
                    cell + 1 // Right -> Left
                } else {
                    continue; // Skip if the cell is in the last column
                }
            }
            2 => {
                if cell < size * (size - 1) {
                    cell + size // Bottom -> Top
                } else {
                    continue; // Skip if the cell is in the last row
                }
            }
            3 => {
                if cell % size > 0 {
                    cell - 1 // Left -> Right
                } else {
                    continue; // Skip if the cell is in the first column
                }
            }
            _ => panic!("Invalid wall index"),
        };

        if opposite_cell < lab_copy.tiles.len() {
            let opposite_wall = match wall_idx {
                0 => 2, // Top -> Bottom
                1 => 3, // Right -> Left
                2 => 0, // Bottom -> Top
                3 => 1, // Left -> Right
                _ => panic!("Invalid wall index"),
            };
            lab_copy.tiles[opposite_cell].walls[opposite_wall] = true;
        }

        // Check if the entrance and exit are still connected
        if is_connected(&lab_copy, entrance_cell, exit_cell) {
            lab.tiles[cell].walls[wall_idx] = true;
        }

        attempts += 1;
    }
}

// Helper function to check if two cells are connected using BFS
fn is_connected(lab: &Lab, start: usize, end: usize) -> bool {
    let size = (lab.tiles.len() as f32).sqrt() as usize;
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);
    visited.insert(start);

    while !queue.is_empty() {
        let cell = queue.pop_front().unwrap();
        if cell == end {
            return true;
        }

        let (x, y) = (cell % size, cell / size);

        // Check neighbors
        if x > 0 && !lab.tiles[cell].walls[3] && !visited.contains(&(cell - 1)) {
            queue.push_back(cell - 1);
            visited.insert(cell - 1);
        }
        if x < size - 1 && !lab.tiles[cell].walls[1] && !visited.contains(&(cell + 1)) {
            queue.push_back(cell + 1);
            visited.insert(cell + 1);
        }
        if y > 0 && !lab.tiles[cell].walls[0] && !visited.contains(&(cell - size)) {
            queue.push_back(cell - size);
            visited.insert(cell - size);
        }
        if y < size - 1 && !lab.tiles[cell].walls[2] && !visited.contains(&(cell + size)) {
            queue.push_back(cell + size);
            visited.insert(cell + size);
        }
    }

    false
}

pub fn print_labyrinth(lab: &Lab) {
    let size = (lab.tiles.len() as f32).sqrt() as usize;

    // Print the top border, leaving a gap for the entrance
    print!("+");
    for x in 0..size {
        let tile = &lab.tiles[x];
        if tile.walls[0] {
            print!("---+"); // Top wall
        } else {
            print!("    "); // Entrance (no top wall)
        }
    }
    println!();

    for y in 0..size {
        // Print the left walls and the cell content
        print!("|");
        for x in 0..size {
            let tile = &lab.tiles[y * size + x];
            print!("   "); // Cell content (empty)
            if tile.walls[1] {
                print!("|"); // Right wall
            } else {
                print!(" "); // No right wall
            }
        }
        println!();

        // Print the bottom walls, leaving a gap for the exit
        print!("+");
        for x in 0..size {
            let tile = &lab.tiles[y * size + x];
            if tile.walls[2] {
                print!("---+"); // Bottom wall
            } else if y == size - 1 {
                print!("    "); // Exit (no bottom wall)
            } else {
                print!("   +"); // No bottom wall
            }
        }
        println!();
    }
}