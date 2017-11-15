
extern crate clamor;
extern crate piston_window;
extern crate sdl2_window;
extern crate shader_version;
extern crate camera_controllers;
extern crate nalgebra;
extern crate vecmath;

#[macro_use]
extern crate gfx;

use piston_window::*;
use clamor::geodesic::Net;
use gfx::traits::*;
use shader_version::Shaders;
use shader_version::glsl::GLSL;
use camera_controllers::{
    FirstPersonSettings,
    FirstPerson,
    CameraPerspective,
    model_view_projection
};

use nalgebra::core::{Vector3};

gfx_vertex_struct!( Vertex {
    a_pos: [f32; 4] = "a_pos",
    a_color: f32 = "a_color",
});

impl Vertex {
    fn new(pos: [f32; 3], light_level: f32) -> Vertex {

        Vertex {
            a_pos: [pos[0], pos[1], pos[2], 1.],
            a_color: light_level
        }
    }
}

gfx_pipeline!( pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]> = "u_model_view_proj",
    out_color: gfx::RenderTarget<::gfx::format::Srgba8> = "o_Color",
    out_depth: gfx::DepthTarget<::gfx::format::DepthStencil> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
});


fn main() {
    println!("Start!");

    let world = Net::build_subdivided(4);
    let faces = world.faces();
    
    println!("Num of faces: {}", faces.len());

    let mut vertex_data: Vec<Vertex> = Vec::new();
    let mut index_data = Vec::new();
    let mut index_counter: u16 = 0;
    for face in faces {

        let normal = (face[0] + face[1] + face[2]).normalize();

        let light_level = (3. + normal.dot(&Vector3::new(1.0,0.0,0.0))) / 6.;

        for vertex_index in 0..3 {
            let vertex_slice = face[vertex_index].as_slice();
            vertex_data.push(Vertex::new([
                vertex_slice[0],
                vertex_slice[1],
                vertex_slice[2],
            ], light_level));
            index_data.push(index_counter);
            index_counter += 1;
        }
    }

    println!("Num vertices: {} {}", vertex_data.len(), index_data.len());

    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new("Clamor", [800, 600])
        .exit_on_esc(true)
        .vsync(true)
        .opengl(opengl)
        .build()
        .expect("OpenGL can't be instantiated");
    // window.set_capture_cursor(true);

    let ref mut factory = window.factory.clone();

    let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, index_data.as_slice());

    let glsl = opengl.to_glsl();

    println!("Creating pipeline?");

    let pso = factory.create_pipeline_simple(
        Shaders::new()
            .set(GLSL::V1_50, include_str!("../assets/cube_150.glslv"))
            .get(glsl).unwrap().as_bytes(),
        Shaders::new()
            .set(GLSL::V1_50, include_str!("../assets/cube_150.glslf"))
            .get(glsl).unwrap().as_bytes(),
        pipe::new()
    ).unwrap();

    println!("Created pso");

    let get_projection = |w: &PistonWindow| {
        let draw_size = w.window.draw_size();
        CameraPerspective {
            fov: 90.0, near_clip: 0.1, far_clip: 1000.0,
            aspect_ratio: (draw_size.width as f32) / (draw_size.height as f32)
        }.projection()
    };

    let model = vecmath::mat4_id();
    let projection = get_projection(&window);
    let mut first_person = FirstPerson::new(
        [0.5, 0.5, 4.0],
        FirstPersonSettings::keyboard_wasd()
    );

    let mut data = pipe::Data {
            vbuf: vbuf.clone(),
            u_model_view_proj: [[0.0; 4]; 4],
            // t_color: (texture_view, factory.create_sampler(sinfo)),
            out_color: window.output_color.clone(),
            out_depth: window.output_stencil.clone(),
        };

    while let Some(event) = window.next() {
        first_person.event(&event);

        window.draw_3d(&event, |window| {
            let args = event.render_args().unwrap();

            window.encoder.clear(&window.output_color, [0.3, 0.3, 0.3, 1.0]);
            window.encoder.clear_depth(&window.output_stencil, 1.0);

            data.u_model_view_proj = model_view_projection(
                model,
                first_person.camera(args.ext_dt).orthogonal(),
                projection
            );
            window.encoder.draw(&slice, &pso, &data);
            
        });
    }

    println!("End!");    
}
