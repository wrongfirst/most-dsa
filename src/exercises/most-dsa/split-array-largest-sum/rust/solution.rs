impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut prefix = vec![0i64; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i] as i64;
        }

        let can_split = |largest: i64| -> bool {
            let mut subarrays = 0;
            let mut i = 0;
            while i < n {
                let mut l = i + 1;
                let mut r = n;
                while l <= r {
                    let mid = l + (r - l) / 2;
                    if prefix[mid] - prefix[i] <= largest {
                        l = mid + 1;
                    } else {
                        r = mid - 1;
                    }
                }
                subarrays += 1;
                i = r;
                if subarrays > k {
                    return false;
                }
            }
            true
        };

        let mut l = *nums.iter().max().unwrap() as i64;
        let mut r = nums.iter().map(|&x| x as i64).sum::<i64>();
        let mut res = r;

        while l <= r {
            let mid = l + (r - l) / 2;
            if can_split(mid) {
                res = mid;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }

        res as i32
    }
}
