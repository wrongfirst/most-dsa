use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut adj: HashMap<String, Vec<(String, f64)>> = HashMap::new();

        for i in 0..equations.len() {
            let a = equations[i][0].clone();
            let b = equations[i][1].clone();

            adj.entry(a.clone()).or_default().push((b.clone(), values[i]));
            adj.entry(b.clone()).or_default().push((a.clone(), 1.0 / values[i]));
        }

        queries
            .iter()
            .map(|q| Self::bfs(&q[0], &q[1], &adj))
            .collect()
    }

    fn bfs(src: &str, target: &str, adj: &HashMap<String, Vec<(String, f64)>>) -> f64 {
        if !adj.contains_key(src) || !adj.contains_key(target) {
            return -1.0;
        }

        let mut queue: VecDeque<(String, f64)> = VecDeque::new();
        let mut visited: HashSet<String> = HashSet::new();

        queue.push_back((src.to_string(), 1.0));
        visited.insert(src.to_string());

        while let Some((node, weight)) = queue.pop_front() {
            if node == target {
                return weight;
            }

            if let Some(neighbors) = adj.get(&node) {
                for (neighbor, neighbor_weight) in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back((neighbor.clone(), weight * neighbor_weight));
                    }
                }
            }
        }

        -1.0
    }
}
