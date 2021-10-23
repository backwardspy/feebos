use core::fmt;

const BUFFER_SIZE: usize = 8192;

pub struct FixedBuffer {
    contents: [u8; BUFFER_SIZE],
    used: usize,
}

impl FixedBuffer {
    pub fn new() -> FixedBuffer {
        FixedBuffer {
            contents: [0; BUFFER_SIZE],
            used: 0,
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.contents[0..self.used]) }
    }
}

impl fmt::Write for FixedBuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.used + s.len() > BUFFER_SIZE {
            return Err(fmt::Error);
        }

        self.contents[self.used..self.used + s.len()].copy_from_slice(s.as_bytes());
        self.used += s.len();

        Ok(())
    }
}
