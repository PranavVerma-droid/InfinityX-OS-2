#![no_std]
#![no_main]

use core::panic::PanicInfo;

// panic call
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // vpa buff print
    let vga_buffer = 0xb8000 as *mut u8;
    let print = b"Booting InfinityX OS V2...";
    
    for (i, &byte) in print.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            // white color on black background lol
            *vga_buffer.offset(i as isize * 2 + 1) = 0x0f; 
        }
    }

    loop {}
}
