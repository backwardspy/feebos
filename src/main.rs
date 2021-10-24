#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(feebos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate font8x8;

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

use feebos::graphics::Color;
use feebos::kernel::k;
use feebos::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    k().init(boot_info);
    println!("feebos started up successfully.");

    #[cfg(test)]
    test_main();

    k().gfx.clear(Color::BLACK);
    k().gfx
        .text("welcome to feebos", 10, 10, Color::LIME, Color::BLACK);

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use core::fmt::Write;
    use feebos::fixed_buffer::FixedBuffer;

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
    feebos::test_panic_handler(info)
}
