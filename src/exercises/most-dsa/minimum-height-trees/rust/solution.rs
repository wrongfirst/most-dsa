use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }

        let mut n = n as usize;
        let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..n {
            adj.insert(i, Vec::new());
        }

        for edge in &edges {
            let (n1, n2) = (edge[0] as usize, edge[1] as usize);
            adj.get_mut(&n1).unwrap().push(n2);
            adj.get_mut(&n2).unwrap().push(n1);
        }

        let mut edge_cnt: HashMap<usize, usize> = HashMap::new();
        let mut leaves: VecDeque<usize> = VecDeque::new();

        for (&src, neighbors) in &adj {
            edge_cnt.insert(src, neighbors.len());
            if neighbors.len() == 1 {
                leaves.push_back(src);
            }
        }

        while !leaves.is_empty() {
            if n <= 2 {
                return leaves.iter().map(|&x| x as i32).collect();
            }

            let leaves_count = leaves.len();
            for _ in 0..leaves_count {
                let node = leaves.pop_front().unwrap();
                n -= 1;
                for &nei in &adj[&node] {
                    if let Some(cnt) = edge_cnt.get_mut(&nei) {
                        *cnt -= 1;
                        if *cnt == 1 {
                            leaves.push_back(nei);
                        }
                    }
                }
            }
        }

        Vec::new()
    }
}
