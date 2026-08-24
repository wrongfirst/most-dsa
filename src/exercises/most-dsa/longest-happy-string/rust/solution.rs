impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut count = [a, b, c];
        let mut res = Vec::new();

        let get_max = |count: &[i32; 3], repeated: i32| -> i32 {
            let mut idx = -1;
            let mut max_cnt = 0;
            for i in 0..3 {
                if i as i32 == repeated || count[i] == 0 {
                    continue;
                }
                if max_cnt < count[i] {
                    max_cnt = count[i];
                    idx = i as i32;
                }
            }
            idx
        };

        let mut repeated = -1;
        loop {
            let max_char = get_max(&count, repeated);
            if max_char == -1 {
                break;
            }
            res.push((b'a' + max_char as u8) as char);
            count[max_char as usize] -= 1;

            if res.len() > 1 && res[res.len() - 1] == res[res.len() - 2] {
                repeated = max_char;
            } else {
                repeated = -1;
            }
        }

        res.into_iter().collect()
    }
}
