#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate font8x8;

mod fixed_buffer;
mod graphics;
mod kernel;
mod output_buffer;
mod serial_writer;

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

use graphics::Color;
use kernel::k;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    k().init(boot_info);

    #[cfg(test)]
    test_main();

    println!("feebos started up successfully.");
    k().gfx.clear(Color::BLACK);
    k().gfx
        .text("welcome to feebos", 10, 10, Color::LIME, Color::BLACK);

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use core::fmt::Write;
    use fixed_buffer::FixedBuffer;

    println!("{}", info);

    // BSOD! :D
    const FG: Color = Color::WHITE;
    const BG: Color = Color::DARKBLUE;
    k().gfx.clear(BG);
    k().gfx.text(":(", 10, 10, FG, BG);
    k().gfx
        .text("something has gone horribly wrong.", 10, 50, FG, BG);
    k().gfx.text("please reboot your computer.", 10, 70, FG, BG);

    let mut panic_info_buffer = FixedBuffer::new();
    write!(panic_info_buffer, "{}", info).unwrap();
    k().gfx
        .text(panic_info_buffer.as_str(), 10, 150, Color::ORANGE, BG);

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests...", tests.len());
    for test in tests {
        test();
    }
    println!("[success]");
    exit_qemu(QemuExitCode::Success);
}

#[cfg(test)]
fn exit_qemu(exit_code: QemuExitCode) -> ! {
    use x86_64::instructions::port::Port;

    println!("Exiting qemu.");
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }

    loop {}
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
