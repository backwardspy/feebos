#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use feebos::{exit_qemu, serial_println, QemuExitCode};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

entry_point!(test_entry_point);

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(feebos::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    serial_println!("EXCEPTION: DOUBLE FAULT ({})", error_code);
    exit_qemu(QemuExitCode::Success);
}

fn test_entry_point(_: &'static mut BootInfo) -> ! {
    feebos::gdt::init();
    init_test_idt();
    stack_overflow();
    panic!("execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    let v = 0;
    volatile::Volatile::new(&v).read();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    feebos::test_panic_handler(info)
}
