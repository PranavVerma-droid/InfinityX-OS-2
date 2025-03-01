//! Memory management module for InfinityX OS V2

pub mod paging;

use crate::logging;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(pub u64);

pub const PAGE_SIZE: usize = 4096;

pub fn init() {
    logging::info("Initializing memory management...");
    
    unsafe {
        paging::init();
    }
    
    logging::info("Memory management initialized successfully");
}

#[inline]
pub fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

#[inline]
pub fn align_down(addr: usize, align: usize) -> usize {
    addr & !(align - 1)
}

#[inline]
pub fn is_aligned(addr: usize, align: usize) -> bool {
    addr & (align - 1) == 0
}
