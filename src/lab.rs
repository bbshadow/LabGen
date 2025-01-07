use std::collections::{HashSet, VecDeque};
use std::process::Command;
use rand::Rng;

#[derive(Clone)]
pub struct LabTile {
    walls: [bool; 4], // [top, right, bottom, left]
}

#[derive(Clone)]
pub struct Lab {
    pub tiles: Vec<LabTile>,
    pub size: usize,
}

impl Lab {
    pub fn new(size: usize) -> Lab {
        Lab {
            tiles: vec![LabTile { walls: [true; 4] }; size * size],
            size,
        }
    }
}

pub fn clear_terminal() {
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

pub fn generate_main_path(lab: &mut Lab, entrance: usize, exit: usize) {
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
        print_labyrinth_with_path(&lab, &visited, entrance, exit, Some((current, 34)), None); // Use blue for the main path
        std::thread::sleep(std::time::Duration::from_millis(30)); // Add a small delay for visualization
    }
}

pub fn generate_dead_end_paths(lab: &mut Lab, entrance: usize, exit: usize) {
    let mut rng = rand::thread_rng();
    let mut visited = HashSet::new();

    // Start with the main path tiles
    visited.extend(lab.tiles.iter().enumerate().filter(|(_, tile)| !tile.walls.iter().all(|&w| w)).map(|(i, _)| i));

    // Iterate over all tiles to ensure they are covered
    for cell in 0..lab.tiles.len() {
        if visited.contains(&cell) {
            continue; // Skip already visited tiles
        }

        // Find the closest tile on the main path to start the dead-end path
        let (cell_x, cell_y) = (cell % lab.size, cell / lab.size);
        let closest_main_path_cell = *visited
            .iter()
            .min_by_key(|&&main_cell| {
                let (main_x, main_y) = (main_cell % lab.size, main_cell / lab.size);
                ((main_x as isize - cell_x as isize).abs() + (main_y as isize - cell_y as isize).abs()) as usize
            })
            .unwrap();

        // Start the dead-end path from the closest main path cell
        let mut current = closest_main_path_cell;
        visited.insert(current);

        // Generate a dead-end path from this tile
        while let Some((next_cell, wall_idx, opposite_wall_idx)) = {
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
                None // Dead end reached
            } else {
                // Randomly choose a neighbor
                Some(neighbors[rng.gen_range(0..neighbors.len())])
            }
        } {
            // Remove the wall between current and next_cell
            lab.tiles[current].walls[wall_idx] = false;
            lab.tiles[next_cell].walls[opposite_wall_idx] = false;

            // Move to the next cell
            current = next_cell;
            visited.insert(current);

            // Clear the terminal and print the current state of the labyrinth
            clear_terminal();
            print_labyrinth_with_path(&lab, &visited, entrance, exit, Some((current, 33)), None); // Use yellow for dead-end paths
            std::thread::sleep(std::time::Duration::from_millis(30)); // Add a small delay for visualization
        }
    }
}

pub fn find_shortest_path(lab: &Lab, entrance: usize, exit: usize) -> Option<Vec<usize>> {
    let size = lab.size;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent = vec![None; size * size]; // To reconstruct the path

    queue.push_back(entrance);
    visited.insert(entrance);

    while let Some(current) = queue.pop_front() {
        if current == exit {
            // Reconstruct the path from exit to entrance
            let mut path = Vec::new();
            let mut node = exit;
            while let Some(prev) = parent[node] {
                path.push(node);
                node = prev;
            }
            path.push(entrance);
            path.reverse();
            return Some(path);
        }

        let (x, y) = (current % size, current / size);

        // Check top neighbor
        if y > 0 && !lab.tiles[current].walls[0] {
            let neighbor = current - size;
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                parent[neighbor] = Some(current);
                queue.push_back(neighbor);
            }
        }
        // Check right neighbor
        if x < size - 1 && !lab.tiles[current].walls[1] {
            let neighbor = current + 1;
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                parent[neighbor] = Some(current);
                queue.push_back(neighbor);
            }
        }
        // Check bottom neighbor
        if y < size - 1 && !lab.tiles[current].walls[2] {
            let neighbor = current + size;
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                parent[neighbor] = Some(current);
                queue.push_back(neighbor);
            }
        }
        // Check left neighbor
        if x > 0 && !lab.tiles[current].walls[3] {
            let neighbor = current - 1;
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                parent[neighbor] = Some(current);
                queue.push_back(neighbor);
            }
        }
    }

    None // No path found
}

pub fn print_labyrinth_with_path(lab: &Lab, visited: &HashSet<usize>, entrance: usize, exit: usize, current_cell: Option<(usize, u8)>, shortest_path: Option<&Vec<usize>>) {
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
            // Highlight shortest path (magenta)
            else if let Some(path) = shortest_path {
                if path.contains(&cell) {
                    print!("\x1b[35m · \x1b[0m"); // Magenta dot for shortest path
                } else if let Some((current, color)) = current_cell {
                    if cell == current {
                        print!("\x1b[{}m · \x1b[0m", color); // Use the provided color for the current cell
                    } else if visited.contains(&cell) {
                        print!("\x1b[34m · \x1b[0m"); // Blue dot for visited cells
                    } else {
                        print!("   "); // Empty cell
                    }
                } else if visited.contains(&cell) {
                    print!("\x1b[34m · \x1b[0m"); // Blue dot for visited cells
                } else {
                    print!("   "); // Empty cell
                }
            } else if let Some((current, color)) = current_cell {
                if cell == current {
                    print!("\x1b[{}m · \x1b[0m", color); // Use the provided color for the current cell
                } else if visited.contains(&cell) {
                    print!("\x1b[34m · \x1b[0m"); // Blue dot for visited cells
                } else {
                    print!("   "); // Empty cell
                }
            } else if visited.contains(&cell) {
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