impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut l = 0;
        let mut r = s.len() as i32 - 1;
        while l < r {
            s.swap(l as usize, r as usize);
            l += 1;
            r -= 1;
        }
    }
}
