#![no_std]
#![no_main]
#![feature(asm)]
#![feature(llvm_asm)]
#![feature(core_intrinsics)]
#![feature(abi_avr_interrupt)]

// bare metal PORTB define
const PINB:  *mut u8 = 0x23 as *mut u8;
const PINB5:  u8 = 0x5 ;// as *mut u8;
const DDRB:  *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;

#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> ! { loop {} }
//fn main() { }
#[no_mangle]
//pub extern "C" fn _start() -> ! { loop{} }
pub extern "C" fn main() { nop(); }

//#[no_mangle]
//pub extern "C" fn exit() { }

fn nop() {
    unsafe {
        llvm_asm!("NOP");
    }
}

#[no_mangle]
pub unsafe extern "avr-interrupt" fn __vector_11() {
    nop();
}

#[no_mangle]
pub unsafe extern "avr-interrupt" fn _ivr_timer1_compare_a() {
    let prev_value = core::intrinsics::volatile_load(PORTB);
    core::intrinsics::volatile_store(PORTB, prev_value ^ PINB5);
}
