use linked_list_allocator::LockedHeap;

pub const HEAP_START: usize = 0x_4444_4444_0000; // simple start address
pub const HEAP_SIZE: usize = 100 * 1024; // 100KiB

use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>, // we limit both to 4KiB, heaps are usually 4KiB
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?; // error handler, should return None when the frames are empty
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE; // we want the frames to be writable, otherwise this whole method was for nothing
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush() // the ? is so it returns early when an error occurs
        };
    }

    // prevent a deadlock when mutex or a spinlock is being run
    // the allocator gets run after the heap is created
    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty(); //LockedHeap uses the SpinLock to sync, something we already use for Mutex
