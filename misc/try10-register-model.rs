#![no_std]
#![no_main]
#![feature(llvm_asm)]

#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> ! { loop {} }

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe { 
        llvm_asm!("NOP");
        llvm_asm!("NOP");
        llvm_asm!("NOP");
    }
// scratch start
    const PINB:  *mut u8 = 0x23 as *mut u8;
    const DDRB:  *mut u8 = 0x24 as *mut u8;
    const PORTB: *mut u8 = 0x25 as *mut u8;
    unsafe {core::ptr::write_volatile(DDRB, *DDRB | 0b0000_0001); }
    unsafe { llvm_asm!("NOP"); }

// scratch end
    loop {}
}
