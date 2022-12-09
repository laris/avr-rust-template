#![no_std]
#![no_main]
#![feature(llvm_asm)]
#![feature(abi_avr_interrupt)]
#![feature(core_intrinsics)]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "avr-interrupt" fn __vector_11() {
    // ...
    nop();
    //core::intrinsics::volatile_store(0x24 as *mut u8, 0b_0010_0000);
}
//#[no_mangle]
//fn main() { }
#[no_mangle]
fn main() {
    // hello nop
    nop();
    nop();
    unsafe { core::intrinsics::volatile_store(0x24 as *mut u8, 0b_0010_0000); }
    nop();
    nop();
}

fn nop() {
    unsafe {
        llvm_asm!("NOP");
    }
}

#[no_mangle]
pub extern "C" fn exit() { }