impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let mut l: i32 = 0;
        let mut r: i32 = chars.len() as i32 - 1;

        while l < r {
            if chars[l as usize] != chars[r as usize] {
                return Self::is_palindrome(&chars, l + 1, r)
                    || Self::is_palindrome(&chars, l, r - 1);
            }
            l += 1;
            r -= 1;
        }

        true
    }

    fn is_palindrome(chars: &[char], mut l: i32, mut r: i32) -> bool {
        while l < r {
            if chars[l as usize] != chars[r as usize] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}
