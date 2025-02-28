use core::panic::PanicInfo;

const VGA_BUFFER_ADDR: *mut u8 = 0xb8000 as *mut u8;
const BOOT_MSG: &[u8] = b"Booting InfinityX OS V2...";

pub fn init() {
    boot_msg();
}

fn boot_msg() {
    for (i, &byte) in BOOT_MSG.iter().enumerate() {
        unsafe {
            *VGA_BUFFER_ADDR.offset(i as isize * 2) = byte;
            *VGA_BUFFER_ADDR.offset(i as isize * 2 + 1) = 0x0f;
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
