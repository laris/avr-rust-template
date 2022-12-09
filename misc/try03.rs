#![no_std]
#![cfg_attr(not(test), no_main)] // #![no_main] interfers with 'cargo test' when targeting the host machine.
#![feature(associated_type_defaults)]
#![feature(const_fn)]

extern crate avr_std_stub;

mod register;

use crate::register::{RegisterBits, Register};

const DDRB:  *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;

#[no_mangle]
#[cfg(not(test))] // The main function interfers with 'cargo test' when targeting the host machine.
fn main() {
  let a: u8 = 0xfe;
  core::mem::drop(a);
  let b: u8 = 0x01;
//  let c = a - b;
//  let d: u8 = 1;
//  d<<1;
	unsafe {
			core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) | 0b00001111);
			core::ptr::write_volatile(PORTB, 0b00001010);
	}
}

#[cfg(test)]
mod test {
    #[test]
    fn test_foo() {
        assert_eq!(1, 1);
    }
}
