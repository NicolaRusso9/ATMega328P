#![no_std]
use avr_device::atmega328p::{Peripherals, USART0};

pub struct Uart {
    usart: USART0,
}

impl Uart {
    pub fn new(usart: USART0) -> Self {
        Self { usart }
    }

    pub fn write_byte(&self, byte: u8) {
        while self.usart.ucsr0a().read().udre0().bit_is_clear() {}

        self.usart.udr0().write(|w| unsafe {
            w.bits(byte)
        });
    }

    pub fn write_str(&self, text: &[u8]) {
        for &byte in text {
            self.write_byte(byte);
        }
    }

    pub fn write_u16(&self, mut value: u16) {
        let mut buffer = [0u8; 5];
        let mut i = 0;

        Self::write_str(&self, b"Dinstance: ");

        if value == 0 {
            self.write_byte(b'0');
            return;
        }

        while value > 0 {
            buffer[i] = (value % 10) as u8 + b'0';
            value /= 10;
            i += 1;
        }

        while i > 0 {
            i -= 1;
            self.write_byte(buffer[i]);
        }

        Self::write_str(&self, b"\n\r");
    }
}