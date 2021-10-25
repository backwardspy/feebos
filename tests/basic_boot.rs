#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(feebos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

use feebos::{halt_loop, kernel::k};

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
fn test_simple_assertion() {
    assert_eq!(1, 1);
}
