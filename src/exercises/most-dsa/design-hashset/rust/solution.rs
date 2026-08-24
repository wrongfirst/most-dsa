struct MyHashSet {
    // Using bit manipulation: key is in range [0, 1000000]
    // 31251 * 32 = 1000032 which covers the range
    data: Vec<u32>,
}

impl MyHashSet {
    pub fn new() -> Self {
        MyHashSet {
            data: vec![0; 31251],
        }
    }

    fn get_mask(&self, key: i32) -> u32 {
        1 << (key % 32)
    }

    pub fn add(&mut self, key: i32) {
        let idx = (key / 32) as usize;
        let mask = self.get_mask(key);
        self.data[idx] |= mask;
    }

    pub fn remove(&mut self, key: i32) {
        if self.contains(key) {
            let idx = (key / 32) as usize;
            let mask = self.get_mask(key);
            self.data[idx] ^= mask;
        }
    }

    pub fn contains(&self, key: i32) -> bool {
        let idx = (key / 32) as usize;
        let mask = self.get_mask(key);
        (self.data[idx] & mask) != 0
    }
}
