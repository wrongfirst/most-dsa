use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut indexed_tasks: Vec<(i64, i64, usize)> = tasks
            .iter()
            .enumerate()
            .map(|(i, t)| (t[0] as i64, t[1] as i64, i))
            .collect();

        indexed_tasks.sort_by_key(|t| t.0);

        let mut res = Vec::new();
        let mut min_heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        let mut i = 0;
        let mut time = indexed_tasks[0].0;

        while !min_heap.is_empty() || i < indexed_tasks.len() {
            while i < indexed_tasks.len() && time >= indexed_tasks[i].0 {
                min_heap.push(Reverse((indexed_tasks[i].1, indexed_tasks[i].2)));
                i += 1;
            }

            if min_heap.is_empty() {
                time = indexed_tasks[i].0;
            } else {
                let Reverse((proc_time, index)) = min_heap.pop().unwrap();
                time += proc_time;
                res.push(index as i32);
            }
        }

        res
    }
}
