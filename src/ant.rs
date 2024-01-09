// ant.rs
pub mod ant {
    #[derive(Debug)]
    pub struct Ant {
        pub path: Vec<u8>,
        pub total_cost: f64,
        pub remaining_nodes: Vec<u8>,
        pub current_node: u8,
    }

    impl Ant {
        // Create a new
        pub fn new(start_node: u8, graph_size: u8) -> Ant {
            let mut remaining_nodes: Vec<u8> = (0..graph_size).collect();

            // Remove the start node from the remaining nodes
            remaining_nodes.remove(start_node as usize);
            Ant {
                path: vec![start_node],
                total_cost: 0.0,
                current_node: start_node,
                remaining_nodes: remaining_nodes,
            }
        }
        pub fn move_to(&mut self, node: u8, distance: f64) {
            self.path.push(node);
            self.total_cost += distance;
            self.current_node = node;
            let index = self.remaining_nodes.iter().position(|&x| x == node).unwrap();
            self.remaining_nodes.remove(index as usize);
        }
        pub fn close_path(&mut self, start_node: u8, distance: f64) {
            self.path.push(start_node);
            self.total_cost += distance;
        }
    }
}
