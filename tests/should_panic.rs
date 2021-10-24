#![no_std]
#![no_main]

use core::panic::PanicInfo;

use feebos::{exit_qemu, println, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
}

fn should_fail() {
    println!("should_panic::should_fail...\t");
    assert_eq!(1, 0);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("[ok]");
    exit_qemu(QemuExitCode::Success);
}
