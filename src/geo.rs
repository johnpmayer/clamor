

use std::rc::Rc;
use std::collections::{HashSet, HashMap};
use std::f32::{self, consts};

use num::Integer;

use nalgebra::geometry::Rotation3;
use nalgebra::core::{Vector3};
use nalgebra;

// use vecmath::Vector2;
use vecmath::{vec2_add, vec2_scale};

type NetCoordinate = [i32; 2];

/*

Primary Icosahedron Net

5   .               o = o = o
                    | / | / I
4   .           o = * - * - o
                | / | / I
3   .       o = * - * - o
            | / | / I
2   .   o = * - * - o
        | / | / I
1   x = * - * - o
    | / | / I
0   * - * - x   .   .   .   .

    0   1   2   3   4   5   6
*/

#[derive(Clone, Debug)]
pub enum NodeType {
    NorthPole,
    ArcticEdge,
    Internal,
    TropicalEdge,
    AntarcticEdge,
    SouthPole,
}

// TODO make a "dump" debug function rather than making these types/fields public
#[derive(Clone, Debug)]
pub struct NetNode {
    coordinates: Vec<NetCoordinate>,
    pub position: Vector3<f32>,
    is_primary: bool,
    node_type: NodeType,
}

#[derive(Clone, Debug)]
pub struct Net {
    factor: i32,
    pub nodes: HashMap<NetCoordinate, Rc<NetNode>>,
    adjacency: HashMap<NetCoordinate, Vec<NetCoordinate>>, // Counter-clockwise w.r.t. the Net
}

#[derive(Debug)]
enum NetError{ InvalidCoordinate }

impl Net {
    pub fn build() -> Net {
        let mut nodes: HashMap<NetCoordinate, Rc<NetNode>> = HashMap::new();

        let circle_latitude_radians: f32 = f32::atan(0.5);
        let circle_longitude_increment: f32 = consts::PI / 5.;

        {
            let mut north_pole_net_coordinates: Vec<NetCoordinate> = Vec::new();
            for i in 0..5 {
                let coord = [i, i + 1];
                north_pole_net_coordinates.push(coord);
            }
            let north_pole_position = Vector3::new(0., 0., 1.);
            let north_pole_node = Rc::new(NetNode { 
                coordinates: north_pole_net_coordinates.clone(),
                position: north_pole_position,
                is_primary: true,
                node_type: NodeType::NorthPole,
            });
            for coord in north_pole_net_coordinates {
                nodes.insert(coord, north_pole_node.clone());
            }
        }

        {
            let mut south_pole_net_coordinates: Vec<NetCoordinate> = Vec::new();
            for i in 0..5 {
                let coord = [i + 2, i];
                south_pole_net_coordinates.push(coord);
            }
            let south_pole_position = Vector3::new(0., 0., -1.);
            let south_pole_node = Rc::new(NetNode { 
                coordinates: south_pole_net_coordinates.clone(), 
                position: south_pole_position,
                is_primary: true,
                node_type: NodeType::SouthPole,
            });
            for coord in south_pole_net_coordinates {
                nodes.insert(coord, south_pole_node.clone());
            }
        }

        for i in 0..5 {
            let coord = [i, i];
            let mut arctic_circle_coordinates = vec!(coord);
            if i == 0 {
                arctic_circle_coordinates.push([5, 5]);
            }
            let mut arctic_circle_position = Vector3::new(1., 0., 0.);
            let latitude_rotation = Rotation3::new(Vector3::new(0., - circle_latitude_radians, 0.));
            let longitude_radians = (i as f32) * 2. * circle_longitude_increment;
            let longitude_rotation = Rotation3::new(Vector3::new(0., 0., longitude_radians));

            arctic_circle_position = longitude_rotation * latitude_rotation * arctic_circle_position;

            let arctic_circle_node = Rc::new(NetNode {
                coordinates: arctic_circle_coordinates.clone(),
                position: arctic_circle_position,
                is_primary: true,
                node_type: if i == 0 { NodeType::TropicalEdge } else { NodeType::Internal},
            });
            for coord in arctic_circle_coordinates {
                
                nodes.insert(coord, arctic_circle_node.clone());
            }
        }

        for i in 0..5 {
            let coord = [i + 1, i];
            let mut antarctic_circle_coordinates = vec!(coord);
            if i == 0 {
                antarctic_circle_coordinates.push([6, 5]);
            }
            let mut antarctic_circle_position = Vector3::new(1., 0., 0.);
            let latitude_rotation = Rotation3::new(Vector3::new(0., circle_latitude_radians, 0.));
            let longitude_radians = circle_longitude_increment + (i as f32) * 2. * circle_longitude_increment;
            let longitude_rotation = Rotation3::new(Vector3::new(0., 0., longitude_radians));

            antarctic_circle_position = longitude_rotation * latitude_rotation * antarctic_circle_position;

            let antarctic_circle_node = Rc::new(NetNode {
                coordinates: antarctic_circle_coordinates.clone(),
                position: antarctic_circle_position,
                is_primary: true,
                node_type: if i == 0 { NodeType::TropicalEdge } else { NodeType::Internal},
            });
            for coord in antarctic_circle_coordinates {
                
                nodes.insert(coord, antarctic_circle_node.clone());
            }
        }

        assert!(nodes.len() == 22);

        let adjacency = Net::calculate_canonical_adjacency(&nodes);

        assert!(adjacency.len() == 12);

        Net { factor: 1, nodes, adjacency }

    }

