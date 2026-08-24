impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n <= 1 {
            return nums;
        }
        Self::merge_sort(&mut nums, 0, n - 1);
        nums
    }

    fn merge_sort(arr: &mut Vec<i32>, l: usize, r: usize) {
        if l >= r {
            return;
        }

        let m = l + (r - l) / 2;
        Self::merge_sort(arr, l, m);
        Self::merge_sort(arr, m + 1, r);
        Self::merge(arr, l, m, r);
    }

    fn merge(arr: &mut Vec<i32>, l: usize, m: usize, r: usize) {
        let left: Vec<i32> = arr[l..=m].to_vec();
        let right: Vec<i32> = arr[m + 1..=r].to_vec();

        let mut i = l;
        let mut j = 0;
        let mut k = 0;

        while j < left.len() && k < right.len() {
            if left[j] <= right[k] {
                arr[i] = left[j];
                j += 1;
            } else {
                arr[i] = right[k];
                k += 1;
            }
            i += 1;
        }

        while j < left.len() {
            arr[i] = left[j];
            j += 1;
            i += 1;
        }

        while k < right.len() {
            arr[i] = right[k];
            k += 1;
            i += 1;
        }
    }
}
