#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(core_intrinsics)]

// disable #![warn(dead_code)]
#![allow(dead_code)]
#![allow(unused_unsafe)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
#![allow(unused_assignments)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

//#![space(progmem)]
//pub static FOOBAR: &'static str = "hello world";
#[link_section = ".progmem.data"]
//static P_BYTE: u8 = 0b0000_0000; //b'A';
static P_BYTE: u8 = 0b0000_0000; //b'A';

// bare metal PORTB define
const SREG:  *mut u8 = 0x5f as *mut u8;
const PINB:  *mut u8 = 0x23 as *mut u8;
const DDRB:  *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;
const PINC:  *mut u8 = 0x26 as *mut u8;
const DDRC:  *mut u8 = 0x27 as *mut u8;
const PORTC: *mut u8 = 0x28 as *mut u8;
const SRAM_u8: *mut u8 = 0x0100 as *mut u8;     //  0x100
const SRAM_u16: *mut u16 = 0x0101 as *mut u16;  //  0x101-0x102
const SRAM_u32: *mut u32 = 0x0103 as *mut u32;  //  0x103-0x106
const SRAM_f32: *mut f32 = 0x0107 as *mut f32;  //  0x107-0x10a
const SRAM_f64: *mut f64 = 0x010a as *mut f64;  //  0x10b-0x110
const SRAM_bool_true:  *mut bool = 0x0111 as *mut bool; // 0x111
const SRAM_bool_false: *mut bool = 0x0112 as *mut bool; // 0x112
const SRAM_char: *mut char = 0x0113 as *mut char;       // 0x113-0x116
const SRAM_str: *mut &str = 0x0117 as *mut &str;       // 0x117-0x1a
const SRAM_i16: *mut i16 = 0x011b as *mut i16;  //  0x11b-0x11c

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn main() {
    // set PORTB as input 
    set_ddrb_input();
    // set PORTC as output
    set_ddrc_output();

    nop();
    nop();
    nop();

    // sbi io5, b
    unsafe {core::ptr::write_volatile(DDRB, *DDRB | 0b0000_0001); }
    nop();
    // cbi io5, b
    unsafe {core::ptr::write_volatile(DDRB, *DDRB & 0b1111_1110); }
    nop();
    // sbr = ori rdh, 1<<b
    let value: u8 = read_pinb();
    unsafe {core::ptr::write_volatile(DDRB, value | 0b0000_0001); }
    nop();
    // cbr = andi rdh, 0xff-1<<b
    let value: u8 = read_pinb();
    unsafe {core::ptr::write_volatile(DDRB, value & 0b1111_1110); }
    nop();
    // bst/bld
    // https://aykevl.nl/2021/02/avr-bitshift
    let value = read_pinb();
    unsafe { core::ptr::write(SRAM_i16, (value as i16) << 15 ); }
    nop();
    // my try, not work
    let value1 = read_pinb();
    let value2 = read_pinb();
    unsafe {core::ptr::write_volatile(DDRB, value1 & 1 | value2); }
    nop();

    // swap
    let value = read_pinb();
    unsafe {core::ptr::write_volatile(DDRB, value << 4); }
    nop();

    // SREG set/clr bset/bclr s
    unsafe {core::ptr::write_volatile(SREG, 0b0000_0000); }
    unsafe {core::intrinsics::volatile_store(SREG, 1 << 4); }
    nop();

    // LDS
    unsafe {core::ptr::write_volatile(PORTB, 
            core::ptr::read(SRAM_u8)
    );
    }
    nop();


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