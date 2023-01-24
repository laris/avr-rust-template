#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
use core::arch::asm;

#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> ! { loop {} }

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe { asm!("NOP"); }
    loop {}
}
