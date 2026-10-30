//! # ATMega328P
//!
//! `lcd1602.rs`
//! If you want, datasheet component is in __datasheet folder.
//! To full understand, related datasheet are:
//! - HD44780.pdf   (LCD datasheet)
//! - PCF8574_PCF8574A.pdf  (Remote 8-bit I/O expander for I2C-bus with interrupt (NXP))
use avr_device::atmega328p::Peripherals;

/// PCF8574 default address.
/// PCF8574:
/// Fixed bits = 0100
/// A2 A1 A0 are configured by hardware pins.
///
/// Common backpack:
/// A2=A1=A0=1
///
/// Binary:
/// 0100_111 = 0x27
///
/// PCF8574A uses:
/// 0111_xxx
///
/// Typical address:
/// 0111_111 = 0x3F
const LCD_ADDR: u8 = 0x27;

/// 0000000001
/// Clears entire display and sets DDRAM address 0 in address counter
const LCD_CLEAR: u8 = 0x01;

/// 000000001—
/// Sets DDRAM address 0 in address counter.
/// Also returns display from being
/// shifted to original position.
/// DDRAM contents remain unchanged.
const LCD_RETURN_HOME: u8 = 0x02;

/// 00000001(I/D)(S)
/// Sets cursor move direction and specifies display shift.
/// These operations are performed during data write
/// and read.
/// Shifting operation:
/// | 1x -> Increment (direction)
/// | 0x -> Decrement (direction)
/// | x1 -> Accompanies display shift
const ENTRY_MODE_SET: u8 = 0x04; // Must combine in OR with increment or shift
const ENTRY_INCREMENT: u8 = 0x02;
const ENTRY_SHIFT: u8 = 0x01;

/// 0000001(D)(C)(B)
/// D = Sets entire display (1 on, 0 off)
/// C = Cursor (1 on, 0 off)
/// B = Blinking cursor position (1 on, 0 off)
///
/// Eg. DISPLAY_CONTROL + B = 0000001000 | 0000000001 == 0000001001 => 0x09
/// Eg. DISPLAY_CONTROL + D + B = 0000001000 | 0000000100 | 0000000001 == 0000001101 => 0x0D
const DISPLAY_CONTROL: u8 = 0x08;
const DISPLAY_ON: u8 = 0x04;
const CURSOR_ON: u8 = 0x02;
const BLINK_ON: u8 = 0x01;

/// Moves cursor and shifts display without changing DDRAM contents.
/// 000001(S/C)(R/L)00
///
/// SC = 1 Display shift
/// SC = 0 Cursor move
/// RL = 1 Shift to the right
/// RL = 0 Shift to the left
const CURSOR_OR_DISPLAY_SHIFT: u8 = 0x10;

/// Sets interface data length
/// (DL), number of display lines
/// (N), and character font (F).
/// 00001(DL)(N)(F)00
const SET_FUNCTION: u8 = 0x20;

/// RS =    0: Instruction register (for write) Busy flag: address counter (for read)
///         1: Data register (for write and read)
/// R/W =   0: Write
///         1: Read
///
/// Funzione principale per l'invio di un byte intero (spezzato in 2 nibble)
fn send_signals(dp: &Peripherals, rs: bool, rw: bool, byte: u8) {
    // ex. A = 0100_0001 --> high_nibble=0100  low_nibble=0001
    let high_nibble = (byte >> 4) & 0x0F;
    send_nibble(dp, high_nibble, rs, rw);

    let low_nibble = byte & 0x0F;
    send_nibble(dp, low_nibble, rs, rw);

    avr_device::asm::delay_cycles(800); // ~50us
}

pub fn lcd_clear(dp: &Peripherals) {
    send_signals(dp, false, false, LCD_CLEAR);
    avr_device::asm::delay_cycles(32000); // ~2 ms
}

pub fn lcd_display_on(dp: &Peripherals) {
    send_signals(
        dp,
        false,
        false,
        DISPLAY_CONTROL | DISPLAY_ON //| CURSOR_ON | BLINK_ON
    );
}

pub fn lcd_set_cursor(dp: &Peripherals, row: u8, col: u8) {
    let addr = match row {
        0 => col,
        1 => 0x40 + col,        // 0x40 is the DDRMA address [0-40][41-80]
        _ => return,
    };

    send_signals(dp, false, false, 0x80 | addr);
}

pub fn lcd_char(dp: &Peripherals, character: u8) {
    send_signals(dp, true, false, character); // RS = 1 for data (text to write on display)
    avr_device::asm::delay_cycles(4000);
}

