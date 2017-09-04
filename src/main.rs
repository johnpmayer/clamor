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

use specs::{ReadStorage, System};

struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}

use specs::World;

use specs::RunNow;

fn main() {
    println!("Start!");

    let mut world = World::new();
    world.register::<Position>();

    let ball = world.create_entity().with(Position { x: 4.0, y: 7.0 }).build();

    let mut hello_world = HelloWorld;
    hello_world.run_now(&world.res);

    println!("End!");    
}
