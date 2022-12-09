#![no_std]
#![no_main]
#![feature(llvm_asm)]
//#![cfg_attr(not(test), no_main)] 
//#![no_main] interfers with 'cargo test' when targeting the host machine.

//extern crate avr_std_stub;

// bare metal PORTB define
const PINB:  *mut u8 = 0x23 as *mut u8;
const DDRB:  *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;

#[no_mangle]
#[cfg(not(test))] // The main function interfers with 'cargo test' when targeting the host machine.

fn main() {
    nop();
    nop();
    nop();
    // set ddrb 
    set_ddrb_output();
    nop();
    nop();
    // write portb 
    let a = 0u8;
    write_portb(a);
    nop();
    nop();
    //
    let b = a + 1;
    write_portb(b);
    nop();
    nop();

    // if
    if a == 0 {
        write_portb(b);
    }
    nop();
    nop();

    // if-else
    if a == 0 {
        write_portb(a);
    } else {
        write_portb(b);
    }
    nop();
    nop();

    // if-else
    set_ddrb_input();
    nop();
    if read_pinb() == 0 {
        set_ddrb_output();
        write_portb(a);
    } else {
        set_ddrb_output();
        write_portb(b);
    }
    nop();
    nop();


    // loop -> rjmp .-2
    loop{};
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
