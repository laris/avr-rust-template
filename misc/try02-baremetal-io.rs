#![no_std]
#![no_main]


/// ## External resources
/// * [Tutorial - Learn the Basics of I/O Pins for an AVR Microcontroller (examples in C)](https://maker.pro/custom/tutorial/learn-the-basics-of-io-pins-for-an-avr-microcontroller)
/// 
/// ## The problem space of IO on AVR
/// 
///   * Reading from or writing to an IO pin is as easy as reading a byte from or writing a byte to the correct location in memory
///   * The main issue is identifying which memory locations assigned to which physical pins and peripherals for each chip
/// 
/// Rather than using bare-metal IO tied to specific AVR chips, it is recommended to use an abstraction layer over the actual I/O
/// register manipulation.
/// 
/// ## A bare-metal example of IO
/// 
/// This example uses hardcoded IO register locations for the ATMega328p. It may work for
/// some other AVR devices in the same family, but others will use different IO register
/// memory mappings and so will not produce the expected output when ran on these devices.
/// 
/// **NOTE**: Executables should prefer using an abstraction layer over the actual I/O such as `embedded-hal`
/// rather than writing to microcontroller-specific I/O registers directly.

/*
extern crate avr_std_stub;

/// The data direction register for PORT B, which is mapped to 0x24 in memory on the atmega328.
const DDRB: *mut u8 = 0x24 as *mut u8;
/// The pin status register for PORT B, which is mapped to 0x25 in memory on the atmega328.
const PORTB: *mut u8 = 0x25 as *mut u8;

#[no_mangle]
pub extern fn main() {
//fn main() {
    unsafe {
        // Set the upper four physical pins on PORT B to inputs, the lower four to outputs.
        // The AVR interprets '1' in the data direction register as 'output', '0' input
        // for the corresponding pin.
        core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) | 0b00001111);

        // Write half of the output pins as high, the other half low.
        core::ptr::write_volatile(PORTB, 0b00001010);
    }
}
*/

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(test)]
mod test {
    #[test]
    fn test_foo() {
        assert_eq!(1, 1);
    }
}
