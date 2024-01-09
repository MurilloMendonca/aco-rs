pub mod graph;
pub mod aco;
pub mod node;
pub mod ant;

use crate::graph::graph::Graph;
use crate::aco::aco::Aco;

fn main() {
    let mut graph = Graph::new("examples/five.txt");
    //graph.print();
    let mut aco = Aco::new(graph);
    for _ in 0..100 {
        aco.iterate();
        //aco.graph.print();
    }
}
