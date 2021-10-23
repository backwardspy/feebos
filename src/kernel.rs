use bootloader::BootInfo;
use lazy_static::lazy_static;
use spin::{Mutex, MutexGuard};

use crate::graphics::GraphicsContext;
use crate::serial_writer::SerialWriter;

const SERIAL_IO_PORT: u16 = 0x3F8;

pub struct Kernel {
    pub gfx: GraphicsContext<'static>,
    pub serial_writer: SerialWriter,
}

lazy_static! {
    pub static ref KERNEL: Mutex<Kernel> = Mutex::new(Kernel {
        gfx: GraphicsContext::new(),
        serial_writer: SerialWriter::new(SERIAL_IO_PORT),
    });
}

impl Kernel {
    pub fn init(&mut self, boot_info: &'static mut BootInfo) {
        if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
            self.gfx.set_framebuffer(framebuffer);
        }
    }
}

// shorthand for accessing the kernel
pub fn k() -> MutexGuard<'static, Kernel> {
    KERNEL.lock()
}
