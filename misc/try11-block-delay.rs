#![no_std]
#![no_main]
#![feature(llvm_asm)]

#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> ! { loop {} }

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe { llvm_asm!("NOP"); }
    call_dummy(); 
    unsafe { llvm_asm!("NOP"); }
    loop {}
}

#[inline]
fn call_dummy() {
    unsafe { llvm_asm!("NOP"); }
}