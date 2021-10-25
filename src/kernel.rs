use bootloader::BootInfo;
use lazy_static::lazy_static;
use spin::{Mutex, MutexGuard};

use crate::{gdt, graphics::GraphicsContext, interrupts};

pub struct Kernel {
    pub gfx: GraphicsContext<'static>,
}

lazy_static! {
    pub static ref KERNEL: Mutex<Kernel> = Mutex::new(Kernel {
        gfx: GraphicsContext::new(),
    });
}

impl Kernel {
    pub fn init(&mut self, boot_info: &'static mut BootInfo) {
        // load GDT and TSS
        gdt::init();

        // load IDT and initialise PICs
        interrupts::init();

        // enable interrupts
        x86_64::instructions::interrupts::enable();

        if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
            self.gfx.set_framebuffer(framebuffer);
        }
    }
}

// shorthand for accessing the kernel
pub fn k() -> MutexGuard<'static, Kernel> {
    KERNEL.lock()
}
