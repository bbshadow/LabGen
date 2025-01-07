use rand::Rng;
use std::collections::HashSet;

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