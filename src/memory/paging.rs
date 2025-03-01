use core::arch::asm;
use crate::logging;

#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageTableFlags {
    Present =       1 << 0,
    Writable =      1 << 1,
    UserAccessible = 1 << 2,
    WriteThrough =  1 << 3,
    NoCache =       1 << 4,
    Accessed =      1 << 5,
    Dirty =         1 << 6,
    HugePage =      1 << 7,
    Global =        1 << 8,
    NoExecute =     1 << 63,
}

#[repr(align(4096))]
#[repr(C)]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PageTableEntry(pub u64);

impl PageTableEntry {
    pub fn new(addr: u64, flags: u64) -> Self {
        PageTableEntry((addr & 0x000fffff_fffff000) | flags)
    }
    
    pub fn is_present(&self) -> bool {
        self.0 & PageTableFlags::Present as u64 != 0
    }
    
    pub fn physical_address(&self) -> u64 {
        self.0 & 0x000fffff_fffff000
    }
    
    pub fn flags(&self) -> u64 {
        self.0 & 0xfff0000000000fff
    }
}

pub unsafe fn init() {
    logging::debug("Setting up paging structures...");
    let mut pml4 = create_page_table();

    identity_map_first_mb(&mut pml4);
    
    load_page_tables(&pml4);
    
    logging::debug("Paging initialized successfully");
}

fn create_page_table() -> PageTable {
    let mut table = PageTable {
        entries: [PageTableEntry(0); 512],
    };
    
    table
}

fn identity_map_first_mb(pml4: &mut PageTable) {
    logging::debug("Identity mapping first 1MB of memory...");

    let mut pdpt = create_page_table();
    let mut pd = create_page_table();
    let mut pt = create_page_table();

    pml4.entries[0] = PageTableEntry::new(
        &pdpt as *const _ as u64,
        PageTableFlags::Present as u64 | PageTableFlags::Writable as u64
    );
    
    pdpt.entries[0] = PageTableEntry::new(
        &pd as *const _ as u64,
        PageTableFlags::Present as u64 | PageTableFlags::Writable as u64
    );
    
    pd.entries[0] = PageTableEntry::new(
        &pt as *const _ as u64,
        PageTableFlags::Present as u64 | PageTableFlags::Writable as u64
    );
    
    for i in 0..256 {
        pt.entries[i] = PageTableEntry::new(
            (i * 4096) as u64,
            PageTableFlags::Present as u64 | PageTableFlags::Writable as u64
        );
    }
    
    logging::debug("First 1MB identity mapped successfully");
}

unsafe fn load_page_tables(pml4: &PageTable) {
    logging::debug("Loading page tables into CR3...");
    
    let pml4_addr = pml4 as *const _ as u64;
    
    asm!(
        "mov cr3, {}",
        in(reg) pml4_addr,
        options(nostack, preserves_flags)
    );
    
    logging::debug("Page tables loaded successfully");
}
