use std::collections::BinaryHeap;

const INF: i64 = std::i64::MAX;

#[derive(Debug)]
struct Graph {
    node: usize,
    directed: bool,
    graph: Vec<Vec<(usize, i64)>>,
}

impl Graph {
    fn new(n: usize, flag: bool) -> Graph {
        Graph { node: n, directed: flag, graph: vec![vec![]; n] }
    }

    fn add_edge(&mut self, start_node: usize, end_node: usize, cost: i64) -> () {
        self.graph[start_node].push((end_node, cost));
        if !self.directed { self.graph[end_node].push((start_node, cost)); }
    }
    fn dijkstra(&self, start_node: usize) -> Vec<i64> {
        let mut dist = vec![INF; self.node];
        dist[start_node] = 0;

        let mut queue: BinaryHeap<(i64, usize)> = BinaryHeap::new();
        queue.push((0, start_node));

        while !queue.is_empty() {
            let (min_dist, idx) = queue.pop().unwrap();
            if dist[idx] < -min_dist {
                continue;
            }
            for &(to, weight) in &self.graph[idx] {
                let cost_cand = dist[idx] + weight;
                if dist[to] > cost_cand {
                    dist[to] = cost_cand;
                    queue.push((-cost_cand, to));
                }
            }
        }
        dist
    }
}
