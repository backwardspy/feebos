#![no_std]
#![no_main]

extern crate font8x8;

mod graphics_context;

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};

use graphics_context::{Color, GraphicsContext};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let mut gfx = GraphicsContext::new(framebuffer);
        gfx.clear(Color::BLACK);
        gfx.text("hello from feebos!", 8, 8, Color::WHITE, Color::BLACK);

        gfx.text(
            "sweetie16 palette from https://lospec.com/palette-list/sweetie-16",
            8,
            24,
            Color::WHITE,
            Color::BLACK,
        );

        let palette = [
            (Color::BLACK, "black"),
            (Color::PURPLE, "purple"),
            (Color::RED, "red"),
            (Color::ORANGE, "orange"),
            (Color::YELLOW, "yellow"),
            (Color::LIME, "lime"),
            (Color::GREEN, "green"),
            (Color::TEAL, "teal"),
            (Color::DARKBLUE, "darkblue"),
            (Color::BLUE, "blue"),
            (Color::LIGHTBLUE, "lightblue"),
            (Color::CYAN, "cyan"),
            (Color::WHITE, "white"),
            (Color::LIGHTGREY, "lightgrey"),
            (Color::GREY, "grey"),
            (Color::DARKGREY, "darkgrey"),
        ];

        for (i, (color, name)) in palette.iter().enumerate() {
            gfx.text(name, 8, 32 + 8 * i as u32, *color, Color::BLACK);
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
