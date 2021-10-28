#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(feebos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use feebos::{
    graphics::{self, Color},
    halt_loop,
    kernel::k,
    println,
    text_buffer::SHELL,
};

entry_point!(kernel_main);

const SHELL_PADDING: u32 = 8;
const SHELL_LINE_SPACING: u32 = 2;

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    k().init(boot_info);

    #[cfg(test)]
    test_main();

    let width = k().gfx.width();
    let height = k().gfx.height();
    let (buf_width, buf_height) =
        graphics::calculate_text_buffer_size(width, height, SHELL_PADDING, SHELL_LINE_SPACING);
    SHELL.lock().resize(buf_width, buf_height);

    println!("welcome to feebos");

    k().gfx.clear(Color::BLACK);
    k().gfx.text_buffer(
        &mut SHELL.lock(),
        SHELL_PADDING,
        SHELL_LINE_SPACING,
        Color::WHITE,
        Color::BLACK,
    );

    halt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    const FG: Color = Color::WHITE;
    const BG: Color = Color::DARKBLUE;

    SHELL.lock().clear();
    println!(":<\n");
    println!("something has gone horribly wrong.");
    println!("please reboot your computer.\n\n");
    println!("{}", info);

    k().gfx.clear(BG);
    k().gfx
        .text_buffer(&mut SHELL.lock(), SHELL_PADDING, SHELL_LINE_SPACING, FG, BG);

    halt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    feebos::test_panic_handler(info)
}
