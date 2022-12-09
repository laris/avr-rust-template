#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(associated_type_defaults)]

mod asm;
use crate::asm::asm::*;
//mod bit;
//use crate::bit::bit::*;
//use bobbin_bits::*;
//use ux::*;

// m328p
const PINB:   *mut u8 = 0x23 as *mut u8;
const DDRB:   *mut u8 = 0x24 as *mut u8;
const PORTB:  *mut u8 = 0x25 as *mut u8;
const SREG:   *mut u8 = 0x5F as *mut u8;
const TCNT1L: *mut u8 = 0x84 as *mut u8;
const TCNT1H: *mut u8 = 0x84 as *mut u8;
const SRAM_START: *mut u8 = 0x100 as *mut u8;
const SREG_T: u8 = 6;


#[no_mangle]
fn main() {
/*
  __nop();
  bit_copy(PINB, 0, PORTB, 1);
  bit_copy(SREG, 1, PORTB, 2);
  bit_copy(TCNT1L, 2, TCNT1H, 3);
  bit_copy(SRAM_START, 3, PORTB, 4);
  __nop();

  __nop();
  bit_set(DDRB, 0);
  bit_set(SREG, 1);
  bit_set(TCNT1H, 2);
  bit_set(SRAM_START, 7);
  __nop();

  __nop();
  bit_clr(DDRB, 0);
  bit_clr(SREG, 1);
  bit_clr(TCNT1H, 2);
  bit_clr(SRAM_START, 7);
  __nop();

  __nop();
  let bit_value: u8 = bit_read_raw(DDRB, 0);
  if bit_value == 0 { bit_set(SREG, SREG_T); } 
  else { bit_clr(SREG, SREG_T); }
  __nop();

  __nop();
  bit_write(PORTB, 0, true);
  bit_write(SREG, 0, false);
  bit_write(TCNT1L, 0, false);
  bit_write(SRAM_START, 0, true);
  __nop();

  __nop();
  bit_set2(DDRB,        u3::new(0));
  __nop();
  bit_set2(SREG,        u3::new(1));
  __nop();
  bit_set2(TCNT1H,      u3::new(2));
  __nop();
  bit_set2(SRAM_START,  u3::new(7));
  __nop();
  bit_set2(SRAM_START,  u3::new(0xff));
  __nop();
*/
/*

pub struct DDRB_BIT0;
impl RegBit for DDRB_BIT0 {
      const REG_ADDR: *mut u8 = DDRB;
      const BIT_IDX:  *mut u3 = 0 as *mut u3;
}

pub struct TCNT1L_BIT0;
impl RegBit for TCNT1L_BIT0 {
      const REG_ADDR: *mut u8 = TCNT1L;
      const BIT_IDX:  *mut u3 = 0 as *mut u3;
}

pub trait RegBit: Sized {
    //type  RegAddr = u8;
    //type  BitIndex: u3;
    //type  BitValue: u1;
    const REG_ADDR: *mut u8;
    const BIT_IDX:  *mut u3;
  
    #[inline(always)]
    fn bit_set() {
      unsafe {
        if Self::REG_ADDR as u8 >= 0x20 && Self::REG_ADDR as u8 <= 0x3F {
          llvm_asm!("SBI $0, $1" 
                  ::"I"(Self::REG_ADDR), "I"(Self::BIT_IDX) :: "volatile");
        } else if Self::REG_ADDR as u8 == 0x5F {
          llvm_asm!("BSET $0" ::"I"(Self::BIT_IDX) :: "volatile");
        } else {  
          core::ptr::write_volatile(Self::REG_ADDR,
          core::ptr::read_volatile( Self::REG_ADDR) | 1 << Self::BIT_IDX as u8);
        }
      }
    }
//    fn bit_clr();
//    fn bit_write();
//    fn bit_read();
}
  __nop();
  //DDRB_BIT0::bit_set();
  __nop();
  //TCNT1L_BIT0::bit_set();
  __nop();
*/
/*
  __nop();
  bit_set2(DDRB,        0);
  __nop();
  bit_set2(SREG,        1);
  __nop();
  bit_set2(TCNT1H,      2);
  __nop();
  bit_set2(SRAM_START,  3);
  __nop();
  bit_set2(SRAM_START,  unsafe { core::ptr::read_volatile(PINB) } );
  __nop();
  let u1_val: U3 = U3::from(7_u8);
  unsafe { core::ptr::write_volatile(PORTB, u1_val.into()) };
  let u1_val: U3 = U3::from(8_u8);
  unsafe { core::ptr::write_volatile(PORTB, u1_val.into()) };
*/
  __nop();
  __nop();


//  let ddrb = 0x24_u8 as *mut UnsafeCell<u8>;
//  unsafe { ptr::write_volatile(PORTB,  (*ddrb)::into_inner() ); };
//  let _ = Register { name: "SREG", addr: UnsafeCell::<u8>::from(0x5f) };


/******************************************************************************/
/* re-test for register model
 */
use core::cell::UnsafeCell;
use core::ptr;
//use bobbin_bits::*;
use ux::*;
/*
pub trait Field {
    type A; // base address
    type N; // offset num
    type T; // value in cell
    fn new(addr: A, offset: N); 
}

pub struct SREG_BIT_T {};

impl Field for SREG_BIT_T {
      fn new(addr: u6, offset: u3, val: u1) -> SREG_BIT_T {
            SREG_BIT_T {
              value: UnsafeCell::new(val),
              base: addr,
              off: offset,
            }
      }
}

pub struct RegisterField<T, N> 
    where T: u1,
          N: u3 
{
    value:  UnsafeCell<T>,
    offset: N,
}
*/

/*
pub struct Bit1 {
    value:  UnsafeCell<ux::u1>,
    offset: ux::u3,
    addr:   ux::u6,
}

pub struct Reg1Byte {
    value: UnsafeCell<u8>,
    addr:  ux::u6,
}

let sreg_t = Bit1 { value:  UnsafeCell::new(u1::new(0)), 
                   offset: u3::new(6) , addr: u6::new(0x5f)};

pub struct VCell<T> {
    value: T,
}

impl VCell<T> {
     pub fn new(addr: u8) {
          pub const ADDR = addr as *const _;
     }
}

//let sreg = VCell { value: 0u8 };
let sreg = VCell::<u8>::new(0x5f);
*/

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct VolatileCell<T> {
    value: T,
}

impl<T> VolatileCell<T> {
    pub const fn new(value: T) -> Self {
        VolatileCell { value }
    }

    #[inline]
    pub fn get(&self) -> T {
        unsafe { ptr::read_volatile(&self.value) }
    }

    #[inline]
    pub fn set(&self, value: T) {
        unsafe { ptr::write_volatile(&self.value as *const T as *mut T, value) }
    }

    #[inline]
    pub fn write(&self, value: T) {
      unsafe { }
    }
}
use core::ops::Deref;
impl<T> Deref for VolatileCell<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &T {
      &self.value
    }
}

use core::ptr::*;
use core::mem::*;
let vc1 = VolatileCell { value: 0x1 };
let ref_vc2 = unsafe { (0x5f as *const VolatileCell<u8>) } ;

//*ref_vc2.deref();
//unsafe { write_volatile(PORTB,size_of::<VolatileCell<u8>>() as u8 ) };


//unsafe { write_volatile(PORTB, *ref_vc2) };
unsafe { write_volatile(PORTB, **ref_vc2 )};


/******************************************************************************/


// end of main
}

/*
pub struct Register {
    name: &'static str,
    addr: UnsafeCell<u8>,
    //addr: UnsafeCell::new(0x5f: u8),
}
*/

/*
impl<T, R> Register<T, R> where T: u8, R: Register {
    pub 
}

pub trait Register {
    pub const fn new(addr) -> T {

    }
}
*/


#[cfg(test)]
mod test {
    #[test]
    fn test_foo() {
        assert_eq!(1, 1);
    }
}


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}