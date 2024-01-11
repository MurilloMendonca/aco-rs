pub mod graph {
    use std::fs::File;
    use std::io::prelude::*;

    use crate::node::node::Node;
    use std::collections::HashMap;
    #[derive(Debug, Clone)]
    pub struct Graph {
        pub nodes: Vec<Node>,
    }

    impl Graph {
        pub fn new(filename: &str) -> Graph {
            let mut file = File::open(filename).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let mut lines = contents.lines();
            let n_nodes = contents.lines().count();
            let mut nodes = Vec::with_capacity(n_nodes);

            for i in 0..n_nodes {
                let mut edges_distances = HashMap::new();
                let mut edges_pheromones = HashMap::new();
                let mut columns = lines.next().unwrap().split_whitespace();
                for j in 0..n_nodes {
                    let distance = columns.next().unwrap().parse::<f64>().unwrap();
                    if distance == 0.0 {
                        continue;
                    }
                    edges_distances.insert(j as u8, distance);
                    edges_pheromones.insert(j as u8, 0.5);
                }
                nodes.push(Node {
                    id: i as u8,
                    edges_distances: edges_distances,
                    edges_pheromones: edges_pheromones,
                });
            }

            Graph { nodes: nodes }
        }

        pub fn print(&self) {
            for node in &self.nodes {
                println!("Node: {}", node.id);
                println!("Edges Distances: {:?}", node.edges_distances);
                println!("Edges Pheromones: {:?}", node.edges_pheromones);
            }
        }
    }
}
