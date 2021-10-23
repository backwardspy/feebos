use core::fmt;

use uart_16550::SerialPort;

use crate::kernel::KERNEL;

pub struct SerialWriter {
    port: SerialPort,
}

impl SerialWriter {
    pub fn new(port: u16) -> SerialWriter {
        let mut port = unsafe { SerialPort::new(port) };
        port.init();
        SerialWriter { port }
    }
}

impl fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.port.send(c as u8);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::serial_writer::_print(format_args!($($arg)*)));
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
    KERNEL.lock().serial_writer.write_fmt(args).unwrap();
}
