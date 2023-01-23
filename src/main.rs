#![no_std]
#![no_main]

#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> ! { loop {} }

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}
