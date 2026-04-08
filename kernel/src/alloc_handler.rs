use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;
use buddy_slab_allocator::GlobalAllocator;
use spin::RwLock;

#[derive(Default)]
pub struct Alloc(pub(crate) RwLock<GlobalAllocator>);


impl Alloc {
    const fn new() -> Self{
        let allocator =GlobalAllocator::new();
        Self(RwLock::new(allocator))
    }

    pub(crate) fn init(&self) {
        self.0.write().init(HEAP_START, HEAP_SIZE).unwrap();
    }
}

unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.write().alloc(layout).unwrap().as_ptr()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.write().dealloc(NonNull::new(ptr).unwrap(), layout)
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        self.0.write().realloc(ptr, layout, new_size)
    }
}

#[global_allocator]
pub static GLOBAL: Alloc = Alloc::new();
const HEAP_START: usize = 0x8000_0000;
const HEAP_SIZE: usize = 0x4000_0000;

