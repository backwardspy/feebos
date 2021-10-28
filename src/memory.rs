use bootloader::boot_info::{MemoryRegionKind, MemoryRegions};
use x86_64::{
    registers::control::Cr3,
    structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

pub struct MemoryRegionsFrameAllocator {
    memory_regions: &'static MemoryRegions,
    next: usize,
}

impl MemoryRegionsFrameAllocator {
    /// # Safety
    /// The caller must guarantee the passed memory regions are valid & usable.
    pub unsafe fn init(memory_regions: &'static MemoryRegions) -> Self {
        Self {
            memory_regions,
            next: 0,
        }
    }

    // find all usable frames in our memory regions
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // find regions of kind "usable"
        self.memory_regions
            .iter()
            // get only regions that are usable
            .filter(|region| region.kind == MemoryRegionKind::Usable)
            // convert regions to address ranges
            .map(|region| region.start..region.end)
            // map each address range to a set of frame start addresses
            .flat_map(|address_range| address_range.step_by(4096))
            // get a PhysFrame for each frame start address
            .map(|address| PhysFrame::containing_address(PhysAddr::new(address)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for MemoryRegionsFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

/// # Safety
/// The caller must guarantee physical memory is mapped prior to calling this function.
/// The caller must only call this function once, to avoid aliasing the mut reference to the table.
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    OffsetPageTable::new(
        active_level4_table(physical_memory_offset),
        physical_memory_offset,
    )
}

unsafe fn active_level4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    // add the offset to the physical address of the L4 table and return it
    let (level4_table_frame, _) = Cr3::read();
    let physical_addrress = level4_table_frame.start_address();
    let virtual_address = physical_memory_offset + physical_addrress.as_u64();
    let level4_table_ptr: *mut PageTable = virtual_address.as_mut_ptr();

    &mut *level4_table_ptr // tremendously unsafe :)
}
