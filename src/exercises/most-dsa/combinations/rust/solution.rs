impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::backtrack(1, n, k, &mut Vec::new(), &mut res);
        res
    }

    fn backtrack(start: i32, n: i32, k: i32, comb: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if comb.len() == k as usize {
            res.push(comb.clone());
            return;
        }

        for i in start..=n {
            comb.push(i);
            Self::backtrack(i + 1, n, k, comb, res);
            comb.pop();
        }
    }
}
