#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![cfg_attr(test, no_main)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod allocator;
pub mod gdt;
pub mod graphics;
pub mod interrupts;
pub mod kernel;
pub mod memory;
pub mod serial_writer;

extern crate alloc;

use core::panic::PanicInfo;

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
use kernel::k;

pub fn halt_loop() -> ! {
    serial_println!("Kernel entering halt loop!");
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
entry_point!(test_entry_point);

#[cfg(test)]
fn test_entry_point(boot_info: &'static mut BootInfo) -> ! {
    k().init(boot_info);
    test_main();
    halt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{:.<76}", core::any::type_name::<T>());
        self();
        serial_println!("[ok]")
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

    serial_println!("Exiting qemu.");
    unsafe {
        let mut port = Port::new(0xF4);
        port.write(exit_code as u32);
    }

    halt_loop();
}

pub fn test_runner(tests: &[&dyn Testable]) -> ! {
    serial_println!("Running {} test(s)...", tests.len());
    for test in tests {
        test.run();
    }
    serial_println!("[success]");
    exit_qemu(QemuExitCode::Success);
}
