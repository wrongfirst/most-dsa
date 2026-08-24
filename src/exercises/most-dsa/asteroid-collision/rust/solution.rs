impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();

        for mut a in asteroids {
            while !stack.is_empty() && *stack.last().unwrap() > 0 && a < 0 {
                let top = *stack.last().unwrap();
                if top > a.abs() {
                    a = 0;
                    break;
                } else if top == a.abs() {
                    stack.pop();
                    a = 0;
                    break;
                } else {
                    stack.pop();
                }
            }
            if a != 0 {
                stack.push(a);
            }
        }

        stack
    }
}
