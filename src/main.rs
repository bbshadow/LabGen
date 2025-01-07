extern crate kiss3d;
extern crate nalgebra as na;
extern crate rand; // Add this to your Cargo.toml dependencies

mod lab;


use lab::{Lab, LabTile};
use na::{Vector3, UnitQuaternion, Translation, U3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use rand::Rng; // Import the random number generator trait

struct Cube {
    object: kiss3d::scene::SceneNode,
    position: na::Translation<f32, U3>,
    color: (f32, f32, f32),
}

struct CubeSet {
    cubes: Vec<Cube>,
}

impl Cube {
    fn new(window: &mut Window, position: na::Translation<f32, U3>, color: (f32, f32, f32)) -> Self {
        let mut cube = window.add_cube(1.0, 1.0, 1.0);
        cube.set_color(color.0, color.1, color.2);
        
        Cube {
            object: cube,
            position,
            color,
        }
    }
}

impl CubeSet {
    fn new(num_cubes: usize, window: &mut Window) -> Self {
        let mut rng = rand::thread_rng();
        let mut cubes = Vec::new();

        for _ in 0..num_cubes {
            let position = na::Translation::<f32, U3>::new(
                rng.gen_range(-10.0..10.0), // Random position between -10 and 10
                rng.gen_range(-10.0..10.0),
                rng.gen_range(-10.0..10.0),
            );

            let color = (
                rng.gen_range(0.0..1.0), // Random color component (red)
                rng.gen_range(0.0..1.0), // Random color component (green)
                rng.gen_range(0.0..1.0), // Random color component (blue)
            );

            cubes.push(Cube::new(window, position, color));
        }

        CubeSet { cubes }
    }

    fn render(&mut self) {
        for cube in &mut self.cubes {
            cube.object.set_local_translation(cube.position);
        }
    }
}

fn main() {
    let mut labyrinth = Lab::new(20);

    lab::fill_labyrinth(&mut labyrinth);
    lab::print_labyrinth(&labyrinth);

}
