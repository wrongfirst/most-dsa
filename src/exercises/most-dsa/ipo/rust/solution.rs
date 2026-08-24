use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let n = profits.len();
        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_by_key(|&i| capital[i]);

        let mut max_profit: BinaryHeap<i32> = BinaryHeap::new();
        let mut idx = 0;
        let mut current_capital = w;

        for _ in 0..k {
            while idx < n && capital[indices[idx]] <= current_capital {
                max_profit.push(profits[indices[idx]]);
                idx += 1;
            }

            if let Some(profit) = max_profit.pop() {
                current_capital += profit;
            } else {
                break;
            }
        }

        current_capital
    }
}
