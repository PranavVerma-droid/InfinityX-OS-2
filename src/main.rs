#![no_std]
#![no_main]

mod bootloader;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    bootloader::init();
    loop {}
}
