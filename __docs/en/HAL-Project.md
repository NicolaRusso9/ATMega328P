[< HOME](../../README.md)
# 🔧 Creating a New Project with AVR-HAL

This guide explains how to create a new **Rust** project for the **Arduino Uno R3 (ATMega328P)** using the excellent **avr-hal** ecosystem maintained by Rahix.

Using the avr-hal template provides a fully configured development environment, allowing you to focus on firmware development instead of project setup.

---

## What is AVR-HAL?

The **Hardware Abstraction Layer (HAL)** offers a higher-level API that simplifies access to microcontroller peripherals compared to working directly with a PAC (Peripheral Access Crate).

Advantages of using a HAL:

- Easier peripheral configuration
- Safer abstractions
- Faster development
- Excellent for learning embedded Rust

---

## Install Required Tools

Install the tools used to generate projects and upload firmware.

```bash
cargo install cargo-generate
cargo install ravedude
```

Verify the installation:

```bash
cargo generate --version
ravedude --version
```

---

## Create a New Project

Generate a new project from the official AVR-HAL template:

```bash
cargo generate --git https://github.com/Rahix/avr-hal-template.git
```

During project creation:

1. Enter the project name.
2. Select **arduino-uno** as the target board.
3. Navigate to the generated directory.

```bash
cd <project-name>
```

4. Build the project.

```bash
cargo build
```

---

## Project Structure

The generated project already includes:

- Board configuration
- AVR target support
- Upload utilities
- Sample firmware
- Development dependencies

You can start modifying the source code immediately.

---

## Build & Flash Firmware

Once your project has been created, follow the instructions in:

- 📖 [Compile & Flash Firmware](./CompileAndInstall.md)

---

## Official Resources

- AVR-HAL: https://github.com/Rahix/avr-hal
- AVR-HAL Template: https://github.com/Rahix/avr-hal-template
