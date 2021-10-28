use crate::{
    allocator, gdt,
    graphics::GraphicsContext,
    interrupts,
    memory::{self, MemoryRegionsFrameAllocator},
};
use bootloader::BootInfo;
use lazy_static::lazy_static;
use spin::{Mutex, MutexGuard};
use x86_64::VirtAddr;

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

        // initialise a mapper and frame allocator
        let physical_memory_offset =
            VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
        let mut mapper = unsafe { memory::init(physical_memory_offset) };
        let mut frame_allocator =
            unsafe { MemoryRegionsFrameAllocator::init(&boot_info.memory_regions) };

        // initialise the heap allocator
        allocator::init_heap(&mut mapper, &mut frame_allocator)
            .expect("heap initialisation failed");

        if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
            self.gfx.set_framebuffer(framebuffer);
        }
    }
}

// shorthand for accessing the kernel
pub fn k() -> MutexGuard<'static, Kernel> {
    KERNEL.lock()
}
