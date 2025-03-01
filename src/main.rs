#![no_std]
#![no_main]
#![feature(format_args_nl)]

mod bootloader;
mod logging;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    logging::init();
    
    bootloader::init();
}
