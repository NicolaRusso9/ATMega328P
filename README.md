# 🦀 ATMega328P Rust Projects

🇬🇧 English | [🇮🇹 Italiano](./README.it.md)

> Collection of Rust 🦀 projects for Arduino Uno R3 (ATMega328P) 📟

---

## Overview

This repository contains multiple **Rust** projects targeting the **Arduino Uno R3 (ATMega328P)** platform.
The goal is to provide an organized environment for learning and experimenting with embedded Rust development using different abstraction levels:

- **HAL (Hardware Abstraction Layer)** for simpler and more portable development.
- **PAC (Peripheral Access Crate)** for direct peripheral access.
- **Bare Metal** programming for complete control over hardware registers without abstraction layers.

The **Blink LED** example introduces the three different approaches available for bare-metal development. Once these concepts are covered, HAL development will no longer be used since I personally believe it hides too much of how the hardware actually works, making it unclear what’s really going on.

---

## 🚀 Getting Started

Before building or running any project, make sure your development environment is properly configured.

### Documentation

- 📋 [System Requirements](./__docs/en/Required.md)
- 🔧 [Create a New Project with HAL](./__docs/en/HAL-Project.md)
- ⚙️ [Create a New Project with PAC](./__docs/en/PAC-Project.md)

---

## 📦 Working with Projects

You can:

- Run any existing project.
- Add a new project by creating a new Rust crate within the repository.
- Experiment with different development approaches (HAL, PAC, or Bare Metal).

---

## 🔨 Build & Flash

To simplify the build and firmware upload process, the repository includes the following helper script:

```bash
./build_and_install.sh
```

If you need to change the target board, MCU configuration, or compiler settings, you can update:

```text
atmega328p.json
```

---

## 📚 Manual Build & Flash

If you prefer not to use the automation script, follow the instructions provided in:

- 🚀 [Compile and Install Guide](./__docs/en/CompileAndInstall.md)

---

## 📂 Repository Structure

```text
ATMega328P/
├── __docs/
│   ├── en/
│   └── it/
├── project_1/
├── project_2/
├── ....
├── build_and_install.sh
├── atmega328p.json
└── README.md
```

---

## 🤝 Contributing

Contributions, improvements, bug fixes, and new project examples are welcome.

Feel free to open an issue or submit a pull request.