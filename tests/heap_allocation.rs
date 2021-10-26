#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(feebos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use feebos::{allocator::HEAP_SIZE, halt_loop, kernel::k};

entry_point!(test_entry_point);

fn test_entry_point(boot_info: &'static mut BootInfo) -> ! {
    k().init(boot_info);
    test_main();
    halt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    feebos::test_panic_handler(info)
}

#[test_case]
fn simple_allocation() {
    let heap_value_1 = Box::new(42);
    let heap_value_2 = Box::new(13);
    assert_eq!(*heap_value_1, 42);
    assert_eq!(*heap_value_2, 13);
}

#[test_case]
fn large_vector() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}
