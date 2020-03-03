use std::collections::BinaryHeap;

const INF: i64 = std::i64::MAX;
struct Graph {
    directed: bool,
    graph: Vec<Vec<(usize, i64)>>,
}

impl Graph {
    fn new(node: usize, flag: bool) -> Graph {
        Graph { directed: flag, graph: vec![vec![(0, 0)]; node] }
    }

    fn add_edge(&mut self, start_node: usize, end_node: usize, cost: i64) -> () {
        self.graph[start_node].push((end_node, cost));
        if !self.directed { self.graph[end_node].push((start_node, cost)); }
    }
    fn dijkstra(&self, start_node: usize) -> Vec<i64> {
        let node = self.graph[0].len();
        let mut dist = vec![INF; node];
        dist[start_node] = 0;

        let mut prev = vec![0; node];
        let mut queue: BinaryHeap<(i64, usize)> = BinaryHeap::new();

        queue.push((dist[start_node], start_node));

        while !queue.is_empty() {
            let (dist_u, u) = queue.pop().unwrap();

            if dist[u] < dist_u {
                continue;
            }

            for &(v, weight) in &self.graph[u] {
                let alt = dist_u + weight;
                if dist[v] > alt {
                    dist[v] = alt;
                    prev[v] = u;
                    queue.push((alt, v));
                }
            }
        };
        dist
    }
}


