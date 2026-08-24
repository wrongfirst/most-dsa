use std::collections::VecDeque;

struct MyStack {
    q: VecDeque<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        MyStack {
            q: VecDeque::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.q.push_back(x);
        // Rotate all elements except the last one to the back
        for _ in 0..self.q.len() - 1 {
            let front = self.q.pop_front().unwrap();
            self.q.push_back(front);
        }
    }

    pub fn pop(&mut self) -> i32 {
        self.q.pop_front().unwrap()
    }

    pub fn top(&self) -> i32 {
        *self.q.front().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.q.is_empty()
    }
}
