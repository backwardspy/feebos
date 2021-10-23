use core::fmt;

pub const BUFFER_WIDTH: u32 = 80;
pub const BUFFER_HEIGHT: u32 = 60;
pub const BUFFER_SIZE: usize = (BUFFER_WIDTH * BUFFER_HEIGHT) as usize;

const AUTOSCROLL_LINES: u32 = 2;

pub struct OutputBuffer {
    chars: [char; BUFFER_SIZE],
    pub cursor: usize,
}

impl OutputBuffer {
    pub fn new() -> OutputBuffer {
        OutputBuffer {
            chars: [' '; BUFFER_SIZE],
            cursor: 0,
        }
    }

    pub fn at(&self, x: u32, y: u32) -> char {
        self.chars[(y * BUFFER_WIDTH + x) as usize]
    }

    pub fn putc(&mut self, c: char) {
        self.chars[self.cursor] = c;
        self.cursor += 1;
        if self.cursor >= BUFFER_SIZE {
            self.scroll(AUTOSCROLL_LINES);
        }
    }

    pub fn puts(&mut self, s: &str) {
        for c in s.chars() {
            self.putc(c);
        }
    }

    pub fn scroll(&mut self, lines: u32) {
        let shift = (BUFFER_WIDTH * lines) as usize;
        for i in (shift..BUFFER_SIZE).rev() {
            self.chars[i - shift] = self.chars[i];
        }
        self.cursor -= shift;
    }
}

impl fmt::Write for OutputBuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.puts(s);
        Ok(())
    }
}