pub fn lcd_print_str(dp: &Peripherals, text: &[u8], row: u8, col: u8) {
    let mut counter: i8 = 0;
    lcd_set_cursor(dp, row, col);

    for i in 0..text.len() {
        if counter == 16 {
            lcd_set_cursor(dp, row + 1, 0);
        }

        lcd_char(dp, text[i]);
        counter += 1;
    }
}

pub fn lcd_entry_mode(dp: &Peripherals, increment: bool, shift: bool) {
    // RS = 0  for command configuration of display
    let mut cmd = ENTRY_MODE_SET;
    if increment { cmd |= ENTRY_INCREMENT; }
    if shift { cmd |= ENTRY_SHIFT; }
    send_signals(dp, false, false, cmd);
}


/// Init TWI (I2C) of ATmega328P
pub fn twi_init(dp: &Peripherals) {
    // Setting bit rate (es. 100kHz con F_CPU = 16MHz -> TWBR = 72, Prescaler = 1)
    dp.TWI.twbr().write(|w| unsafe { w.bits(72) });
    dp.TWI.twsr().write(|w| unsafe { w.bits(0) }); // Prescaler = 1
    // Enable TWI module
    dp.TWI.twcr().write(|w| w.twen().set_bit());
}

/// Send single byte over TWI (I2C) to PCF8574T
fn twi_write_byte(dp: &Peripherals, data: u8) {
    // Send START
    dp.TWI.twcr().write(|w| {
        w.twint().set_bit();
        w.twsta().set_bit();
        w.twen().set_bit()
    });
    while !dp.TWI.twcr().read().twint().bit_is_set() {}

    // Send Slave Address (SLA+W)
    dp.TWI.twdr().write(|w| unsafe { w.bits(LCD_ADDR << 1) });
    dp.TWI.twcr().write(|w| {
        w.twint().set_bit();
        w.twen().set_bit()
    });
    while !dp.TWI.twcr().read().twint().bit_is_set() {}

    // Send data byte (pin PCF8574)
    dp.TWI.twdr().write(|w| unsafe { w.bits(data) });
    dp.TWI.twcr().write(|w| {
        w.twint().set_bit();
        w.twen().set_bit()
    });
    while !dp.TWI.twcr().read().twint().bit_is_set() {}

    // Send STOP
    dp.TWI.twcr().write(|w| {
        w.twint().set_bit();
        w.twsto().set_bit();
        w.twen().set_bit()
    });
}

/// Send single nibble (4 bit) over control pin (RS, Backlight ed Enable)
fn send_nibble(dp: &Peripherals, nibble: u8, rs: bool, rw: bool) {
    let mut base_val = 0;

    // P0 = RS
    if rs {
        base_val |= 0x01;   // P0
    }

    // P1 = R/W (0 -> Write - 1 -> Read)
    if rw {
        base_val |= 0x02;   // P1
    }

    // P3 = Backlight (always on 1 -> 0x08)
    base_val |= 0x08;

    // Set nibble on pin from P4 to P7
    base_val |= (nibble & 0x0F) << 4;

    // Impulse Enable HIGH (P2 = 1 -> 0x04)
    twi_write_byte(dp, base_val | 0x04);

    // Low hardware delay for latch
    avr_device::asm::delay_cycles(100);

    // Impulse Enable LOW (P2 = 0)
    twi_write_byte(dp, base_val & !0x04);

    avr_device::asm::delay_cycles(100);
}


/// Init display and I2C communication.
pub fn lcd_init(dp: &Peripherals) {
    twi_init(dp);

    // Attesa iniziale per l'accensione dell'LCD (> 40ms)
    for _ in 0..100 {
        avr_device::asm::delay_cycles(16000); // Circa 1ms a 16MHz
    }

    // Sequenza di reset forzato a 4 bit (come richiesto dal datasheet HD44780)
    send_nibble(dp, 0x03, false, false);
    avr_device::asm::delay_cycles(80000); // ~5ms
    send_nibble(dp, 0x03, false, false);
    avr_device::asm::delay_cycles(16000); // ~1ms
    send_nibble(dp, 0x03, false, false);
    send_nibble(dp, 0x02, false, false); // Imposta la modalità a 4 bit
    avr_device::asm::delay_cycles(16000);       // ~1 ms

    // Configurazione standard
    send_signals(dp, false, false, SET_FUNCTION | 0x08); // Function Set: 4-bit, 2 lines, 5x8 font (0x28)
    lcd_display_on(dp);
    lcd_entry_mode(dp, true, false);
    lcd_clear(dp);
}
