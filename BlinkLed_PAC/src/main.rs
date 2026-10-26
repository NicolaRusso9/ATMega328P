#![no_std]
#![no_main]

use panic_halt as _;

#[avr_device::entry]
fn main() -> ! {
    let dp = avr_device::atmega328p::Peripherals::take().unwrap();
    
    // PB5 come output
    dp.PORTB.ddrb().modify(|_, w| w.pb5().set_bit());

    loop {
        // LED ON
        dp.PORTB.portb().modify(|_, w| w.pb5().set_bit());

        delay();

        // LED OFF
        dp.PORTB.portb().modify(|_, w| w.pb5().clear_bit());

        delay();
    }
}

fn delay() {
    for _ in 0..100_000 {
        avr_device::asm::nop();
    }
}