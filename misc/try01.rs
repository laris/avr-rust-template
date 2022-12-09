#![no_std]
#![cfg_attr(not(test), no_main)] // #![no_main] interfers with 'cargo test' when targeting the host machine.
#![feature(llvm_asm)]

extern crate avr_std_stub;
const NUM_1: u8 = 1;
static NUM_2: u8 = 2;

#[no_mangle]
#[cfg(not(test))] // The main function interfers with 'cargo test' when targeting the host machine.
fn main() {
	let a = NUM_1;
	let b = NUM_2;
    let c = a + b;

    unsafe {
        llvm_asm!("NOP\nLPM");
        llvm_asm!(
            "PUSH $0"
            :
            : "r"(c)
            :
        );
        llvm_asm!("NOP\nLPM");
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_foo() {
        assert_eq!(1, 1);
    }
}
