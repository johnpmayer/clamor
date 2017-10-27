
extern crate piston_window;

use piston_window::*;

const BLACK: [f32; 4] = [0., 0., 0., 1.];

fn main() {
    println!("Start!");

    let mut window: PistonWindow = WindowSettings::new("Clamor", [800, 600])
        .exit_on_esc(true)
        .vsync(true)
        .opengl(OpenGL::V3_2)
        .build()
        .expect("OpenGL can't be instantiated");

    while let Some(event) = window.next() {
        window.draw_3d(&event, |window| {
            window.encoder.clear(&window.output_color, BLACK);
        });
    }

    println!("End!");    
}
