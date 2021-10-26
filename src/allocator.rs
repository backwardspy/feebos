use core::alloc::Layout;
use linked_list_allocator::LockedHeap;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

use crate::serial_println;

pub const HEAP_START: usize = 0x4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!(
        "Allocation error: {:?}\nConsider increasing the heap size in allocator.rs",
        layout
    )
}

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    serial_println!("Initialising heap...");

    // get enough pages to cover the required heap size
    let page_range = {
        let first_address = VirtAddr::new(HEAP_START as u64);
        let last_address = first_address + HEAP_SIZE - 1u64;
        let first_page = Page::<Size4KiB>::containing_address(first_address);
        let last_page = Page::<Size4KiB>::containing_address(last_address);
        Page::range_inclusive(first_page, last_page)
    };

    // allocate a frame for each page and map it in the page table.
    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush();
        }
    }

    // resize the heap allocator
    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}
