impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut n = n;

        // Reduce n by factors of 4
        while n % 4 == 0 {
            n /= 4;
        }

        // Check if n % 8 == 7 (Legendre's three-square theorem)
        if n % 8 == 7 {
            return 4;
        }

        // Check if n is a perfect square
        let is_square = |num: i32| -> bool {
            let s = (num as f64).sqrt() as i32;
            s * s == num
        };

        if is_square(n) {
            return 1;
        }

        // Check if n can be expressed as sum of two squares
        let mut i = 1;
        while i * i <= n {
            if is_square(n - i * i) {
                return 2;
            }
            i += 1;
        }

        3
    }
}
