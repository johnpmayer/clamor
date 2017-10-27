

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{HashSet, HashMap};
use std::f32::{self, consts};

use nalgebra::geometry::Rotation3;
use nalgebra::core::Vector3;

type NetCoordinate = (i32, i32);

#[derive(Debug)]
struct NetNode {
    coordinates: Rc<Vec<NetCoordinate>>,
    position: Vector3<f32>,
}

#[derive(Debug)]
struct Net {
    nodes: HashMap<NetCoordinate, Rc<NetNode>>,
    adjacency: HashMap<NetCoordinate, Vec<NetCoordinate>>,
}

#[derive(Debug)]
enum NetError {
    InvalidCoordinate
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

        let mut adjacency: HashMap<NetCoordinate, Vec<NetCoordinate>> = HashMap::new();

        for node_coordinate in nodes.keys() {

            if nodes.get(node_coordinate).unwrap().coordinates[0] != *node_coordinate {
                // Only store canonical coordinates in the adjacency lookup
                continue
            }

            let neighbors = Net::canonical_neighbors(&nodes, node_coordinate).unwrap();
            println!("{:?}, {:?}", node_coordinate, neighbors);
            assert!(neighbors.len() == 5);
            adjacency.insert(*node_coordinate, neighbors);
        }

        Net { nodes, adjacency }

    }

    fn face_base_coordinates() -> Vec<NetCoordinate> {
        let mut coordinates = Vec::new();
        for i in 0..5 {
            coordinates.push((i, i));
            coordinates.push((i + 1, i));
        }
        coordinates
    }

    fn canonical_neighbors(nodes: &HashMap<NetCoordinate, Rc<NetNode>>, coordinate: &NetCoordinate) -> Result<Vec<NetCoordinate>, NetError> {
        let mut neighbors = HashSet::new();
        let mut counter_clockwise_neighbors = Vec::new();

        let offsets: [NetCoordinate; 6] = [
            (1,0),(1,1),(0,1),
            (-1,0),(-1,-1),(0,-1),
        ];

        let node: &Rc<NetNode> = nodes.get(&coordinate).ok_or(NetError::InvalidCoordinate)?;

        for &node_coordinate in node.coordinates.iter() {
            let (node_x, node_y) = node_coordinate;
            for offset in offsets.iter() {
                let (offset_x, offset_y) = *offset;

                let test_coordinate = (node_x + offset_x, node_y + offset_y);
                if node.coordinates.iter().find(|c| **c == test_coordinate) ==  None {
                    for neighbor_node in nodes.get(&test_coordinate) {
                        // Only insert the canonical coordinate
                        let canonical_neighbor_coordinate = neighbor_node.coordinates[0];
                        if neighbors.insert(canonical_neighbor_coordinate) {
                           counter_clockwise_neighbors.push(canonical_neighbor_coordinate)
                        }
                    }
                }
            }
        }

        Ok(counter_clockwise_neighbors)
    }

    fn subdivide(&self) -> Net {

        /*
        Scale the 12 primary nodes and their aliases
        Create canonical and alias coordinates for all non-canonical edge nodes
            North pole x5
                For each north hemisphere non-polar primary node:
                    the canonical edge is defined as: node + (0,1) = north pole
                    the non-canonical edge is defined as: node + (-1, 0) = north pole
            South pole x5
                For each south hemisphere non-polar primary node:
                    the canonical edge is defined as: node + (1,0) = south pole
                    the non-canonical edge is defined as: node + (0, -1) = south pole
            Tropics belt x1
                The canonical edge is defined as: (0,0) <--> (1,0)
                The non-canonical edge is defined as: (5,5) <--> (6,5)
        Create all remaining 'edge' nodes, relative to the 10 non-polar primary nodes
        Create all remaining internal nodes, relative to the the 10 non-polar primary nodes, (2x triangles, top & bottom)
        */

        panic!("TODO")
    }
}


#[test]
fn run_icosahedron() {
    Net::build();
}