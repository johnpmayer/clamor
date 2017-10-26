
extern crate piston_window;
extern crate clamor;

use piston_window::*;

use clamor::geo;

const BLACK: [f32; 4] = [0., 0., 0., 1.];

fn main() {
    println!("Start!");

    let mut window: PistonWindow = WindowSettings::new("Clamor", [800, 600])
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .expect("OpenGL can't be instantiated");

    while let Some(event) = window.next() {
        window.draw_2d(&event, |_context, graphics| {
            clear(BLACK, graphics);
        });
    }

    println!("End!");    
}
