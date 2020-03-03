use std::cmp::min;
use std::usize;

struct Graph {
    node: Vec<Vec<i32>>,
}

impl Graph {
    fn new(node: usize) -> Graph {
        Graph { node: vec![vec![1000000007; node]; node] }
    }

    fn add_edge(&mut self, start_node: usize, end_node: usize, edge_cost: i32) -> () {
        self.node[start_node][end_node] = edge_cost;
        self.node[end_node][start_node] = edge_cost;
    }
    fn floyd_warshall(&mut self) -> () {
        let i = self.node[0].len();
        for relay_node in 0..i {
            for start_node in 0..i {
                for end_node in 0..i {
                    self.node[start_node][end_node] = min(self.node[start_node][end_node], self.node[start_node][relay_node] + self.node[relay_node][end_node]);
                }
            }
        }
    }
}

