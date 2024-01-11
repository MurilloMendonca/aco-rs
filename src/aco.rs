pub mod aco {
    use crate::ant::ant::Ant;
    use crate::graph::graph::Graph;

    pub struct Aco {
        pub alpha: f64,
        pub beta: f64,
        pub q: f64,
        pub pheromone_persistence: f64,
        pub min_pheromone: f64,
        pub max_pheromone: f64,
        pub number_of_ants: u8,
        pub ants: Vec<Ant>,
        pub graph: Graph,
        pub elite_ant: Ant,
    }

    impl Aco {
        pub fn new(graph: Graph) -> Aco {
            let n_ants = graph.nodes.len() as u8;
            let mut elite_ant = Ant::new(0, n_ants);
            elite_ant.total_cost = f64::MAX;
            Aco {
                alpha: 1.0,
                beta: 2.0,
                q: 1.0,
                pheromone_persistence: 0.01,
                min_pheromone: 0.1,
                max_pheromone: 4.0,
                number_of_ants: n_ants * 2,
                ants: Vec::new(),
                graph: graph.clone(),
                elite_ant: elite_ant.clone(),
            }
        }
        fn reset_ants(&mut self) {
            self.ants = Vec::new();
            for _ in 0..self.number_of_ants {
                self.ants.push(Ant::new(0, self.graph.nodes.len() as u8));
            }
        }

        fn move_ant_complete_path(&self, ant: &mut Ant) {
            while ant.remaining_nodes.len() > 0 {
                let current_node = ant.current_node;
                let current_node = &self.graph.nodes[current_node as usize];
                let connected_nodes = current_node.edges_distances.keys();
                let remaining_nodes = &ant.remaining_nodes;

                let available_nodes = connected_nodes.filter(|&x| remaining_nodes.contains(x));
                let desires = available_nodes.map(|x| {
                    let distance = current_node.edges_distances[x];
                    let pheromone = current_node.edges_pheromones[x];
                    let desire =
                        f64::powf(1.0 / distance, self.alpha) * f64::powf(pheromone, self.beta);
                    (x, desire)
                });
                let total_desire = desires.clone().fold(0.0, |acc, x| acc + x.1);
                let probabilities = desires.map(|x| (x.0, x.1 / total_desire));

                let ordered_node_by_probability = probabilities.collect::<Vec<(&u8, f64)>>();
                //random number between 0 and 1
                let random_number = rand::random::<f64>();
                for i in 0..ordered_node_by_probability.len() {
                    let node = ordered_node_by_probability[i];
                    if random_number < node.1 {
                        // println!("Ant {:?} moving from  node {:?} to node {:?}", ant, current_node,node.0);
                        // println!("Probabilities: {:?}", ordered_node_by_probability);
                        ant.move_to(*node.0, current_node.edges_distances[node.0]);
                        break;
                    }
                }
            }
            let current_node = ant.current_node;
            let start_node = ant.path[0];
            ant.close_path(
                start_node,
                self.graph.nodes[current_node as usize].edges_distances[&start_node],
            );
        }

        fn move_all_ants(&mut self) {
            let mut ants = self.ants.clone();
            for ant in &mut ants {
                self.move_ant_complete_path(ant);
            }
            self.ants = ants;
        }

        fn evaporate_pheromone(&mut self) {
            self.graph.nodes.iter_mut().for_each(|node| {
                node.edges_pheromones.iter_mut().for_each(|edge| {
                    let current_pheromone = *edge.1;
                    let new_pheromone = (1.0 - self.pheromone_persistence) * current_pheromone;
                    *edge.1 = if new_pheromone < self.min_pheromone {
                        self.min_pheromone
                    } else {
                        new_pheromone
                    };
                    // println!("Pheromone {:?} -> {:?} = {:?} -> {:?}", node.id, edge.0, current_pheromone, new_pheromone);
                });
            });
        }

        fn update_pheronome(&mut self, ant: &Ant) {
            let total_cost = ant.total_cost;
            for i in 0..ant.path.len() - 1 {
                let current_node = ant.path[i];
                let next_node = ant.path[i + 1];
                let current_pheromone =
                    self.graph.nodes[current_node as usize].edges_pheromones[&next_node];
                let mut new_pheromone = current_pheromone + self.q / total_cost;
                if new_pheromone < self.min_pheromone {
                    new_pheromone = self.min_pheromone;
                }
                if new_pheromone > self.max_pheromone {
                    new_pheromone = self.max_pheromone;
                }
                let edge_to_change = self.graph.nodes[current_node as usize]
                    .edges_pheromones
                    .get_mut(&next_node);
                if edge_to_change.is_none() {
                    println!("Edge {:?} {:?} not found", current_node, next_node);
                    continue;
                }
                let edge = edge_to_change.unwrap();
                //println!("Pheromone {:?} -> {:?} = {:?} -> {:?}\n", current_node, next_node, current_pheromone, new_pheromone);
                *edge = new_pheromone as f64;
            }
        }

        fn update_pheronome_all_ants(&mut self) {
            let mut ants = self.ants.clone();
            for ant in &mut ants {
                self.update_pheronome(ant);
            }

            self.ants = ants;
        }

        pub fn iterate(&mut self) {
            self.reset_ants();

            // Move ants
            self.move_all_ants();

            // Evaporate pheromone
            self.evaporate_pheromone();
            // Update pheromone
            self.update_pheronome_all_ants();
            // Get the best ant (the one with the lowest total cost) that starts on the node 0
            let best_ant = self
                .ants
                .iter()
                .filter(|&x| x.path[0] == 0)
                .min_by(|x, y| x.total_cost.partial_cmp(&y.total_cost).unwrap())
                .unwrap();
            if best_ant.total_cost < self.elite_ant.total_cost {
                self.elite_ant = best_ant.clone();
            }
            // println!(
            //     "Best path: {:?} -- Score: {:}",
            //      self.elite_ant.path, self.elite_ant.total_cost
            //  );
        }
    }
}
