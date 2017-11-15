
use geodesic::{Net, NetCoordinate};
use rand::{StdRng, SeedableRng, sample};
use std::collections::{HashMap, HashSet};

struct TectonicData {

}

struct Tectonic {
    net: Net,
    data: HashMap<NetCoordinate, TectonicData>
}

struct Plate {
    nodes: HashSet<NetCoordinate>
}

impl Tectonic {

    fn build() -> Tectonic {

        let net = Net::build_subdivided(4);
        let roots = Tectonic::roots(&net, 10);
        let plates = Tectonic::plates(&net, roots);

        panic!("TODO")

    }

    fn plates(net: &Net, roots: Vec<NetCoordinate>) -> Vec<Plate> {

        // create connected components by:
        // maintaining members for each plate, frontiers for each plate, and global visited? or edge removal
        // keep cycling through the frontiers and randomly select one eligible coordinate,
        // adding it to the plate
        // remove a plate from the cycle once its frontier is empty
        // PROOF - all nodes will be added (otherwise that node not added would be in the frontier of one or more plates)
        // ISSUE - plates are potentially very small, but probably not
        // ISSUE - plates can be very concave; maybe weight? maybe "break up" plates

        panic!("TODO")

    }

    fn roots(net: &Net, count: i32) -> Vec<NetCoordinate> {

        let mut tectonic_roots: Vec<NetCoordinate> = Vec::new();
        
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        let permutation = 1..net.adjacency.len();
        let root_indices = sample(&mut rng, permutation, 10);

        for root_index in root_indices.iter() {
            println!("Root index {}", root_index);
        }

        for (root, root_index) in net.adjacency.keys().zip(root_indices) {
            if root_index < count as usize {
                tectonic_roots.push(*root);
            }
        }
        
        tectonic_roots

    }

}

#[test]
fn make_plates() {
    Tectonic::build();
}