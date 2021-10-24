#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

pub mod fixed_buffer;
pub mod graphics;
pub mod kernel;
pub mod output_buffer;
pub mod serial_writer;

#[cfg(test)]
entry_point!(test_entry_point);

#[cfg(test)]
fn test_entry_point(_: &'static mut BootInfo) -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        self();
        println!("{:.<76}[ok]", core::any::type_name::<T>());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    use x86_64::instructions::port::Port;

    println!("Exiting qemu.");
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }

    loop {}
}

pub fn test_runner(tests: &[&dyn Testable]) -> ! {
    println!("Running {} tests...", tests.len());
    for test in tests {
        test.run();
    }
    println!("[success]");
    exit_qemu(QemuExitCode::Success);
}
