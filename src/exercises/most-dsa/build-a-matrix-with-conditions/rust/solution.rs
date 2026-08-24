use std::collections::VecDeque;

impl Solution {
    pub fn build_matrix(k: i32, row_conditions: Vec<Vec<i32>>, col_conditions: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row_order = Self::topo_sort(k, &row_conditions);
        if row_order.len() != k as usize {
            return vec![];
        }

        let col_order = Self::topo_sort(k, &col_conditions);
        if col_order.len() != k as usize {
            return vec![];
        }

        let mut res = vec![vec![0; k as usize]; k as usize];
        let mut col_index = vec![0; (k + 1) as usize];
        for i in 0..k as usize {
            col_index[col_order[i] as usize] = i;
        }

        for i in 0..k as usize {
            res[i][col_index[row_order[i] as usize]] = row_order[i];
        }

        res
    }

    fn topo_sort(k: i32, edges: &[Vec<i32>]) -> Vec<i32> {
        let mut indegree = vec![0; (k + 1) as usize];
        let mut adj: Vec<Vec<i32>> = vec![vec![]; (k + 1) as usize];

        for edge in edges {
            adj[edge[0] as usize].push(edge[1]);
            indegree[edge[1] as usize] += 1;
        }

        let mut queue: VecDeque<i32> = VecDeque::new();
        let mut order: Vec<i32> = Vec::new();

        for i in 1..=k {
            if indegree[i as usize] == 0 {
                queue.push_back(i);
            }
        }

        while let Some(node) = queue.pop_front() {
            order.push(node);
            for &nei in &adj[node as usize] {
                indegree[nei as usize] -= 1;
                if indegree[nei as usize] == 0 {
                    queue.push_back(nei);
                }
            }
        }

        if order.len() != k as usize {
            return vec![];
        }
        order
    }
}
