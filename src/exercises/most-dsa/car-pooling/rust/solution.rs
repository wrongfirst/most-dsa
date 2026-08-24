use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn car_pooling(mut trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        trips.sort_by_key(|t| t[1]);

        let mut min_heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        let mut cur_pass = 0;

        for trip in trips {
            let num_pass = trip[0];
            let start = trip[1];
            let end = trip[2];

            while let Some(&Reverse((end_time, passengers))) = min_heap.peek() {
                if end_time <= start {
                    cur_pass -= passengers;
                    min_heap.pop();
                } else {
                    break;
                }
            }

            cur_pass += num_pass;
            if cur_pass > capacity {
                return false;
            }

            min_heap.push(Reverse((end, num_pass)));
        }

        true
    }
}
