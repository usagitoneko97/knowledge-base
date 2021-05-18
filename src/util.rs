#[derive(Default)]
pub struct BiCycle {
    pub total_len: usize,
    pub current_item: usize,
}

impl BiCycle {
    pub fn new(total_len: usize) -> Self {
        BiCycle {
            total_len,
            current_item: 0,
        }
    }

    pub fn next(&mut self) -> Option<usize> {
        self.current_item = if self.current_item >= self.total_len - 1 {
            0
        } else {
            self.current_item + 1
        };
        Some(self.current_item)
    }

    pub fn prev(&mut self) -> Option<usize> {
        self.current_item = if self.current_item == 0 {
            self.total_len - 1
        } else {
            self.current_item - 1
        };
        Some(self.current_item)
    }
}

