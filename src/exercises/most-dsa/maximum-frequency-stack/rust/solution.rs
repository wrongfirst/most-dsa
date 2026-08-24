use std::collections::HashMap;

struct FreqStack {
    cnt: HashMap<i32, i32>,
    stacks: Vec<Vec<i32>>,
}

impl FreqStack {
    pub fn new() -> Self {
        FreqStack {
            cnt: HashMap::new(),
            stacks: vec![vec![]],
        }
    }

    pub fn push(&mut self, val: i32) {
        let val_cnt = 1 + *self.cnt.get(&val).unwrap_or(&0);
        self.cnt.insert(val, val_cnt);
        if val_cnt as usize == self.stacks.len() {
            self.stacks.push(vec![]);
        }
        self.stacks[val_cnt as usize].push(val);
    }

    pub fn pop(&mut self) -> i32 {
        let res = self.stacks.last_mut().unwrap().pop().unwrap();
        *self.cnt.get_mut(&res).unwrap() -= 1;
        if self.stacks.last().unwrap().is_empty() {
            self.stacks.pop();
        }
        res
    }
}
