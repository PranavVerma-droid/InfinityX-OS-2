use core::panic::PanicInfo;
use core::arch::asm;
use crate::logging::{self};

#[allow(dead_code)]
struct BootInfo {
    memory_map_tag: Option<&'static MemoryMapTag>,
    boot_stage: BootStage,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
enum BootStage {
    Start,
    CPUCheck,
    MemoryInit,
    Complete,
    Error,
}

#[repr(C)]
struct MemoryMapTag {
    entries_count: u32,
    entry_size: u32,
}

pub fn init() -> ! {
    logging::info("InfinityX OS V2 Bootloader Starting...");

    logging::info("Initializing boot info structure...");
    
    let mut boot_info = BootInfo {
        memory_map_tag: None,
        boot_stage: BootStage::Start,
    };
    
    logging::info("Checking CPU compatibility...");
    boot_info.boot_stage = BootStage::CPUCheck;
    if !check_cpu() {
        logging::critical("CPU requirements not met - CPUID not available");
        boot_error("CPU requirements not met - CPUID not available");
    }


    logging::info("Detecting system memory...");
    boot_info.boot_stage = BootStage::MemoryInit;
    if let Err(_) = detect_memory(&mut boot_info) {
        logging::critical("Memory detection failed - Cannot access memory map");
        boot_error("Memory detection failed - Cannot access memory map");
    }


    logging::info("Setting up Global Descriptor Table...");
    setup_gdt();


    logging::info("Setting up Interrupt Descriptor Table...");
    setup_idt();
 
    logging::info("Finalizing boot sequence...");
    boot_info.boot_stage = BootStage::Complete;


    logging::info("Loading kernel into memory...");
    load_kernel();

}

fn check_cpu() -> bool {
    logging::debug("Checking CPU CPUID support...");
    logging::debug("Starting CPU Check...");
    unsafe {
        let mut flags1: u64;
        let mut flags2: u64;
        
        asm!(
            "pushf",
            "pop {0}",
            "mov {1}, {0}",
            "xor {0}, 0x200000",
            "push {0}",
            "popf",
            "pushf",
            "pop {0}",
            out(reg) flags1,
            out(reg) flags2,
            options(nomem, preserves_flags)
        );

        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        let result = flags1 != flags2;
        
        logging::debug(if result { 
            "CPU check result: PASSED" 
        } else { 
            "CPU check result: FAILED" 
        });
        
        result
    }
}

#[allow(unused_variables)]
fn detect_memory(boot_info: &mut BootInfo) -> Result<(), ()> {
    logging::debug("Starting memory detection...");
    logging::warn("Memory Detection loading not yet implemented");
    //TODO: Basic memory detection using BIOS e820
    Ok(())
}

fn setup_gdt() {
    logging::debug("Setting up GDT...");
    logging::warn("GDT loading not yet implemented");
    //TODO: Initialize Global Descriptor Table
}

fn setup_idt() {
    logging::debug("Setting up IDT...");
    logging::warn("IDT loading not yet implemented");
    //TODO: IDT Loading Logic
}

fn load_kernel() -> ! {
    logging::debug("Starting kernel load process...");
    logging::warn("Kernel loading not yet implemented");
    //TODO: Kernel loading logic
    loop {}
}

fn boot_error(msg: &str) -> ! {
    logging::critical("Boot halted with error");
    logging::critical(msg);
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() { 
        logging::critical("Panic occurred during boot",);
        logging::critical("Panic location:");
        logging::critical(location.file());
    } else {
        logging::critical("Fatal error occurred during boot");
    }
    
    loop {}
}
