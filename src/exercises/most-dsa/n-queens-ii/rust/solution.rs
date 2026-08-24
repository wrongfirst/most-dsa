impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut col: u32 = 0;
        let mut pos_diag: u32 = 0;
        let mut neg_diag: u32 = 0;
        let mut res = 0;

        fn backtrack(
            r: usize,
            n: usize,
            col: &mut u32,
            pos_diag: &mut u32,
            neg_diag: &mut u32,
            res: &mut i32,
        ) {
            if r == n {
                *res += 1;
                return;
            }
            for c in 0..n {
                if (*col & (1 << c)) != 0
                    || (*pos_diag & (1 << (r + c))) != 0
                    || (*neg_diag & (1 << (r + n - c))) != 0
                {
                    continue;
                }
                *col ^= 1 << c;
                *pos_diag ^= 1 << (r + c);
                *neg_diag ^= 1 << (r + n - c);

                backtrack(r + 1, n, col, pos_diag, neg_diag, res);

                *col ^= 1 << c;
                *pos_diag ^= 1 << (r + c);
                *neg_diag ^= 1 << (r + n - c);
            }
        }

        backtrack(0, n, &mut col, &mut pos_diag, &mut neg_diag, &mut res);
        res
    }
}
