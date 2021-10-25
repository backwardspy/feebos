#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(feebos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use feebos::{graphics::Color, halt_loop, kernel::k, serial_println};

extern crate font8x8;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    k().init(boot_info);
    serial_println!("feebos kernel initialised");

    #[cfg(test)]
    test_main();

    k().gfx.clear(Color::BLACK);
    k().gfx
        .text("welcome to feebos", 10, 10, Color::WHITE, Color::BLACK);

    halt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);

    // // BSOD! :D
    // const FG: Color = Color::WHITE;
    // const BG: Color = Color::DARKBLUE;
    // k().gfx.clear(BG);
    // k().gfx.text(":(", 10, 10, FG, BG);
    // k().gfx
    //     .text("something has gone horribly wrong.", 10, 50, FG, BG);
    // k().gfx.text("please reboot your computer.", 10, 70, FG, BG);

    // // TODO: print the stack trace. ideally we'd like to allocate a string for this.
    // k().gfx.text(
    //     "check serial output for error details",
    //     10,
    //     150,
    //     Color::ORANGE,
    //     BG,
    // );

    halt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    feebos::test_panic_handler(info)
}
