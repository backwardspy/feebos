use alloc::vec::Vec;
use core::fmt;
use lazy_static::lazy_static;

pub struct TextBuffer {
    pub width: usize,
    pub height: usize,
    chars: Vec<char>,
    dirty: Vec<bool>,
    cursor: usize,
}

lazy_static! {
    pub static ref SHELL: spin::Mutex<TextBuffer> = spin::Mutex::new(TextBuffer::empty());
}

impl TextBuffer {
    pub fn empty() -> Self {
        Self::new(0, 0)
    }

    pub fn new(width: usize, height: usize) -> Self {
        let capacity = width * height;
        Self {
            width,
            height,
            chars: vec![' '; capacity],
            dirty: vec![false; capacity],
            cursor: 0,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        let capacity = width * height;
        self.chars.resize(capacity, ' ');
        self.dirty.resize(capacity, false);
        self.cursor = 0;
    }

    pub fn clear(&mut self) {
        for i in 0..self.capacity() {
            self.chars[i] = ' ';
            self.dirty[i] = true;
        }
    }

    pub fn insert(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            _ => {
                self.chars[self.cursor] = c;
                self.dirty[self.cursor] = true;
                self.cursor += 1;
                self.scroll_if_necessary()
            }
        }
    }

    pub fn delete(&mut self) {
        self.cursor -= 1;
        self.chars[self.cursor] = ' ';
    }

    pub fn newline(&mut self) {
        self.cursor = (1 + self.cursor / self.width) * self.width;
        self.scroll_if_necessary()
    }

    pub fn move_cursor(&mut self, offset: isize) {
        self.cursor = (self.cursor as isize + offset) as usize;
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn at(&self, cursor: usize) -> char {
        self.chars[cursor]
    }

    pub fn capacity(&self) -> usize {
        self.width * self.height
    }

    pub fn dirty(&self) -> impl Iterator<Item = usize> + '_ {
        self.dirty
            .iter()
            .enumerate()
            .filter(|(_, dirty)| **dirty)
            .map(|(i, _)| i)
    }

    fn scroll_if_necessary(&mut self) {
        if self.cursor >= self.capacity() {
            self.scroll(2);
        }
    }

    pub fn scroll(&mut self, lines: usize) {
        let scoop = lines * self.width;
        let end = self.capacity() - scoop;
        for i in 0..end {
            self.chars[i] = self.chars[i + scoop];
        }
        for i in 0..scoop {
            self.chars[end + i] = ' ';
        }
        self.mark_all_dirty();
        self.cursor -= scoop;
    }

    fn mark_all_dirty(&mut self) {
        for cell in &mut self.dirty {
            *cell = true;
        }
    }

    pub fn mark_all_clean(&mut self) {
        for cell in &mut self.dirty {
            *cell = false;
        }
    }
}

impl fmt::Write for TextBuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.insert(c);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::text_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::print!("{}\n", format_args!($($arg)*));
    })
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    x86_64::instructions::interrupts::without_interrupts(|| {
        SHELL.lock().write_fmt(args).unwrap();
    });
}
