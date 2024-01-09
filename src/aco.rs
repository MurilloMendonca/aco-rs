pub mod aco {
    use crate::ant::ant::Ant;
    use crate::graph::graph::Graph;
    pub struct Aco {
        pub alpha: f32,
        pub beta: f32,
        pub q: f64,
        pub pheromone_persistence: f64,
        pub number_of_ants: u8,
        pub ants: Vec<Ant>,
        pub graph: Graph,
    }

    impl Aco {
        pub fn new(graph: Graph) -> Aco {
            let n_ants = graph.nodes.len() as u8;
            Aco {
                alpha: 1.0,
                beta: 1.0,
                q: 4.0,
                pheromone_persistence: 0.5,
                number_of_ants: n_ants,
                ants: Vec::new(),
                graph: graph,
            }
        }

        pub fn iterate(&mut self) {
            // Reset ants
            self.ants = Vec::new();
            // Create ants
            for i in 0..self.number_of_ants {
                self.ants.push(Ant::new(i, self.graph.nodes.len() as u8));
            }

            // Move ants
            for ant in &mut self.ants {
                while ant.remaining_nodes.len() > 0 {
                    let current_node = ant.current_node;
                    let connected_nodes = self.graph.nodes[current_node as usize]
                        .edges_distances
                        .keys();
                    let remaining_nodes = &ant.remaining_nodes;

                    let available_nodes = connected_nodes.filter(|&x| remaining_nodes.contains(x));
                    let desires = available_nodes.map(|x| {
                        let distance = self.graph.nodes[current_node as usize].edges_distances[x];
                        let pheromone = self.graph.nodes[current_node as usize].edges_pheromones[x];
                        let desire =
                            f32::powf(1.0 / distance as f32, self.alpha as f32) * f32::powf(pheromone as f32, self.beta as f32);
                        (x, desire)
                    });
                    let total_desire = desires.clone().fold(0.0, |acc, x| acc + x.1);
                    let probabilities = desires.map(|x| (x.0, x.1 / total_desire));

                    let ordered_node_by_probability = probabilities.collect::<Vec<(&u8, f32)>>();
                    //random number between 0 and 1
                    let random_number = rand::random::<f32>();
                    for i in 0..ordered_node_by_probability.len() {
                        let node = ordered_node_by_probability[i];
                        if random_number < node.1 {
                            //println!("Ant {:?} moving from  node {:?} to node {:?}", ant, current_node,node.0);
                            ant.move_to(
                                *node.0,
                                self.graph.nodes[current_node as usize].edges_distances[node.0],
                            );
                            break;
                        }
                    }
                }
            }

            // Complete the path returning to the start node
            // and update the total total cost
            for ant in &mut self.ants {
                let current_node = ant.current_node;
                let start_node = ant.path[0];
                ant.close_path(
                    start_node,
                    self.graph.nodes[current_node as usize].edges_distances[&start_node],
                );
            }

            // Evaporate pheromone
            for node in &mut self.graph.nodes {
                for edge in &mut node.edges_pheromones {
                    let current_pheromone = *edge.1;
                    let new_pheromone = (1.0 - self.pheromone_persistence) * current_pheromone;
                    *edge.1 = new_pheromone;
                }
            }
            // Update pheromone
            for ant in &mut self.ants {
                let total_cost = ant.total_cost;
                for i in 0..ant.path.len()-1 {
                    let current_node = ant.path[i];
                    let next_node = ant.path[i + 1];
                    let current_pheromone =
                        self.graph.nodes[current_node as usize].edges_pheromones[&next_node];
                    let new_pheromone = current_pheromone + self.q / total_cost;
                    let edge_to_change = self.graph.nodes[current_node as usize].edges_pheromones.get_mut(&next_node);
                    if edge_to_change.is_none() {
                        println!("Edge {:?} {:?} not found", current_node, next_node);
                        continue;
                    }
                    let edge = edge_to_change.unwrap();
                    *edge = new_pheromone as f64;
                }
            }

            let best_ant = self.ants.iter().min_by(|x, y| x.total_cost.partial_cmp(&y.total_cost).unwrap()).unwrap();
            println!("Best path: {:?} -- Score: {:}", best_ant.path, best_ant.total_cost);
        }
    }
}
