impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort();
        let mut res = 0;
        let mut l = 0;
        let mut r = people.len() as i32 - 1;
        while l <= r {
            let remain = limit - people[r as usize];
            r -= 1;
            res += 1;
            if l <= r && remain >= people[l as usize] {
                l += 1;
            }
        }
        res
    }
}
