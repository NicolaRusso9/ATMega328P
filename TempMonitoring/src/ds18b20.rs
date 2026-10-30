//! # ATMega328P
//!
//! `ds18b20.rs` 
//! If you want, datasheet component is in __datasheet folder.
//! The DS18B20 uses a strict 1-Wire communication protocol to insure data integrity. Several signal types
//! are defined by this protocol: reset pulse, presence pulse, write 0, write 1, read 0, and read 1. The bus
//! master initiates all of these signals, with the exception of the presence pulse
//!
//! 1-Wire Timing:
//! Reset Low Time      480 µs min
//! Presence Detect     60-240 µs
//! Write Slot          60 µs
//! Read Slot           60 µs
//!

/// Pull the 1-Wire bus low.
///
/// PORTC0 is configured as an output and driven to GND.
#[inline(always)]
pub fn pin_low(port_b: &avr_device::atmega328p::PORTC) {
    port_b.portc().modify(|_, w| w.pc0().clear_bit());
    port_b.ddrc().modify(|_, w| w.pc0().set_bit()); // Output
}

/// Release the 1-Wire bus.
///
/// PB0 is switched back to input mode and the internal
/// pull-up resistor is enabled.
#[inline(always)]
pub fn pin_release(port_b: &avr_device::atmega328p::PORTC) {
    port_b.ddrc().modify(|_, w| w.pc0().clear_bit()); // Input
    port_b.portc().modify(|_, w| w.pc0().set_bit());  // Pull-up interno
}

/// Read the current logic level from the 1-Wire bus.
///
/// Returns:
/// - true  = logic high
/// - false = logic low
#[inline(always)]
pub fn pin_read(pin_b: &avr_device::atmega328p::portc::PINC) -> bool {
    pin_b.read().pc0().bit_is_set()
}

/// Send a 1-Wire reset pulse and check for sensor presence.
///
/// Sequence:
/// 1. Pull bus low for 480 µs.
/// 2. Release the bus.
/// 3. Wait 70 µs.
/// 4. Sample the presence pulse.
/// 5. Wait for the reset cycle to complete.
///
/// Returns true if a DS18B20 responds.
pub fn reset(
    port_b: &avr_device::atmega328p::PORTC,
    pin_b: &avr_device::atmega328p::portc::PINC,
    tc1: &avr_device::atmega328p::TC1
) -> bool {
    pin_low(port_b);
    delay_us(tc1, 480);
    pin_release(port_b);
    delay_us(tc1, 70);
    let presence = !pin_read(pin_b); // True if the sensor pulls the line down
    delay_us(tc1, 410);
    presence
}

/// Write a single bit on the 1-Wire bus.
///
/// Logic 1:
/// - short low pulse (~6 µs)
///
/// Logic 0:
/// - long low pulse (~60 µs)
///
/// Timing values follow the DS18B20 datasheet.
pub fn write_bit(
    port_b: &avr_device::atmega328p::PORTC,
    tc1: &avr_device::atmega328p::TC1,
    bit: bool
) {
    pin_low(port_b);
    if bit {
        delay_us(tc1, 6);
        pin_release(port_b);
        delay_us(tc1, 64);
    } else {
        delay_us(tc1, 60);
        pin_release(port_b);
        delay_us(tc1, 10);
    }
}

/// Read a single bit from the 1-Wire bus.
///
/// The master starts a read slot by briefly pulling
/// the bus low, then releases it.
/// The sensor drives the line and the level is sampled
/// approximately 10 µs later.
pub fn read_bit(
    port_b: &avr_device::atmega328p::PORTC,
    pin_b: &avr_device::atmega328p::portc::PINC,
    tc1: &avr_device::atmega328p::TC1
) -> bool {
    pin_low(port_b);
    delay_us(tc1, 2);
    pin_release(port_b);
    delay_us(tc1, 10);
    let bit = pin_read(pin_b);
    delay_us(tc1, 50);
    bit
}

/// Transmit one byte, least-significant bit first (LSB first).
pub fn write_byte(
    port_b: &avr_device::atmega328p::PORTC,
    tc1: &avr_device::atmega328p::TC1,
    byte: u8
) {
    for i in 0..8 {
        write_bit(port_b, tc1, (byte & (1 << i)) != 0);
    }
}

/// Read one byte, least-significant bit first (LSB first).
pub fn read_byte(
    port_b: &avr_device::atmega328p::PORTC,
    pin_b: &avr_device::atmega328p::portc::PINC,
    tc1: &avr_device::atmega328p::TC1
) -> u8 {
    let mut byte = 0;
    for i in 0..8 {
        if read_bit(port_b, pin_b, tc1) {
            byte |= 1 << i;
        }
    }
    byte
}

/// Millisecond delay helper built on top of delay_us().
pub fn delay_ms(tc1: &avr_device::atmega328p::TC1, ms: u16) {
    for _ in 0..ms {
        delay_us(tc1, 1000);
    }
}


/// The main reason that we need precise delays (in microseconds)
/// comes from the way the 1-Wire protocol used by the DS18B20 to work.
/// To synchronize data transmission between the microcontroller and the device, the 1-Wire protocol doesn't
/// have a clock wire. Since there is only a single data line (DQ), time acts as the clock. Communication
/// relies entirely on the precise millisecond or microsecond duration of electrical pulses.
#[inline(always)]
fn delay_us(tc1: &avr_device::atmega328p::TC1, us: u16) {
    // Reset counter of Timer1
    tc1.tcnt1().write(|w| unsafe { w.bits(0) });

    // Calculate necessary tick:
    // one tick = 0.5 µs, to get 'us' microseconds needs us * 2 ticks.
    let target_ticks = ((us as u32) * 2) as u16;

    // Wait until timer reaches the desired value.
    while tc1.tcnt1().read().bits() < target_ticks {
        core::hint::spin_loop(); // Tells to compiler that we're in active waiting
    }
}
