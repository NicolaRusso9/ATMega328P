[< HOME](../../README.md)

# 🚀 Compile & Flash Firmware

This page explains how to build and upload firmware to an **Arduino Uno R3 (ATMega328P)**.

---

## Building the Project

### Standard Release Build

Use the standard Rust build process:

```bash
cargo build --release
```

---

### Build Using the Generic AVR Target

Example using the built-in AVR target:

```bash
cargo build -Z build-std=core --target avr-none
```

---

### Build Using a Custom Target Specification

This repository includes a custom target definition for the ATMega328P.

```bash
cargo build --target atmega328p.json -Zjson-target-spec
```

> **Recommended:** this is the preferred build configuration for this repository.

---

## Generating the HEX File

After compiling the project, convert the generated executable into an Intel HEX file:

```bash
avr-objcopy \
    -O ihex \
    target/<target>/debug/blink \
    firmware.hex
```

Replace:

- `<target>` with your target directory.
- `blink` with the name of your executable.

---

## Flashing the Firmware

Upload the generated HEX file to the board using `avrdude`:

```bash
avrdude \
    -p m328p \
    -c arduino \
    -P /dev/ttyACM0 \
    -b 115200 \
    -U flash:w:firmware.hex:i
```

### Parameters

| Parameter                   | Description                         |
|-----------------------------|-------------------------------------|
| `-p m328p`                  | Target MCU (ATMega328P)             |
| `-c arduino`                | Arduino bootloader programmer       |
| `-P /dev/ttyACM0`           | Serial device                       |
| `-b 115200`                 | Baud rate                           |
| `-U flash:w:firmware.hex:i` | Writes the HEX file to flash memory |

---

## Successful Upload

If the flashing process completes successfully, you should see:

```text
avrdude done. Thank you.
```

---

## Troubleshooting

### Permission Denied

Make sure your user has access to the serial device:

```bash
sudo usermod -aG dialout $USER
```

Log out and back in for the change to take effect.

### Port Not Found

Check available serial devices:

```bash
ls /dev/ttyACM*
```

or

```bash
ls /dev/ttyUSB*
```

### Board Not Detected

Verify that:

- The USB cable supports data transfer.
- The correct serial port is selected.
- The Arduino bootloader is present on the MCU.

---

## Automation Script

This repository also provides an automation script that performs the build and flashing steps for you:

```bash
./build_and_install.sh
```

This is the easiest way to compile and deploy firmware during development.