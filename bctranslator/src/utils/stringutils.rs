

pub struct StringBuilder {
    buffer: Vec<char>
}

impl StringBuilder {
    pub fn new() -> StringBuilder {
        Self {
            buffer: Vec::new()
        }
    }
    pub fn push(&mut self, c: char) {
        self.buffer.push(c)
    }
    pub fn push_str(&mut self, s: &str) {
        self.buffer.append(&mut s.chars().collect())
    }
    pub fn pack(&mut self) -> String {
        String::from_iter(&self.buffer)
    }
    pub fn clear(&mut self) {
        self.buffer.clear()
    }
}