use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::nalgebra::Translation3;
use std::path::Path;
use crate::lab::Lab;

pub fn render_labyrinth_3d(lab: &Lab) {
    let mut window = Window::new("Labyrinth 3D");
    window.set_light(Light::StickToCamera);

    let wall_height = 1.0;
    let wall_size = 1.0;
    let center_offset = lab.size as f32 / 2.0;

    // Add floor
    let mut floor = window.add_cube(lab.size as f32, 0.1, lab.size as f32);
    floor.set_color(0.5, 0.5, 1.0); // Gray color for walls
    floor.set_local_translation(Translation3::new(-0.5, -0.5, -0.5));

    for y in 0..lab.size {
        for x in 0..lab.size {
            let cell = y * lab.size + x;
            let tile = &lab.tiles[cell];

            // Render walls
            if tile.get_wall(0) { // if top wall
                // Top wall
                let mut wall = window.add_cube(wall_size, wall_height, 0.1);
                wall.set_texture_from_file(Path::new("resources/texture.jpg"),"tex"); // Apply texture
                wall.set_local_translation(Translation3::new(x as f32 - center_offset, 0.0, y as f32 - center_offset - 0.5));
            }
            if tile.get_wall(1) { // if right wall
                // Right wall
                let mut wall = window.add_cube(0.1, wall_height, wall_size);
                wall.set_texture_from_file(Path::new("resources/texture.jpg"),"tex"); // Apply texture
                wall.set_local_translation(Translation3::new(x as f32 - center_offset + 0.5, 0.0, y as f32 - center_offset));
            }
            if tile.get_wall(2) { // if bottom wall
                // Bottom wall
                let mut wall = window.add_cube(wall_size, wall_height, 0.1);
                wall.set_texture_from_file(Path::new("resources/texture.jpg"),"tex"); // Apply texture
                wall.set_local_translation(Translation3::new(x as f32 - center_offset, 0.0, y as f32 - center_offset + 0.5));
            }
            if tile.get_wall(3) { // if left wall
                // Left wall
                let mut wall = window.add_cube(0.1, wall_height, wall_size);
                wall.set_texture_from_file(Path::new("resources/texture.jpg"),"tex"); // Apply texture
                wall.set_local_translation(Translation3::new(x as f32 - center_offset - 0.5, 0.0, y as f32 - center_offset));
            }
        }
    }

    let mut entrance_cube = window.add_cube(wall_size, wall_height, wall_size);
    entrance_cube.set_color(0.0, 1.0, 0.0); // Green for entrance
    entrance_cube.set_local_translation(Translation3::new(-center_offset, 0.0, -center_offset));

    let mut exit_cube = window.add_cube(wall_size, wall_height, wall_size);
    exit_cube.set_color(1.0, 0.0, 0.0); // Red for exit
    exit_cube.set_local_translation(Translation3::new(center_offset - 1.0, 0.0, center_offset - 1.0));

    // Render the 3D labyrinth
    while window.render() {
        // You can add camera controls or other interactions here
    }
}