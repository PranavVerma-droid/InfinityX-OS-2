use core::panic::PanicInfo;
use core::arch::asm;
use core::ptr::write_volatile;

const VGA_BUFFER_ADDR: *mut u8 = 0xb8000 as *mut u8;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;
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
    let mut boot_info = BootInfo {
        memory_map_tag: None,
        boot_stage: BootStage::Start,
    };

    clear_screen();
    print_at(0, 0, "InfinityX OS V2 Bootloader");
    
    // CPU Check
    boot_info.boot_stage = BootStage::CPUCheck;
    if !check_cpu() {
        boot_error("CPU requirements not met");
    }
    print_at(0, 1, "CPU Check: OK");

    // Memory Detection
    boot_info.boot_stage = BootStage::MemoryInit;
    if let Err(_) = detect_memory(&mut boot_info) {
        boot_error("Memory detection failed");
    }
    print_at(0, 2, "Memory Detection: OK");

    // Setup GDT and IDT
    setup_gdt();
    setup_idt();
    print_at(0, 3, "Tables Initialized: OK");

    boot_info.boot_stage = BootStage::Complete;
    print_at(0, 5, "Boot Complete - Loading Kernel...");

    load_kernel()
}

fn check_cpu() -> bool {
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
        flags1 != flags2
    }
}

#[allow(unused_variables)]
fn detect_memory(boot_info: &mut BootInfo) -> Result<(), ()> {
    //TODO: Basic memory detection using BIOS e820
    Ok(())
}

fn setup_gdt() {
    //TODO: Initialize Global Descriptor Table
}

fn setup_idt() {
    //TODO: Initialize Interrupt Descriptor Table
}

fn load_kernel() -> ! {
    //TODO: Kernel loading logic
    loop {}
}

fn clear_screen() {
    for i in 0..VGA_WIDTH * VGA_HEIGHT {
        unsafe {
            write_volatile(VGA_BUFFER_ADDR.offset(i as isize * 2), b' ');
            write_volatile(VGA_BUFFER_ADDR.offset(i as isize * 2 + 1), 0x07);
        }
    }
}

fn print_at(x: usize, y: usize, msg: &str) {
    let offset = y * VGA_WIDTH + x;
    for (i, &byte) in msg.as_bytes().iter().enumerate() {
        unsafe {
            write_volatile(VGA_BUFFER_ADDR.offset((offset + i) as isize * 2), byte);
            write_volatile(VGA_BUFFER_ADDR.offset((offset + i) as isize * 2 + 1), 0x0f);
        }
    }
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
}

fn boot_error(msg: &str) -> ! {
    print_at(0, VGA_HEIGHT - 2, "Boot Error:");
    print_at(2, VGA_HEIGHT - 1, msg);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print_at(0, VGA_HEIGHT - 2, "KERNEL PANIC!");
    print_at(2, VGA_HEIGHT - 1, "System halted.");
    loop {}
}
