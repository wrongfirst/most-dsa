use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = num_courses as usize;
        let mut adj: Vec<HashSet<usize>> = vec![HashSet::new(); n];
        let mut indegree: Vec<i32> = vec![0; n];
        let mut is_prereq: Vec<HashSet<usize>> = vec![HashSet::new(); n];

        for prereq in &prerequisites {
            let pre = prereq[0] as usize;
            let crs = prereq[1] as usize;
            adj[pre].insert(crs);
            indegree[crs] += 1;
        }

        let mut q: VecDeque<usize> = VecDeque::new();
        for i in 0..n {
            if indegree[i] == 0 {
                q.push_back(i);
            }
        }

        while let Some(node) = q.pop_front() {
            for &neighbor in &adj[node] {
                is_prereq[neighbor].insert(node);
                let prereqs_of_node: Vec<usize> = is_prereq[node].iter().cloned().collect();
                for prereq in prereqs_of_node {
                    is_prereq[neighbor].insert(prereq);
                }
                indegree[neighbor] -= 1;
                if indegree[neighbor] == 0 {
                    q.push_back(neighbor);
                }
            }
        }

        queries
            .iter()
            .map(|query| {
                let u = query[0] as usize;
                let v = query[1] as usize;
                is_prereq[v].contains(&u)
            })
            .collect()
    }
}
