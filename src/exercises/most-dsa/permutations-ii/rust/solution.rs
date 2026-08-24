impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut nums = nums;
        nums.sort();
        Self::dfs(&mut nums, 0, &mut res);
        res
    }

    fn dfs(nums: &mut Vec<i32>, i: usize, res: &mut Vec<Vec<i32>>) {
        if i == nums.len() {
            res.push(nums.clone());
            return;
        }

        for j in i..nums.len() {
            if j > i && nums[i] == nums[j] {
                continue;
            }

            nums.swap(i, j);
            Self::dfs(nums, i + 1, res);
        }

        // Restore the array
        for j in (i + 1..nums.len()).rev() {
            nums.swap(j, i);
        }
    }
}
