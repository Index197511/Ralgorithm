const INF: i64 = std::i64::MAX;
struct Graph {
    num_node: usize,
    edges: Vec<(usize, usize, i64)>,
}

impl Graph {
    fn new(num_of_node: usize) -> Self {
        Graph { num_node: num_of_node, edges: Vec::new() }
    }

    fn add_edge(&mut self, start_node: usize, end_node: usize, edge_cost: i64, directed: bool) -> () {
        self.edges.push((start_node, end_node, edge_cost));
        if !directed { self.edges.push((end_node, start_node, edge_cost)); }
    }

    fn bellman_ford(&self, start_node: usize) -> i64 {
        let node = self.num_node;
        let mut distance: Vec<i64> = vec![INF; node];
        distance[start_node] = 0;
        for i in 0..node {
            for &(u, v, c) in &self.edges {
                if distance[u] != INF && distance[u] + c < distance[v] {
                    distance[v] = distance[u] + c;
                    if i == (node - 1) && v == (node - 1) {
                        return -INF;
                    }
                }
            }
        }
        -distance[node - 1]
    }
}