    fn calculate_canonical_adjacency(nodes: &HashMap<NetCoordinate, Rc<NetNode>>) -> HashMap<NetCoordinate, Vec<NetCoordinate>> {
        let mut adjacency: HashMap<NetCoordinate, Vec<NetCoordinate>> = HashMap::new();

        for (node_coordinate, node) in nodes.iter() {

            if nodes.get(node_coordinate).unwrap().coordinates[0] != *node_coordinate {
                // Only store canonical coordinates in the adjacency lookup
                continue
            }

            let neighbors = Net::canonical_neighbors(&nodes, node_coordinate).unwrap();

            println!("{:?} ({:?}), {:?}", node_coordinate, node.node_type, neighbors);

            assert!(
                if node.is_primary { neighbors.len() == 5 } else { neighbors.len() == 6 }
            );
            adjacency.insert(*node_coordinate, neighbors);
        }

        adjacency
    }

    fn primary_non_polar_coordinates() -> Vec<NetCoordinate> {
        let mut coordinates = Vec::new();
        for i in 0..5 {
            coordinates.push([i, i]);
            coordinates.push([i + 1, i]);
        }
        coordinates
    }

    fn canonical_neighbors(nodes: &HashMap<NetCoordinate, Rc<NetNode>>, coordinate: &NetCoordinate) -> Result<Vec<NetCoordinate>, NetError> {
        let mut neighbors = HashSet::new();
        let mut counter_clockwise_neighbors = Vec::new();

        let node: &Rc<NetNode> = nodes.get(coordinate).ok_or(NetError::InvalidCoordinate)?;

        let mut test_coordinates = Vec::new();

        match node.node_type {
            NodeType::NorthPole => {
                for &node_coordinate in node.coordinates.iter() {
                    let offset = [0, -1];
                    let test_coordinate = vec2_add(node_coordinate, offset);
                    test_coordinates.push(test_coordinate);
                }
            },
            NodeType::ArcticEdge => {
                let canonical_offsets: [NetCoordinate; 4] = [
                    [0,-1],
                    [1,0],
                    [1,1],
                    [0,1],
                ];
                for canonical_offset in canonical_offsets.iter() {
                    let test_coordinate = vec2_add(node.coordinates[0], *canonical_offset);
                    test_coordinates.push(test_coordinate);
                }
                let noncanonical_offsets: [NetCoordinate; 2] = [
                    [-1, -1],
                    [0, -1],
                ];
                for noncanonical_offset in noncanonical_offsets.iter() {
                    let test_coordinate = vec2_add(node.coordinates[1], *noncanonical_offset);
                    test_coordinates.push(test_coordinate);
                }
            },
            NodeType::Internal => {
                let internal_offsets: [NetCoordinate; 6] = [
                    [1,0],
                    [1,1],
                    [0,1],
                    [-1,0],
                    [-1,-1],
                    [0,-1],
                ];
                for offset in internal_offsets.iter() {
                    let test_coordinate = vec2_add(*coordinate, *offset);
                    test_coordinates.push(test_coordinate);
                }
            },
            NodeType::TropicalEdge => {
                let canonical_offsets: [NetCoordinate; 3] = [
                    [1,0],
                    [1,1],
                    [0,1],
                ];
                for canonical_offset in canonical_offsets.iter() {
                    let test_coordinate = vec2_add(node.coordinates[0], *canonical_offset);
                    test_coordinates.push(test_coordinate);
                }
                let noncanonical_offsets: [NetCoordinate; 3] = [
                    [-1, 0],
                    [-1, -1],
                    [0, -1],
                ];
                for noncanonical_offset in noncanonical_offsets.iter() {
                    let test_coordinate = vec2_add(node.coordinates[1], *noncanonical_offset);
                    test_coordinates.push(test_coordinate);
                }
            },
            NodeType::AntarcticEdge => {
                let canonical_offsets: [NetCoordinate; 4] = [
                    [1,0],
                    [1,1],
                    [0,1],
                    [-1, 0],
                ];
                for canonical_offset in canonical_offsets.iter() {
                    let test_coordinate = vec2_add(node.coordinates[0], *canonical_offset);
                    test_coordinates.push(test_coordinate);
                }
                let noncanonical_offsets: [NetCoordinate; 2] = [
                    [-1, 0],
                    [-1, -1],
                ];
                for noncanonical_offset in noncanonical_offsets.iter() {
                    let test_coordinate = vec2_add(node.coordinates[1], *noncanonical_offset);
                    test_coordinates.push(test_coordinate);
                }
            },
            NodeType::SouthPole => {
                for &node_coordinate in node.coordinates.iter() {
                    let offset = [-1, 0];
                    let test_coordinate = vec2_add(node_coordinate, offset);
                    test_coordinates.push(test_coordinate);
                }
            },
        }

        for test_coordinate in test_coordinates {
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

        Ok(counter_clockwise_neighbors)
    }

    pub fn build_subdivided(factor: i32) -> Net {

        let mut nodes = HashMap::new();

        let primary_net = Net::build();
        assert!(primary_net.nodes.len() == 22);

        // Scale the 12 primary nodes and their aliases
        for (_, primary_node) in primary_net.nodes.iter().filter(|&(coord, ref node)| {
            node.coordinates[0] == *coord
        }) {
            // println!("Primary node: {:?}", primary_node);
            let mut new_primary_node: NetNode = primary_node.as_ref().clone();

            for ref mut coordinate in &mut new_primary_node.coordinates {
                **coordinate = vec2_scale(**coordinate, factor);
            }

            let rc_new_primary_node = Rc::new(new_primary_node);

            for new_coordinate in rc_new_primary_node.clone().coordinates.iter() {
                nodes.insert(*new_coordinate, rc_new_primary_node.clone());
                // println!("New primary node: {:?} -> {:?}", new_coordinate, rc_new_primary_node);
            }
            
        }

        assert!(nodes.len() == 22);

        // For each non-polar primary node
        // - Create all canonical edge nodes (3x, up, diagonal, right)
        // - Create all internal nodes (2x triangles, top & bottom)

        let mut nodes_to_insert = Vec::new();

        for primary_non_polar_coordinate in Net::primary_non_polar_coordinates() {

            let root_coordinate = vec2_scale(primary_non_polar_coordinate, factor);
            let root_node = nodes.get(&root_coordinate).unwrap();
            let root_position = root_node.position;

            // println!("Primary non_polar node: {:?}", root_coordinate);

            let up_offset = [0, 1];
            let parallel_offset = [1, 1];
            let right_offset = [1, 0];

            let up_coordinate = vec2_add(root_coordinate, vec2_scale(up_offset, factor));
            let up_node = nodes.get(&up_coordinate).unwrap();
            let up_displacement = (up_node.position - root_position) / (factor as f32);

            let parallel_coordinate = vec2_add(root_coordinate, vec2_scale(parallel_offset, factor));
            let parallel_node = nodes.get(&parallel_coordinate).unwrap();
            let parallel_displacement = (parallel_node.position - root_position)  / (factor as f32);
            
            let right_coordinate = vec2_add(root_coordinate, vec2_scale(right_offset, factor));
            let right_node = nodes.get(&right_coordinate).unwrap();
            let right_displacement = (right_node.position - root_position)  / (factor as f32);

            // Up Edge
            for i in 1..factor {
                let edge_node_coord = vec2_add(root_coordinate, vec2_scale(up_offset, i));
                // println!("Up Edge {} -> {:?}", i, edge_node_coord);
                let edge_node_displacement = up_displacement * (i as f32);
                let edge_node_position = root_position + edge_node_displacement;
                let edge_node = Rc::new(NetNode {
                    coordinates: vec!(edge_node_coord),
                    position: edge_node_position.normalize(),
                    is_primary: false,
                    node_type: NodeType::Internal,
                });
                nodes_to_insert.push((edge_node_coord, edge_node));
            }

            // Parallel Edge
            for i in 1..factor {
                let edge_node_coord = vec2_add(root_coordinate, vec2_scale(parallel_offset, i));
                // println!("Parallel Edge {} -> {:?}", i, edge_node_coord);
                let edge_node_displacement = parallel_displacement * (i as f32);
                let edge_node_position = root_position + edge_node_displacement;
                let edge_node = Rc::new(NetNode {
                    coordinates: vec!(edge_node_coord),
                    position: edge_node_position.normalize(),
                    is_primary: false,
                    node_type: NodeType::Internal,
                });
                nodes_to_insert.push((edge_node_coord, edge_node));
            }

            // Right Edge
            for i in 1..factor {
                let edge_node_coord = vec2_add(root_coordinate, vec2_scale(right_offset, i));
                // println!("Right Edge {} -> {:?}", i, edge_node_coord);
                let edge_node_displacement = right_displacement * (i as f32);
                let edge_node_position = root_position + edge_node_displacement;
                let edge_node = Rc::new(NetNode {
                    coordinates: vec!(edge_node_coord),
                    position: edge_node_position.normalize(),
                    is_primary: false,
                    node_type: NodeType::Internal,
                });
                nodes_to_insert.push((edge_node_coord, edge_node));
            }

            // Upper triangle
            for i in 1..factor {
                for j in 1..(factor - i) {
                    let internal_node_coord = vec2_add(vec2_add(root_coordinate, vec2_scale(up_offset, i)), vec2_scale(parallel_offset, j));
                    // println!("Upper Triangle {},{} -> {:?}", i, j, internal_node_coord);
                    let internal_node_displacement = up_displacement * (i as f32) + parallel_displacement * (j as f32);
                    let internal_node_position = root_position + internal_node_displacement;
                    let internal_node = Rc::new(NetNode {
                        coordinates: vec!(internal_node_coord),
                        position: internal_node_position.normalize(),
                        is_primary: false,
                        node_type: NodeType::Internal,
                    });
                    nodes_to_insert.push((internal_node_coord, internal_node));
                }
            }

            // Lower triangle
            for i in 1..factor {
                for j in 1..(factor - i) {
                    let internal_node_coord = vec2_add(vec2_add(root_coordinate, vec2_scale(right_offset, i)), vec2_scale(parallel_offset, j));
                    // println!("Lower Triangle {},{} -> {:?}", i, j, internal_node_coord);
                    let internal_node_displacement = right_displacement * (i as f32) + parallel_displacement * (j as f32);
                    let internal_node_position = root_position + internal_node_displacement;
                    let internal_node = Rc::new(NetNode {
                        coordinates: vec!(internal_node_coord),
                        position: internal_node_position.normalize(),
                        is_primary: false,
                        node_type: NodeType::Internal,
                    });
                    nodes_to_insert.push((internal_node_coord, internal_node));
                }
            }

        }

        // Needed because we have immutable borrows in the above block
        for (new_coord, new_node) in nodes_to_insert {
            nodes.insert(new_coord, new_node);
        }

        // println!("Including canonical edges and iterior nodes: {}", nodes.len());
        assert!(nodes.len() as i32 == 
            22 + // Primary nodes, canonical & non-canonical
            (10 * (factor - 1) * (factor + 1)) // Canonical edge nodes and all internal nodes
        );

        // Create all non-canonical edge nodes - an additional 11 * (factor - 1)
        // - 5x North Pole
        // - 5x South Pole
        // - 1x "Tropics"

        let mut noncanonical_nodes_to_insert = Vec::new();

        // North pole noncanonical edge nodes

        for canonical_edge_node_index in 0..5 {
            let canonical_edge_root_coordinate = [canonical_edge_node_index * factor, (canonical_edge_node_index + 1) * factor];
            let canonical_edge_offset = [0, -1];

            let noncanonical_edge_node_index = (canonical_edge_node_index - 1).mod_floor(&5);
            let noncanonical_edge_root_coordinate = [noncanonical_edge_node_index * factor, (noncanonical_edge_node_index + 1) * factor];
            let noncanonical_edge_offset = [1, 0];
            
            for offset_index in 1..factor {
                let canonical_edge_coordinate = vec2_add(canonical_edge_root_coordinate, vec2_scale(canonical_edge_offset, offset_index));
                let noncanonical_edge_coordinate = vec2_add(noncanonical_edge_root_coordinate, vec2_scale(noncanonical_edge_offset, offset_index));

                let mut canonical_node_rc: &mut Rc<NetNode> = nodes.get_mut(&canonical_edge_coordinate).unwrap();
                
                {
                    // Safe to unwrap, this is an edge node created above and there should only be one active reference
                    let ref mut canonical_node = Rc::get_mut(canonical_node_rc).unwrap();
                    canonical_node.node_type = NodeType::ArcticEdge;
                    canonical_node.coordinates.push(noncanonical_edge_coordinate);
                }

                noncanonical_nodes_to_insert.push((noncanonical_edge_coordinate, canonical_node_rc.clone()));
            }
        }

        // South pole noncanonical edge nodes

        for canonical_edge_node_index in 0..5 {
            let canonical_edge_root_coordinate = [(canonical_edge_node_index + 2) * factor, canonical_edge_node_index * factor];
            let canonical_edge_offset = [-1, 0];

            let noncanonical_edge_node_index = (canonical_edge_node_index - 1).mod_floor(&5);
            let noncanonical_edge_root_coordinate = [(noncanonical_edge_node_index + 2) * factor, noncanonical_edge_node_index * factor];
            let noncanonical_edge_offset = [0, 1];
            
            for offset_index in 1..factor {
                let canonical_edge_coordinate = vec2_add(canonical_edge_root_coordinate, vec2_scale(canonical_edge_offset, offset_index));
                let noncanonical_edge_coordinate = vec2_add(noncanonical_edge_root_coordinate, vec2_scale(noncanonical_edge_offset, offset_index));

                let mut canonical_node_rc: &mut Rc<NetNode> = nodes.get_mut(&canonical_edge_coordinate).unwrap();
                
                {
                    // Safe to unwrap, this is an edge node created above and there should only be one active reference
                    let ref mut canonical_node = Rc::get_mut(canonical_node_rc).unwrap();
                    canonical_node.node_type = NodeType::AntarcticEdge;
                    canonical_node.coordinates.push(noncanonical_edge_coordinate);
                }

                noncanonical_nodes_to_insert.push((noncanonical_edge_coordinate, canonical_node_rc.clone()));
            }
        }

        // "Tropics" noncanonical edge nodes

        {
            let canonical_tropics_root = [0, 0];
            let noncanonical_tropics_root = [5 * factor, 5 * factor];
            let tropics_offset = [1, 0];

            for offset_index in 1..factor {
                let canonical_edge_coordinate = vec2_add(canonical_tropics_root, vec2_scale(tropics_offset, offset_index));
                let noncanonical_edge_coordinate = vec2_add(noncanonical_tropics_root, vec2_scale(tropics_offset, offset_index));

                let mut canonical_node_rc: &mut Rc<NetNode> = nodes.get_mut(&canonical_edge_coordinate).unwrap();
                
                {
                    // Safe to unwrap, this is an edge node created above and there should only be one active reference
                    let ref mut canonical_node = Rc::get_mut(canonical_node_rc).unwrap();
                    canonical_node.node_type = NodeType::TropicalEdge;
                    canonical_node.coordinates.push(noncanonical_edge_coordinate);
                }

                noncanonical_nodes_to_insert.push((noncanonical_edge_coordinate, canonical_node_rc.clone()));
            }
        }

        // Needed because we have immutable borrows in the above block
        for (new_coord, new_node) in noncanonical_nodes_to_insert {
            nodes.insert(new_coord, new_node);
        }

        // println!("Including non-canonical edges: {}", nodes.len());
        assert!(nodes.len() as i32 == 
            22 + // Primary nodes, canonical & non-canonical
            10 * (factor - 1) * (factor + 1) + // Canonical edge nodes and all internal nodes
            11 * (factor - 1) // Non-canonical edge nodes
        );

        let adjacency = Net::calculate_canonical_adjacency(&nodes);
        
        assert!(adjacency.len() as i32 == 
            12 + // primary
            10 * (factor - 1) * (factor + 1) // Canonical edge nodes and all internal nodes
        );

        Net { factor, nodes, adjacency }
    }

    pub fn faces(&self) -> Vec<[Vector3<f32>; 3]> {

        let mut faces = Vec::new();

        for (center_coord, neighbor_coords) in self.adjacency.iter() {
            let num_neighbors = neighbor_coords.len(); // May be 5 or 6
            let center_node = self.nodes.get(center_coord).unwrap();

            let mut midpoints: Vec<Vector3<f32>> = Vec::new();

            for face_i in 0..num_neighbors {
                let right_coord = neighbor_coords[face_i];
                let right_node = self.nodes.get(&right_coord).unwrap();

                let left_coord = neighbor_coords[(face_i + 1) % num_neighbors];
                let left_node = self.nodes.get(&left_coord).unwrap();

                let midpoint = (center_node.position + right_node.position + left_node.position) / 3.;

                midpoints.push(midpoint);
            }

            for dual_face_i in 0..num_neighbors {
                let right_midpoint = midpoints.as_slice()[dual_face_i];
                let left_midpoint = midpoints.as_slice()[(dual_face_i + 1) % num_neighbors];
                

                println!(
                    "Face {}: (Center = {:?} --> )\n\tcenter {:?}\n\tright {:?}\n\tleft {:?}\n\tnorm cr {}\n\tnorm rl {}\n\tnorm lc {}",
                    dual_face_i, center_coord, //right_coord, left_coord,
                    center_node.position,
                    right_midpoint,
                    left_midpoint,
                    nalgebra::norm(&(center_node.position - right_midpoint)),
                    nalgebra::norm(&(right_midpoint - left_midpoint)),
                    nalgebra::norm(&(left_midpoint - center_node.position)),
                );



                faces.push([
                    center_node.position,
                    right_midpoint.normalize(),
                    left_midpoint.normalize(),
                ])
            }
        }

        faces

    }
}

#[test]
fn modulo_behavior() {
    assert!((-1) % 5 == -1); // The '%' operator is actually division remainder, not signed modulus
    assert!((-1).mod_floor(&5) == 4); // This is the one I want to use!!!
}

#[test]
fn run_icosahedron() {
    let n = Net::build();
    println!("build - {} {}", n.nodes.len(), n.adjacency.len());
}

#[test]
fn run_icosahedron_2() {
    let n = Net::build_subdivided(2);
    println!("build_subdivided 2 - {} {}", n.nodes.len(), n.adjacency.len());
}

#[test]
fn run_icosahedron_5() {
    Net::build_subdivided(5);
}

#[test]
fn run_faces_5() {
    Net::build_subdivided(5).faces();
}