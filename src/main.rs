pub mod aco;
pub mod ant;
pub mod graph;
pub mod node;

use crate::aco::aco::Aco;
use crate::graph::graph::Graph;

use std::sync::{Arc, Mutex};
use std::thread;

fn run(graph: Graph, iterations: usize, repetitions: usize) {
    let graph = Arc::new(graph);
    let best_ants = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..repetitions).map(|_| {
        let graph_clone = Arc::clone(&graph);
        let best_ants_clone = Arc::clone(&best_ants);

        thread::spawn(move || {
            let mut aco = Aco::new((*graph_clone).clone());
            for _ in 0..iterations {
                aco.iterate();
            }
            let mut best_ants = best_ants_clone.lock().unwrap();
            println!("Best ant: {:?}", aco.elite_ant);
            best_ants.push(aco.elite_ant.clone());
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let best_ants = best_ants.lock().unwrap();
    let best_ant = best_ants
        .iter()
        .min_by(|x, y| x.total_cost.partial_cmp(&y.total_cost).unwrap())
        .unwrap();

    println!("Best ant: {:?}", best_ant);
}

fn main() {
    let graph = Graph::new("examples/five.txt");
    run(graph, 100, 100);
}
