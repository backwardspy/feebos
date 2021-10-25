use core::fmt;

use uart_16550::SerialPort;

const SERIAL_IO_PORT: u16 = 0x3F8;

pub static SERIAL: spin::Mutex<SerialPort> =
    spin::Mutex::new(unsafe { SerialPort::new(SERIAL_IO_PORT) });

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::serial_writer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ({
        $crate::serial_print!("{}\n", format_args!($($arg)*));
    })
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    x86_64::instructions::interrupts::without_interrupts(|| {
        SERIAL.lock().write_fmt(args).unwrap();
    });
}
