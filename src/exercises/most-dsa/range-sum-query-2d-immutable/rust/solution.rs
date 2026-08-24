struct NumMatrix {
    sum_mat: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut sum_mat = vec![vec![0; cols + 1]; rows + 1];

        for r in 0..rows {
            let mut prefix = 0;
            for c in 0..cols {
                prefix += matrix[r][c];
                let above = sum_mat[r][c + 1];
                sum_mat[r + 1][c + 1] = prefix + above;
            }
        }

        NumMatrix { sum_mat }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let r1 = row1 as usize + 1;
        let c1 = col1 as usize + 1;
        let r2 = row2 as usize + 1;
        let c2 = col2 as usize + 1;

        let bottom_right = self.sum_mat[r2][c2];
        let above = self.sum_mat[r1 - 1][c2];
        let left = self.sum_mat[r2][c1 - 1];
        let top_left = self.sum_mat[r1 - 1][c1 - 1];

        bottom_right - above - left + top_left
    }
}
