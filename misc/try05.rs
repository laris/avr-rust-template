#![no_std]
#![no_main]
//#![cfg_attr(not(test), no_main)] // #![no_main] interfers with 'cargo test' when targeting the host machine.
//#[cfg(not(test))] // The main function interfers with 'cargo test' when targeting the host machine.
#![allow(unused_imports)]
#![allow(unused_attributes)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_unsafe)]
#![feature(core_intrinsics)]
#![feature(llvm_asm)]

//extern crate avr_std_stub;
//extern crate try4;
mod constdefine;
use crate::constdefine::*;
mod asm;
use crate::asm::*;
mod register;
use crate::register::*;
//mod bit;
//use crate::bit::bit::*;
use ux;
mod mycore;
mod mcu;
mod asmtest;
mod mmodel;
use mmodel::*;

use asmtest::*;

use core::intrinsics::{volatile_load, volatile_store};
use core::ptr::{read_volatile, write_volatile};



#[no_mangle]
fn main() {
    __nop();
    //bit_test(); // asmtest.rs
    //delay_test();
    //prom_test(); // asmtest.rs
    /*
      pub static BAR: &'static [u8;2] = &[0xff, 0x00];
      let a = BAR[1];
      unsafe { volatile_store(PORTB, a); }

      __nop();
      __nop();
      pub static FOO: &'static str = "ABC";
      let b = FOO.bytes().nth(0).unwrap();
      unsafe { volatile_store(DDRB, b); }

      __nop();
      __nop();
      __nop();

    //#[inline(always)]
      static C: u8 = 1;
      static D: u8 = 1;
      unsafe { volatile_store(DDRB, C+D); }
    */
    __nop();
    __nop();
    __nop();

    //  __bset(SREG_T);
    //  __bclr(SREG_T);
    //  __sec();
    //  __clc();

    __nop();
    __nop();
    __nop();

    //  let value_pinb = __in(PINB);
    //  __out(PORTB, value_pinb);

    __nop();
    __nop();
    __nop();

    //  __out(PINB, 0xFF);

    __nop();
    __nop();
    __nop();

    //  __sbi(PORTB, PORTB0);
    //  __cbi(PORTB, PORTB7);

    __nop();
    __nop();
    __nop();
    /*
      let value_sbr = 0x00;
      let value_sbr_ed = __sbr(value_sbr, BIT_MSK_7);
      __out(PORTB, value_sbr_ed);
      __nop();
      let value_cbr = 0xFF;
      let value_cbr_ed = __cbr(value_cbr, BIT_MSK_7);
      __out(PORTB, value_cbr_ed);

      __nop();
      __nop();
      __nop();
      let value_bst = 0x0000_0001;  // init data
      __bst(value_bst, BIT_IDX_0);  // copy data bit to T
      __nop();
      let value_bld = 0xFE;         // init data
      __bld(value_bld, BIT_IDX_7);  // copy T into data
      __out(PORTB, value_bld);      // output
    */
    __nop();
    __nop();
    __nop();

    //crate::mycore::mem::core_mem_discriminant();
    //crate::mycore::intrinsics::core_intrinsics_copy_and_copy_nonoverlapping();
    //crate::mycore::intrinsics::core_intrinsics_transmute();
    //crate::mycore::intrinsics::core_intrinsics_write_bytes();
    /*
       let pin_val = __in(PINB);
       drop(pin_val);              // not work
       core::mem::forget(pin_val); // not work
       __out(PORTB, pin_val);
    */

    unsafe {
        let mut val = core::ptr::read_volatile(PINB);
        //val |= 0x10;
        //val |= 0x0F;
        if val & 0x01 == 1u8 {
            core::ptr::write_volatile(PORTB, val);
        }
    };
}

/*******************************************/

/*
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
*/

/*
#[cfg(test)]
mod test {
    #[test]
    fn test_foo() {
        assert_eq!(1, 1);
    }
}
*/

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
