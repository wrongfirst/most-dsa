impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for op in operations {
            match op.as_str() {
                "+" => {
                    let top = stack.pop().unwrap();
                    let new_top = top + stack.last().unwrap();
                    stack.push(top);
                    stack.push(new_top);
                }
                "D" => {
                    let doubled = 2 * stack.last().unwrap();
                    stack.push(doubled);
                }
                "C" => {
                    stack.pop();
                }
                _ => {
                    stack.push(op.parse::<i32>().unwrap());
                }
            }
        }

        stack.iter().sum()
    }
}
