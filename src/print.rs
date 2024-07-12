#![no_std]

use core::fmt::{self, Write};
use core::result::Result::Ok;

const PL011:usize = 0x09000000;
const UART_DR:usize = 0x0;
const UART_FR:usize = 0x018;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn _print(args: fmt::Arguments) {
    let mut writer = UartWriter {};
    writer.write_fmt(args).unwrap();
}

struct UartWriter;

impl Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            putc(c);
        }
        Ok(())
    }
}

pub fn putc(c: u8) {
    loop{
        if is_write_fifo_full() == false {
            write_char(c);
            break;
        }
    }
}
fn write_char(c: u8) {
    unsafe { core::ptr::write_volatile((PL011 + UART_DR) as *mut u8, c) };
}

fn is_write_fifo_full() -> bool {
    (unsafe { core::ptr::read_volatile((PL011 + UART_FR) as *const u16) } & (1 << 5)) != 0
}