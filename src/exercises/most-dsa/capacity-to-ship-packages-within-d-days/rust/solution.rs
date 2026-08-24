impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut l = *weights.iter().max().unwrap();
        let mut r: i32 = weights.iter().sum();
        let mut res = r;

        while l <= r {
            let cap = (l + r) / 2;
            if Self::can_ship(&weights, days, cap) {
                res = res.min(cap);
                r = cap - 1;
            } else {
                l = cap + 1;
            }
        }
        res
    }

    fn can_ship(weights: &[i32], days: i32, cap: i32) -> bool {
        let mut ships = 1;
        let mut curr_cap = cap;
        for &w in weights {
            if curr_cap - w < 0 {
                ships += 1;
                if ships > days {
                    return false;
                }
                curr_cap = cap;
            }
            curr_cap -= w;
        }
        true
    }
}
