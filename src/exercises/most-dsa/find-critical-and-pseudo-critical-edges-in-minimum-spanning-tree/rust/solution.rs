use std::collections::HashSet;

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, node: usize) -> usize {
        if self.parent[node] != node {
            self.parent[node] = self.find(self.parent[node]);
        }
        self.parent[node]
    }

    fn union(&mut self, u: usize, v: usize) -> bool {
        let pu = self.find(u);
        let pv = self.find(v);
        if pu == pv {
            return false;
        }
        if self.size[pu] < self.size[pv] {
            self.size[pv] += self.size[pu];
            self.parent[pu] = pv;
        } else {
            self.size[pu] += self.size[pv];
            self.parent[pv] = pu;
        }
        true
    }
}

impl Solution {
    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut mst: Vec<Vec<i32>> = vec![Vec::new(); n];
        let mut mst_edges: HashSet<i32> = HashSet::new();
        let mut pseudo_critical_edges: HashSet<i32> = HashSet::new();

        // Sort edges by weight, keeping track of original index
        let mut edge_list: Vec<(i32, i32, i32, i32)> = edges
            .iter()
            .enumerate()
            .map(|(i, e)| (e[2], e[0], e[1], i as i32))
            .collect();
        edge_list.sort_by_key(|e| e.0);

        let mut uf = UnionFind::new(n);
        for &(weight, u, v, index) in &edge_list {
            let (u, v) = (u as usize, v as usize);
            if uf.union(u, v) {
                mst[u].push(index);
                mst[v].push(index);
                mst_edges.insert(index);
            }
        }

        // Check non-MST edges for pseudo-critical
        for i in 0..edges.len() {
            if mst_edges.contains(&(i as i32)) {
                continue;
            }
            let mut path: Vec<i32> = Vec::new();
            let destination = edges[i][1];
            if Self::dfs(edges[i][0] as usize, -1, destination as usize, &mst, &edges, &mut path) {
                for &p in &path {
                    if edges[p as usize][2] == edges[i][2] {
                        pseudo_critical_edges.insert(i as i32);
                        pseudo_critical_edges.insert(p);
                    }
                }
            }
        }

        let critical: Vec<i32> = mst_edges
            .iter()
            .filter(|&e| !pseudo_critical_edges.contains(e))
            .copied()
            .collect();

        vec![critical, pseudo_critical_edges.into_iter().collect()]
    }

    fn dfs(
        node: usize,
        parent: i32,
        destination: usize,
        mst: &Vec<Vec<i32>>,
        edges: &Vec<Vec<i32>>,
        path: &mut Vec<i32>,
    ) -> bool {
        if node == destination {
            return true;
        }
        for &edge_index in &mst[node] {
            if edge_index == parent {
                continue;
            }
            path.push(edge_index);
            let next_node = if edges[edge_index as usize][0] as usize == node {
                edges[edge_index as usize][1] as usize
            } else {
                edges[edge_index as usize][0] as usize
            };
            if Self::dfs(next_node, edge_index, destination, mst, edges, path) {
                return true;
            }
            path.pop();
        }
        false
    }
}
