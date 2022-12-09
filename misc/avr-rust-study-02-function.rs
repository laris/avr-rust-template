#![no_std]
#![no_main]
#![feature(llvm_asm)]

// disable #![warn(dead_code)]
#![allow(dead_code)]
#![allow(unused_unsafe)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
#![allow(unused_assignments)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

// bare metal PORTB define
const PINB:  *mut u8 = 0x23 as *mut u8;
const DDRB:  *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;
const SRAM_u8: *mut u8 = 0x0100 as *mut u8;     //  0x100
const SRAM_u16: *mut u16 = 0x0101 as *mut u16;  //  0x101-0x102
const SRAM_u32: *mut u32 = 0x0103 as *mut u32;  //  0x103-0x106
const SRAM_f32: *mut f32 = 0x0107 as *mut f32;  //  0x107-0x10a
const SRAM_f64: *mut f64 = 0x010a as *mut f64;  //  0x10b-0x110
const SRAM_bool_true:  *mut bool = 0x0111 as *mut bool; // 0x111
const SRAM_bool_false: *mut bool = 0x0112 as *mut bool; // 0x112
const SRAM_char: *mut char = 0x0113 as *mut char;       // 0x113-0x116
const SRAM_str: *mut &str = 0x0117 as *mut &str;       // 0x117

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

    // set PORTB as output
    set_ddrb_output();
    nop();
    nop();
    nop();

    another_fun1();
    nop();
    nop();
    nop();

    another_fun2(0xaa, 0x55);
    nop();
    nop();
    nop();

    set_ddrb_input();
    let input_1 = read_pinb();
    let input_2 = read_pinb();
    set_ddrb_output();
    nop();
    unsafe { core::ptr::write(SRAM_u8, another_fun3(input_1, input_2)); }
    nop();
    nop();


}

fn another_fun1() {
    write_portb(0x55);
}

fn another_fun2(x: u8, y: u8) {
    write_portb(x);
    write_portb(y);
}

fn another_fun3(x: u8, y: u8) -> u8 {
    x + y
    //return x + y;
}

fn nop() {
    unsafe {
        llvm_asm!("NOP");
    }
}

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