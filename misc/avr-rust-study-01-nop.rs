#![no_std]
#![no_main]
#![feature(llvm_asm)]

#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> ! { loop {} }


//const SRAM_u8: *mut u8 = 0x0100 as *mut u8;     //  0x100

/*
//use avr_progmem::progmem;
//use avr_progmem::read_byte;
#[link_section = ".progmem.data"]
static P_BYTE: u8 = b'A';

progmem!{
    /// Static string stored in progmem!
    pub static progmem WORDS: [u8; 4] = *b"abcd";
}

let data: [u8; 4] = WORDS.load();
assert_eq!(b"abcd", &data);
 */

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe { 
        llvm_asm!("NOP");
        llvm_asm!("NOP");
        /*
        let data: u8 = unsafe { read_byte(&P_BYTE) };
        //assert_eq!(b'A', data);
        core::ptr::write(SRAM_u8, read_byte(&P_BYTE));
        llvm_asm!("NOP");
        llvm_asm!("NOP");

        llvm_asm!("NOP");
        progmem!{
        /// Static string stored in progmem!
        pub static progmem WORDS: [u8; 4] = *b"abcd";
        }
        let data: [u8; 4] = WORDS.load();
        */
        llvm_asm!("NOP");
    }

    loop {}
}
