#![no_std]

#[path="./print.rs"]
#[macro_use]
mod print;
use core::panic;

const PL011:usize = 0x09000000;

const UART_DR:usize = 0x0;
const UART_FR:usize = 0x018;

const CANCEL:u8 = 94;

const MAX_ELEMENTS:usize = 10;

/*
pub fn read() -> i32 {

}

fn scan(l: &[u8]){
    let l:&[u8] = &read();
    //let _l: &[u8] = _read(l);
}

fn _scan(l: &[u8]) -> &[u8] {
    let i:&[u8] = &read();
    let l_:&[u8] = [l,i];
    let 
    l_
}*/

fn add_to_slice(slice: &[u32], value: u32) -> [u32; MAX_ELEMENTS] {
    let len = slice.len();

    let mut new_array = [0u32; MAX_ELEMENTS];
    
    for i in 0..len {
        new_array[i] = slice[i];
    }
    
    new_array[len] = value;
    
    new_array
}

pub fn read_byte() -> u8 {
    let mut i:u8 = 0;
    loop {
        if (is_read_fifo_full() == true){
            println!("ERROR: READ_FIFO_IS_FILL");
            panic!();
        }
        if(read_fifo() == true){
            i = unsafe{ core::ptr::read_volatile((PL011 + UART_DR) as *const u8)};
            if(i != 13 && ( i < 48 || 57 < i ) ){
                continue;
            }
            print!("{}",i);
            break;
        }
    }
    i
}

pub fn _read_byte() -> u8 {
    let mut i:u8 = 0;
    loop {
        if (is_read_fifo_full() == true){
            println!("ERROR: READ_FIFO_IS_FILL");
            panic!();
        }
        if(read_fifo() == true){
            i = unsafe{ core::ptr::read_volatile((PL011 + UART_DR) as *const u8)};
            if(i != 45 && i != CANCEL && ( i < 48 || 57 < i ) ){
                continue;
            }
            print!("{}",i);
            break;
        }
    }
    i
}

fn read_fifo() -> bool{
    (unsafe{ core::ptr::read_volatile((PL011 + UART_FR) as *const u16 )} & (1 << 4)) == 0
}

fn is_read_fifo_full() -> bool{ //true : ERROR
    (unsafe { core::ptr::read_volatile((PL011 + UART_FR) as *const u16 )} & (1 << 6)) == 1
}