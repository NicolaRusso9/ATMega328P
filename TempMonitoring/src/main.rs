//! In this example, we explore:
//! - DS18B20 module = Temperature
//! - I2C LCD1602 = LCD monitor
//!
//! The project will measure the room temperature and show the
//! output on the LCD.
#![no_std]
#![no_main]

mod ds18b20;
mod lcd1602;

use panic_halt as _;
use core::fmt::Write;
use heapless::String;

#[avr_device::entry]
fn main() -> ! {
    // Get peripherals from PAC of Atmega328P
    let dp = avr_device::atmega328p::Peripherals::take().unwrap();
    lcd1602::lcd_init(&dp);

   /*
    // ONLY TO TEST LCD
    let text: &[u8] = b"Who's next?";
    lcd1602::lcd_print_str(&dp, text);

    loop {
        core::hint::spin_loop();
    }
    */

    // Configure Timer1 with prescaler 8 (1 tick = 0.5 µs)
    dp.TC1.tccr1b().write(|w| unsafe {
        w.cs1().bits(0b010) // 0b010 = Prescaler 8
    });

    loop {
        // Send reset and check DS18B20
        if ds18b20::reset(&dp.PORTC, &dp.PORTC.pinc(), &dp.TC1) {
            // Start temperature conversion
            ds18b20::write_byte(&dp.PORTC, &dp.TC1, 0xCC); // Skip ROM
            ds18b20::write_byte(&dp.PORTC, &dp.TC1, 0x44); // Convert T

            // Wait for conversion (up to 750ms set on max resolution)
            ds18b20::delay_ms(&dp.TC1, 750);

            // Read data from Scratchpad of sensor
            ds18b20::reset(&dp.PORTC, &dp.PORTC.pinc(), &dp.TC1);
            ds18b20::write_byte(&dp.PORTC, &dp.TC1, 0xCC); // Skip ROM
            ds18b20::write_byte(&dp.PORTC, &dp.TC1, 0xBE); // Read Scratchpad

            let lsb = ds18b20::read_byte(&dp.PORTC, &dp.PORTC.pinc(), &dp.TC1);
            let msb = ds18b20::read_byte(&dp.PORTC, &dp.PORTC.pinc(), &dp.TC1);

            // Union of bytes of raw data
            let _raw_temp = ((msb as u16) << 8) | (lsb as u16);

            // Convert data to ° Celsius - Every unit value of _raw_temp is equals to 0.0625 °C (1/16)
            let temp_whole = _raw_temp >> 4; // Equivalent to division for 16 (integer)
            let temp_frac = (_raw_temp & 0x0F) * 625; // Get decimal (multiply for 1000 to get millesimi)

            // CCreate new string on stack with capacity of 32 byte
            let mut s: String<32> = String::new();

            // Write into string in secure mode
            write!(s, "Temp.: {}.{:02} C.", temp_whole, temp_frac /100).unwrap(); // Stampa: Temp.: 23.46 (rounded)

            // Get slice of byte (&[u8])
            let text: &[u8] = s.as_bytes();

            lcd1602::lcd_print_str(&dp, text, 0, 0);
        }

        // Wait before next temp read
        ds18b20::delay_ms(&dp.TC1, 5000);
    }
}