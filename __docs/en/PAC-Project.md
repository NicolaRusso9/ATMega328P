[< HOME](../../README.md)
# Creating a New Project with PAC

This guide describes the minimum setup required to develop Rust firmware for the **ATMega328P** using the **Peripheral Access Crate (PAC)** directly.
Unlike a HAL-based project, working with a PAC gives you direct access to the microcontroller peripherals and registers, making it ideal for learning how the hardware works internally.

---

## What is a PAC?

A **Peripheral Access Crate (PAC)** is a low-level Rust crate generated from the microcontroller's datasheet.

Using a PAC allows you to:

- Access hardware registers directly
- Configure peripherals without abstraction layers
- Write lightweight firmware
- Better understand the MCU architecture

If you prefer easier peripheral configuration and higher-level APIs, consider using the HAL approach instead.

---

## Prerequisites

Complete the basic system setup first:

- 📋 [Required](Required.md)

---

## Create a New Project

Create a standard Rust binary project:

```bash
cargo new blink
cd blink
```

---

## Add Required Dependencies

Edit your `Cargo.toml` file:

```toml
[dependencies]
avr-device = "0.9"
panic-halt = "1.0"

[profile.release]
panic = "abort"
lto = true
opt-level = "s"
```

### Dependency Overview

| Dependency   | Purpose                                           |
|--------------|---------------------------------------------------|
| `avr-device` | Peripheral Access Crates for AVR microcontrollers |
| `panic-halt` | Stops program execution when a panic occurs       |

---

## Verify AVR Target Support

Check whether Rust recognizes AVR targets:

```bash
rustc --print target-list | grep avr
```

Example output:

```text
avr-none
```

---

## Software Stack

A minimal PAC-based environment consists of:

- Rust Nightly
- rust-src
- avr-device (PAC)
- panic-halt
- avr-gcc
- avr-objcopy
- avrdude

---

## Tools Not Required

The following tools are commonly used with **avr-hal**, but are not required when developing directly with a PAC:

```text
cargo-generate
ravedude
```

You may still install them if you use HAL-based projects elsewhere.

---

## PAC vs HAL

| Feature                | PAC | HAL        |
|------------------------|-----|------------|
| Direct register access | ✅  | ❌         |
| Low-level control      | ✅  | ⚠️ Partial |
| Ease of use            | ❌  | ✅         |
| Learning MCU internals | ✅  | ⚠️         |
| Faster development     | ❌  | ✅         |

---

## Recommended Project Structure

```text
blink/
├── src/
│   └── main.rs
├── Cargo.toml
├── atmega328p.json
└── .cargo/
    └── config.toml
```

---

## Build & Flash Firmware

Once the project has been created, follow the instructions in:

- 📖 [Compile & Install](CompileAndInstall.md)
