pub struct GapBuffer {
    buffer: Vec<char>,
    gap_start: usize,
    gap_end: usize,
}

impl GapBuffer {
    pub fn new(size: usize) -> Self {
        let mut buffer = Vec::with_capacity(size);
        buffer.resize(size, ' ');

        GapBuffer {
            buffer,
            gap_start: 0,
            gap_end: size,
        }
    }

    pub fn build(initial: &str) -> Self {
        let size = initial.len();

        let mut buffer = Vec::with_capacity(size);
        buffer.extend(initial.chars());

        GapBuffer {
            buffer,
            gap_start: size,
            gap_end: size,
        }
    }

    pub fn insert(&mut self, c: char) {
        if self.gap_start == self.gap_end {
            self.resize_buffer();
        }

        self.buffer[self.gap_start] = c;
        self.gap_start += 1;
    }

    pub fn delete(&mut self) {
        if self.gap_start > 0 {
            self.gap_start -= 1;
            self.buffer[self.gap_start] = ' ';
        }
    }

    fn _move_cursor(&mut self, offset: isize) {
        let new_pos = (self.gap_start as isize + offset) as usize;
        if new_pos <= self.gap_end {
            self.gap_start = new_pos;
        }
    }

    fn resize_buffer(&mut self) {
        let old_len = self.buffer.len();
        let new_len = old_len * 2;
        self.buffer.resize(new_len, ' ');
        let gap_size = self.gap_end - self.gap_start;
        self.buffer
            .copy_within(self.gap_end..old_len, old_len + gap_size);
        self.gap_end = new_len;
    }

    pub fn to_string(&self) -> String {
        let mut result = String::with_capacity(self.buffer.len() - (self.gap_end - self.gap_start));
        result.extend(&self.buffer[..self.gap_start]);
        result.extend(&self.buffer[self.gap_end..]);
        result
    }
}
