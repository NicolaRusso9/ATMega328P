#![no_std]   // no standard library.
#![no_main]  // no main entry point of Rust.
//#![feature(asm_experimental_arch)]  ---> ENABLE IF YOU WANT TO USE ASM ON AVR (NOT STABLE)

use core::ptr::{read_volatile, write_volatile};

use panic_halt as _; // In the event of a panic, the microcontroller stops.

/// MAPPING HARDWARE REGISTRY
///
/// DDRB  (Data Direction Register B)
///   - bit = 0 -> pin INPUT
///   - bit = 1 -> pin OUTPUT
///
/// PORTB (Port Data Register B)
///   - if pin is OUTPUT:
///         0 -> level LOW
///         1 -> level HIGH
///
/// PB5 == D13 of Arduino Uno motherboard.
///
/// Addresses memory-mapped:
///   DDRB  = 0x24
///   PORTB = 0x25
const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    // Read DDRB value and force its value at 1 for bit 5 --> Value = 0010_0000
    unsafe {
        let value = read_volatile(DDRB);
        write_volatile(DDRB, value | (1 << 5));
    }

    loop {
        // Led ON - Set PB5 to HIGH --> 0000_0000 | 0010_0000 = 0010_0000
        unsafe {
            let value = read_volatile(PORTB);
            write_volatile(PORTB, value | (1 << 5));
        }

        delay();

        // Led OFF - Set PB5 to LOW --> 1101_1111 & 0010_0000 = 0000_0000
        unsafe {
            let value = read_volatile(PORTB);
            write_volatile(PORTB, value & !(1 << 5));
        }

        delay();
    }
}

/// Busy-wait delay.
///
/// Executes a large number of NOP (No Operation) instructions.
///
/// Useful for:
/// - learning purposes
/// - hardware bring-up
/// - quick prototypes
///
/// Not recommended for production firmware because it wastes
/// CPU cycles. A hardware timer is usually preferred for
/// accurate and non-blocking delays.
fn delay() {
    for _ in 0..100_000_000 {
        unsafe {
            //core::arch::asm!("nop");  ---> ENABLE IF YOU ARER USING ASM ON AVR FEATURE
            core::ptr::read_volatile(&0);   // Alternative withouth asm on avr
        }
    }
}