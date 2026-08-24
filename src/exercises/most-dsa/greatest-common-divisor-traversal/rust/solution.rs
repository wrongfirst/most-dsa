use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n == 1 {
            return true;
        }
        if nums.iter().any(|&x| x == 1) {
            return false;
        }

        let max_val = *nums.iter().max().unwrap() as usize;
        let mut sieve = vec![0usize; max_val + 1];

        let mut p = 2;
        while p * p <= max_val {
            if sieve[p] == 0 {
                let mut composite = p * p;
                while composite <= max_val {
                    sieve[composite] = p;
                    composite += p;
                }
            }
            p += 1;
        }

        let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();

        for i in 0..n {
            let mut num = nums[i] as usize;
            if sieve[num] == 0 {
                // num is prime
                adj.entry(i).or_default().push(n + num);
                adj.entry(n + num).or_default().push(i);
                continue;
            }

            while num > 1 {
                let prime = if sieve[num] != 0 { sieve[num] } else { num };
                adj.entry(i).or_default().push(n + prime);
                adj.entry(n + prime).or_default().push(i);
                while num % prime == 0 {
                    num /= prime;
                }
            }
        }

        let mut visited: HashSet<usize> = HashSet::new();
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(0);
        visited.insert(0);

        while let Some(node) = queue.pop_front() {
            if let Some(neighbors) = adj.get(&node) {
                for &nei in neighbors {
                    if !visited.contains(&nei) {
                        visited.insert(nei);
                        queue.push_back(nei);
                    }
                }
            }
        }

        for i in 0..n {
            if !visited.contains(&i) {
                return false;
            }
        }
        true
    }
}
