extern crate specs;
#[macro_use] extern crate specs_derive;

use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[component(VecStorage)]
struct Position {
    x: f32,
    y: f32
}

#[derive(Component, Debug)]
#[component(VecStorage)]
struct Velocity {
    x: f32,
    y: f32,
}

fn main() {
    println!("Hello, world!");
}
