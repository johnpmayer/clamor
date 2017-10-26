

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{HashSet, HashMap};
use std::f32::{self, consts};

use nalgebra::geometry::Rotation3;
use nalgebra::core::Vector3;

type NetCoordinate = (usize, usize);

#[derive(Debug)]
struct NetNode {
    coordinates: Rc<Vec<NetCoordinate>>,
    position: Vector3<f32>,
}

#[derive(Debug)]
struct Net {
    nodes: HashMap<NetCoordinate, Rc<NetNode>>,
}

impl Net {
    fn build() -> Net {
        let mut nodes: HashMap<NetCoordinate, Rc<NetNode>> = HashMap::new();

        let circle_latitude_radians: f32 = f32::atan(0.5);
        let circle_longitude_increment: f32 = consts::PI / 5.;

        {
            let mut north_pole_net_coordinates: Vec<NetCoordinate> = Vec::new();
            for i in 0..5 {
                let coord = (i, i + 1);
                north_pole_net_coordinates.push(coord);
            }
            let north_pole_position = Vector3::new(0., 0., 1.);
            let north_pole_node = Rc::new(NetNode { 
                coordinates: Rc::new(north_pole_net_coordinates.clone()), 
                position: north_pole_position 
            });
            for coord in north_pole_net_coordinates {
                nodes.insert(coord, north_pole_node.clone());
            }
        }

        {
            let mut south_pole_net_coordinates: Vec<NetCoordinate> = Vec::new();
            for i in 0..5 {
                let coord = (i + 2, i);
                south_pole_net_coordinates.push(coord);
            }
            let south_pole_position = Vector3::new(0., 0., -1.);
            let south_pole_node = Rc::new(NetNode { 
                coordinates: Rc::new(south_pole_net_coordinates.clone()), 
                position: south_pole_position 
            });
            for coord in south_pole_net_coordinates {
                nodes.insert(coord, south_pole_node.clone());
            }
        }

        for i in 0..5 {
            let coord = (i, i);
            let mut arctic_circle_coordinates = vec!(coord);
            if i == 0 {
                arctic_circle_coordinates.push((5, 5));
            }
            let mut arctic_circle_position = Vector3::new(1., 0., 0.);
            let latitude_rotation = Rotation3::new(Vector3::new(0., circle_latitude_radians, 0.));
            let longitude_radians = circle_longitude_increment * i as f32;
            let longitude_rotation = Rotation3::new(Vector3::new(0., 0., longitude_radians));

            arctic_circle_position = longitude_rotation * latitude_rotation * arctic_circle_position;

            let arctic_circle_node = Rc::new(NetNode {
                coordinates: Rc::new(arctic_circle_coordinates.clone()),
                position: arctic_circle_position
            });
            for coord in arctic_circle_coordinates {
                
                nodes.insert(coord, arctic_circle_node.clone());
            }
        }

        for i in 0..5 {
            let coord = (i + 1, i);
            let mut antarctic_circle_coordinates = vec!(coord);
            if i == 0 {
                antarctic_circle_coordinates.push((6, 5));
            }
            let mut antarctic_circle_position = Vector3::new(1., 0., 0.);
            let latitude_rotation = Rotation3::new(Vector3::new(0., - circle_latitude_radians, 0.));
            let longitude_radians = circle_longitude_increment + circle_longitude_increment * i as f32;
            let longitude_rotation = Rotation3::new(Vector3::new(0., 0., longitude_radians));

            antarctic_circle_position = longitude_rotation * latitude_rotation * antarctic_circle_position;

            let antarctic_circle_node = Rc::new(NetNode {
                coordinates: Rc::new(antarctic_circle_coordinates.clone()),
                position: antarctic_circle_position
            });
            for coord in antarctic_circle_coordinates {
                
                nodes.insert(coord, antarctic_circle_node.clone());
            }
        }

        for node in nodes.iter() {
            println!("{:?}", node)
        }

        Net { nodes }

    }
}


#[test]
fn run_icosahedron() {
    Net::build();
}