#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use feebos::{exit_qemu, kernel::k, serial_print, serial_println, QemuExitCode};

entry_point!(test_entry_point);

fn test_entry_point(boot_info: &'static mut BootInfo) -> ! {
    k().init(boot_info);
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
}

fn should_fail() {
    serial_print!("{:.<76}", "should_panic::should_fail");
    assert_eq!(1, 0);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
}
