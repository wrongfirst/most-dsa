use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let rows = heights.len();
        let cols = heights[0].len();
        let mut min_heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
        min_heap.push(Reverse((0, 0, 0))); // (diff, row, col)
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let directions: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some(Reverse((diff, r, c))) = min_heap.pop() {
            if visited.contains(&(r, c)) {
                continue;
            }

            visited.insert((r, c));

            if r == rows - 1 && c == cols - 1 {
                return diff;
            }

            for (dr, dc) in &directions {
                let new_r = r as i32 + dr;
                let new_c = c as i32 + dc;

                if new_r < 0 || new_c < 0 || new_r >= rows as i32 || new_c >= cols as i32 {
                    continue;
                }

                let new_r = new_r as usize;
                let new_c = new_c as usize;

                if visited.contains(&(new_r, new_c)) {
                    continue;
                }

                let new_diff = diff.max((heights[r][c] - heights[new_r][new_c]).abs());
                min_heap.push(Reverse((new_diff, new_r, new_c)));
            }
        }

        0
    }
}
