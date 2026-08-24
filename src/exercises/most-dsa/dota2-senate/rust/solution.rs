impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut senate: Vec<char> = senate.chars().collect();
        let mut cnt: i32 = 0;
        let mut i = 0;

        while i < senate.len() {
            let c = senate[i];
            if c == 'R' {
                if cnt < 0 {
                    senate.push('D');
                }
                cnt += 1;
            } else {
                if cnt > 0 {
                    senate.push('R');
                }
                cnt -= 1;
            }
            i += 1;
        }

        if cnt > 0 {
            "Radiant".to_string()
        } else {
            "Dire".to_string()
        }
    }
}
