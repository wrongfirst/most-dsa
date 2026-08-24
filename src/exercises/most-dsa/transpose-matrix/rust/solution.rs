impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = matrix.len();
        let cols = matrix[0].len();

        if rows == cols {
            let mut result = matrix.clone();
            for r in 0..rows {
                for c in 0..r {
                    let tmp = result[r][c];
                    result[r][c] = result[c][r];
                    result[c][r] = tmp;
                }
            }
            return result;
        }

        let mut res = vec![vec![0; rows]; cols];

        for r in 0..rows {
            for c in 0..cols {
                res[c][r] = matrix[r][c];
            }
        }

        res
    }
}
