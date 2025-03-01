#[allow(unused_imports)]
use core::fmt::{self, Write};
use core::ptr::write_volatile;
use core::sync::atomic::{AtomicUsize, Ordering};

const VGA_BUFFER_ADDR: *mut u8 = 0xb8000 as *mut u8;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

// Log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)]
pub enum LogLevel {
    DEBUG = 0x07,   // Light gray on black
    INFO = 0x0F,    // White on black
    WARN = 0x0E,    // Yellow on black
    ERROR = 0x04,   // Red on black
    CRITICAL = 0xC0, // Black on red
}

static CURSOR_POS_X: AtomicUsize = AtomicUsize::new(0);
static CURSOR_POS_Y: AtomicUsize = AtomicUsize::new(0);

pub struct Logger;

impl Logger {
    pub fn init(&self) {
        clear_screen();
    }

    pub fn log(&self, level: LogLevel, msg: &str) {
        let level_str = match level {
            LogLevel::DEBUG => "[DEBUG] ",
            LogLevel::INFO => "[INFO] ",
            LogLevel::WARN => "[WARN] ",
            LogLevel::ERROR => "[ERROR] ",
            LogLevel::CRITICAL => "[CRITICAL] ",
        };
        
        self.print(level_str, level);
        self.print(msg, level);
        self.print("\n", level);
    }
    
    fn print(&self, msg: &str, level: LogLevel) {
        for &byte in msg.as_bytes() {
            if byte == b'\n' {
                self.new_line();
            } else {
                self.print_char(byte, level as u8);
                CURSOR_POS_X.fetch_add(1, Ordering::SeqCst);
                
                if CURSOR_POS_X.load(Ordering::SeqCst) >= VGA_WIDTH {
                    self.new_line();
                }
            }
        }
    }
    
    fn print_char(&self, byte: u8, color: u8) {
        let x = CURSOR_POS_X.load(Ordering::SeqCst);
        let y = CURSOR_POS_Y.load(Ordering::SeqCst);
        let offset = y * VGA_WIDTH + x;
        
        unsafe {
            write_volatile(VGA_BUFFER_ADDR.offset(offset as isize * 2), byte);
            write_volatile(VGA_BUFFER_ADDR.offset(offset as isize * 2 + 1), color);
        }
        core::sync::atomic::fence(Ordering::SeqCst);
    }
    
    fn new_line(&self) {
        CURSOR_POS_X.store(0, Ordering::SeqCst);
        let mut y = CURSOR_POS_Y.load(Ordering::SeqCst);
        y += 1;
        
        // Scroll if needed
        if y >= VGA_HEIGHT {
            scroll_screen();
            y = VGA_HEIGHT - 1;
        }
        
        CURSOR_POS_Y.store(y, Ordering::SeqCst);
    }
    #[allow(dead_code)]
    pub fn print_at(&self, x: usize, y: usize, msg: &str, level: LogLevel) {
        let old_x = CURSOR_POS_X.load(Ordering::SeqCst);
        let old_y = CURSOR_POS_Y.load(Ordering::SeqCst);
        
        CURSOR_POS_X.store(x, Ordering::SeqCst);
        CURSOR_POS_Y.store(y, Ordering::SeqCst);
        
        self.print(msg, level);
        
        CURSOR_POS_X.store(old_x, Ordering::SeqCst);
        CURSOR_POS_Y.store(old_y, Ordering::SeqCst);
    }
}

// global access.
pub static LOGGER: Logger = Logger;

pub fn init() {
    LOGGER.init();
}

pub fn debug(msg: &str) {
    LOGGER.log(LogLevel::DEBUG, msg);
}

pub fn info(msg: &str) {
    LOGGER.log(LogLevel::INFO, msg);
}

pub fn warn(msg: &str) {
    LOGGER.log(LogLevel::WARN, msg);
}
#[allow(dead_code)]
pub fn error(msg: &str) {
    LOGGER.log(LogLevel::ERROR, msg);
}

pub fn critical(msg: &str) {
    LOGGER.log(LogLevel::CRITICAL, msg);
}

#[allow(dead_code)]
pub fn concat_str(s1: &str, s2: &str) -> [u8; 128] {
    let mut buffer = [0u8; 128];
    let mut index = 0;
    
    for &byte in s1.as_bytes() {
        if index < buffer.len() {
            buffer[index] = byte;
            index += 1;
        }
    }
    
    for &byte in s2.as_bytes() {
        if index < buffer.len() {
            buffer[index] = byte;
            index += 1;
        }
    }
    
    buffer
}

#[allow(dead_code)]
pub fn bool_to_str(b: bool) -> &'static str {
    if b { "true" } else { "false" }
}

pub fn clear_screen() {
    for i in 0..VGA_WIDTH * VGA_HEIGHT {
        unsafe {
            write_volatile(VGA_BUFFER_ADDR.offset(i as isize * 2), b' ');
            write_volatile(VGA_BUFFER_ADDR.offset(i as isize * 2 + 1), 0x07);
        }
    }
    CURSOR_POS_X.store(0, Ordering::SeqCst);
    CURSOR_POS_Y.store(0, Ordering::SeqCst);
}

fn scroll_screen() {
    for y in 1..VGA_HEIGHT {
        for x in 0..VGA_WIDTH {
            let src_offset = y * VGA_WIDTH + x;
            let dst_offset = (y - 1) * VGA_WIDTH + x;
            
            unsafe {
                let char_byte = *VGA_BUFFER_ADDR.offset(src_offset as isize * 2);
                let color_byte = *VGA_BUFFER_ADDR.offset(src_offset as isize * 2 + 1);
                
                write_volatile(VGA_BUFFER_ADDR.offset(dst_offset as isize * 2), char_byte);
                write_volatile(VGA_BUFFER_ADDR.offset(dst_offset as isize * 2 + 1), color_byte);
            }
        }
    }
    

    let last_line = VGA_HEIGHT - 1;
    for x in 0..VGA_WIDTH {
        let offset = last_line * VGA_WIDTH + x;
        unsafe {
            write_volatile(VGA_BUFFER_ADDR.offset(offset as isize * 2), b' ');
            write_volatile(VGA_BUFFER_ADDR.offset(offset as isize * 2 + 1), 0x07);
        }
    }
}

pub struct Writer;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        LOGGER.print(s, LogLevel::INFO);
        Ok(())
    }
}

#[macro_export]
macro_rules! log_debug {
    ($msg:expr) => {
        $crate::logging::debug($msg);
    };
}

#[macro_export]
macro_rules! log_info {
    ($msg:expr) => {
        $crate::logging::info($msg);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($msg:expr) => {
        $crate::logging::warn($msg);
    };
}

#[macro_export]
macro_rules! log_error {
    ($msg:expr) => {
        $crate::logging::error($msg);
    };
}

#[macro_export]
macro_rules! log_critical {
    ($msg:expr) => {
        $crate::logging::critical($msg);
    };
}
