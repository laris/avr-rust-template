#![no_std]
#![no_main]
#![feature(llvm_asm)]

// disable #![warn(dead_code)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
#![allow(unused_assignments)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

// bare metal PORT define
const PINB:     *mut u8  = 0x23   as *mut u8;
const DDRB:     *mut u8  = 0x24   as *mut u8;
const PORTB:    *mut u8  = 0x25   as *mut u8;
const PINC:     *mut u8  = 0x26   as *mut u8;
const DDRC:     *mut u8  = 0x27   as *mut u8;
const PORTC:    *mut u8  = 0x28   as *mut u8;
const SRAM_u8:  *mut u8  = 0x0100 as *mut u8;   //  0x100
const SRAM_u16: *mut u16 = 0x0101 as *mut u16;  //  0x101-0x102
const SRAM_u32: *mut u32 = 0x0103 as *mut u32;  //  0x103-0x106
const SRAM_f32: *mut f32 = 0x0107 as *mut f32;  //  0x107-0x10a
const SRAM_f64: *mut f64 = 0x010a as *mut f64;  //  0x10b-0x110
const SRAM_bool_true:  *mut bool = 0x0111 as *mut bool; // 0x111
const SRAM_bool_false: *mut bool = 0x0112 as *mut bool; // 0x112
const SRAM_char: *mut char = 0x0113 as *mut char;       // 0x113-0x116
const SRAM_str:  *mut &str = 0x0117 as *mut &str;       // 0x117

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn main() {
    // hello nop
    nop();
    nop();
    nop();

    // set PORTB as input 
    set_ddrb_input();
    set_ddrc_output();
    nop();
    nop();
    nop();

    // if-else-if-else
    let read_value = read_pinb();
    if read_value == 0 {
        write_portc(0xff);
    } else if read_value == 1 {
        write_portc(0x55);
    } else {
        write_portc(0xaa);
    }
    nop();
    nop();
    nop();

    write_portc(if read_pinb() == 0 { 0x55 } else { 0xaa });
    nop();
    nop();
    nop();

    // while
    let mut read_value;
    while { read_value = read_pinb(); read_value } != 0 {
        write_portc(read_value + 1);
    }
    nop();
    nop();
    nop();

    // for
    for i in 0..5 {
        write_portc(i);
    }
    nop();
    // array in stack
    let u8_arr = [0x00, 0x55, 0xaa, 0xff];
    for i in u8_arr.iter() {
        write_portc(*i);
    }
    nop();
    nop();
    nop();

    // loop
//    let char_arr = ['a', 'b', 'c', 'd'];
//    let mut i = 0;
//    write_portc(char_arr[i].len_utf8() as u8);
//    nop();
 //   write_portc( {let mut byte =[0;1];  char_arr[i].encode_utf8(&mut byte); byte[0] as u8} );
 //   nop();

    /*
    This loop report err:
    error: linking with `avr-gcc` failed: exit code: 1
    src/main.rs:101: undefined reference to `abort'
    panicking.rs:69: undefined reference to `abort' 
    let location = loop {
        if char_arr[i] == 'c' {
            //write_portc(i as u8);
            break i;
        }
        //write_portc(char_arr[i].len_utf8() as u8);
        i += 1;
        //write_portc(i as u8);
    }; 
    */ 

    nop();
    nop();
    nop();
/*
    let char2_arr: [u8;4] = ['a' as u8, 'b' as u8, 'c' as u8, 'd' as u8];
    let mut i: usize = 0;
    let location2 = loop {
        //if char2_arr[i] as char == 'c' {
        //if i == 3 {
            //write_portc(i as u8);
            //break i;
        //}
        //println!("index is: {}, {}", i, char_arr2[i]);
        write_portc(i as u8);
        i += 1;
    };
    //write_portc(location2 as u8);
*/
    nop();
    nop();
    nop();
}

fn nop() {
    unsafe {
        llvm_asm!("NOP");
    }
}
// PORTB 
fn set_ddrb_input() {
    unsafe {
        core::ptr::write_volatile(DDRB, 0x00);
    }
}

fn set_ddrb_output() {
    unsafe {
        core::ptr::write_volatile(DDRB, 0xFF);
    }
}

fn read_pinb() -> u8 {
    unsafe {
        core::ptr::read_volatile(PINB)
    }
}

fn write_portb(byte: u8) {
    unsafe {
        core::ptr::write_volatile(PORTB, byte);
    }
}
//PORTC
fn set_ddrc_input() {
    unsafe {
        core::ptr::write_volatile(DDRC, 0x00);
    }
}

fn set_ddrc_output() {
    unsafe {
        core::ptr::write_volatile(DDRC, 0xFF);
    }
}

fn read_pinc() -> u8 {
    unsafe {
        core::ptr::read_volatile(PINC)
    }
}

fn write_portc(byte: u8) {
    unsafe {
        core::ptr::write_volatile(PORTC, byte);
    }
}
