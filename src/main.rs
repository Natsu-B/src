#![no_std]
#![no_main]

#[macro_use]
mod print;

use core::{arch::asm,panic::PanicInfo};

#[no_mangle]
fn efi_main() -> ! {
    println!("Hello");
    loop {unsafe{asm!("wfi");}}
}

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    println!("ERROR");
    loop{unsafe{asm!("wfi");}}
}