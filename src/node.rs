pub mod node {
    use std::collections::HashMap;
    pub struct Node {
        pub id: u8,
        pub edges_distances: HashMap<u8, f64>,
        pub edges_pheromones: HashMap<u8, f64>,
    }
}
