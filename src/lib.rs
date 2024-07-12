#![no_std]

#[path = "./print.rs"]
#[macro_use]
pub mod print;

use core::{panic, u8};

use print::putc;
use core::unreachable;
use core::format_args;
use core::concat;

const PL011: usize = 0x09000000;

const UART_DR: usize = 0x0;
const UART_FR: usize = 0x018;

const CANCEL: u8 = 94;
const FINISH: u8 = 13;
const MINUS: u8 = 45;

const MAX_ELEMENTS: usize = 10;

pub fn read() -> i32 {
    let return_math = scan();
    println!("{:?}",return_math);
    let j = return_math[0];
    if j == MINUS {
        let i = _read(&return_math, 0, 1);
        return -i;
    }else if j >= 48 && j <= 57 {
        let i = _read(&return_math, (j - 48) as i32, 1);
        return i;
    } else {
        println!("unreachable but reach in fn read()");
        unreachable!();
    }
}

fn _read(return_math: &[u8;MAX_ELEMENTS],i: i32,u:usize) -> i32 {
    let j = return_math[u];
    if j == 0 {
        i
    } else if j >= 48 && j <= 57 {
        _read(&return_math,(i * 10 + ((j - 48) as i32)), u + 1)
    } else {
        println!("unreachhable but reach in fn _read()");
        unreachable!();
    }
}

fn scan() -> [u8; MAX_ELEMENTS] {
    let math: [u8; MAX_ELEMENTS] = [0u8; MAX_ELEMENTS];
    let _math = add_to_slice(&math, read_byte(), 0);
    let return_math = _scan(1, &_math);
    return_math
}

fn _scan(i: usize, math: &[u8; MAX_ELEMENTS]) -> [u8; MAX_ELEMENTS] {
    let read = _read_byte();
    if read == FINISH {
        *math
    } else if read == CANCEL {
        println!("Retry");
        let return_math = scan();
        return_math
    } else {
        let _math = add_to_slice(&math, read, i);
        let return_math;
        if i == MAX_ELEMENTS {
            println!("ERROR: TOO_MANY_ARGUMENTS");
            print!("Retry :");
            return_math = scan();
        } else {
            return_math = _scan(i + 1, &_math);
        }
        return_math
    }
}

fn add_to_slice(slice: &[u8; MAX_ELEMENTS], value: u8, _i: usize) -> [u8; MAX_ELEMENTS] {
    let mut new_array = [0u8; MAX_ELEMENTS];

    for i in 0..MAX_ELEMENTS {
        new_array[i] = slice[i];
    }
    new_array[_i] = value;
    new_array
}

fn index(i: usize) -> i32 {
    if i == 0 {
        1i32
    } else {
        10 * index(i - 1)
    }
}

fn read_byte() -> u8 {
    let mut i: u8 = 0;
    loop {
        if is_read_fifo_full() == true {
            println!("ERROR: READ_FIFO_IS_FILL");
            panic!();
        }
        if read_fifo() == true {
            i = unsafe { core::ptr::read_volatile((PL011 + UART_DR) as *const u8) };
            if i != MINUS && (i < 48 || 57 < i) {
                continue;
            }
            putc(i);
            break;
        }
    }
    i
}

fn _read_byte() -> u8 {
    let mut i: u8 = 0;
    loop {
        if is_read_fifo_full() == true {
            println!("ERROR: READ_FIFO_IS_FILL");
            panic!();
        }
        if read_fifo() == true {
            i = unsafe { core::ptr::read_volatile((PL011 + UART_DR) as *const u8) };
            if i != FINISH && i != CANCEL && (i < 48 || 57 < i) {
                continue;
            }
            putc(i);
            break;
        }
    }
    i
}

fn read_fifo() -> bool {
    (unsafe { core::ptr::read_volatile((PL011 + UART_FR) as *const u16) } & (1 << 4)) == 0
}

fn is_read_fifo_full() -> bool {
    //true : ERROR
    (unsafe { core::ptr::read_volatile((PL011 + UART_FR) as *const u16) } & (1 << 6)) == 1
}
