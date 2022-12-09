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

    // primitive types
    let int_u8:     u8      = 0xff;
    let int_u16:    u16     = 0xffff;
    let int_u32:    u32     = 0xffff_ffff;
    let int_u64:    u64     = 0xffff_ffff_ffff_ffff;
    let int_u128:   u128    = 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff;
    let int_usize:  usize   = 0xff;

    write_portb(int_u8.count_ones() as u8);
    nop();
    write_portb(int_u16.count_ones() as u8);
    nop();
    write_portb(int_u32.count_ones() as u8);
    nop();
    write_portb(int_u64.count_ones() as u8);
    nop();
    write_portb(int_u128.count_ones() as u8);
    nop();
    write_portb(int_usize.count_ones() as u8);

    nop();
    nop();
    nop();

    unsafe { core::ptr::write(SRAM_u8, 0xff); }
    nop();
    unsafe { core::ptr::write(SRAM_u8,  int_u8); }
    nop();
    unsafe { core::ptr::write(SRAM_u16, int_u16); }
    nop();
    unsafe { core::ptr::write(SRAM_u32, int_u32); }
    nop();
    unsafe { core::ptr::write(SRAM_u32, int_u32.count_ones()); }
    
    nop();
    nop();
    nop();

    // float
    let fp_pi_32: f32 = 3.14;
    unsafe { core::ptr::write(SRAM_f32, fp_pi_32); }
    nop();
    let fp_pi_64: f64 = 3.14;
    unsafe { core::ptr::write(SRAM_f64, fp_pi_64); }

    nop();
    nop();
    nop();

    // bool
    let bool_true: bool = true;
    unsafe { core::ptr::write(SRAM_bool_true, bool_true); }
    nop();
    let bool_false: bool = false;
    unsafe { core::ptr::write(SRAM_bool_false, bool_false); }
    nop();
    write_portb(core::mem::size_of::<u8>() as u8);

    nop();
    nop();
    nop();

    // char
    let char_a: char = 'a';
    unsafe { core::ptr::write(SRAM_char, char_a); }
    nop();
    nop();
    nop();

    // str
    let str_a: &'static str = "abc";
    //let str_a = "a";
    write_portb(core::mem::size_of_val(str_a) as u8);   // 1
    nop();
    write_portb(str_a.as_bytes().len() as u8);          // 1
    nop();
    write_portb(str_a.as_bytes()[0]);
    write_portb(str_a.as_bytes()[1]);
    write_portb(str_a.as_bytes()[2]);
    //write_portb(str_a.as_bytes()[3]);
    nop();
    unsafe { core::ptr::write(SRAM_str, str_a); }

    nop();
    nop();
    nop();

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