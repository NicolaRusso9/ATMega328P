[< HOME](../../README.md)
# 🚀 Debugging with UART

When developing embedded applications, it is often necessary to view values 
read from sensors or monitor the state of a running application.

This guide describes a simple solution based on UART communication for debugging firmware developed for the ATmega328P using Rust.

---
## Requirements
### Software installation on the development PC

Install `picocom`, a lightweight serial terminal used to read messages sent from the microcontroller's UART.
```bash
sudo apt install picocom
```

---

### Viewing serial messages
To connect to the Arduino's serial port and begin viewing debug messages:
```bash
picocom -b 115200 /dev/ttyACM0
```

Where:
- 115200 is the baud rate configured in the firmware.
- /dev/ttyACM0 is the serial port exposed by the Arduino on the Linux system.

Once the connection is established, all data sent via UART will be displayed in the terminal.

---

### Exiting Picocom
To end the monitoring session:
```
Ctrl + A
Ctrl + X
```
> Note: These two commands must be executed in sequence, not simultaneously.
---

### Sending UART Messages
To simplify debugging tasks, a minimal UART library is available at: `./library/uart`
Before using it, you must initialize the USART peripheral. The following example configures:
- MCU clock: 16 MHz
- Baud rate: 115200
- Frame format: 8N1 (8 data bits, no parity, 1 stop bit)

```rust
// 16 MHz - 115200 baud
dp.USART0
    .ubrr0()
    .write(|w| unsafe { w.bits(8) });

// Enable transmission (TX)
dp.USART0.ucsr0b().write(|w| {
    w.txen0().set_bit()
});

// 8N1 configuration:
// - 8 data bits
// - No parity
// - 1 stop bit
dp.USART0.ucsr0c().write(|w| unsafe {
    w.bits(0b0000_0110)
});

let uart = Uart::new(dp.USART0);
```

Once the peripheral has been initialized, you can send a debug message:
`uart.write_str(b"Hi, we're connected!\r\n");`