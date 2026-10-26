[< HOME](../../README.md)

# 📋 System Requirements

This page describes all required dependencies and configuration steps needed to build, flash, and debug Rust applications for the **Arduino Uno R3 (ATMega328P)**.

---

# 1. Install the AVR Toolchain

Update the package index and install the required AVR development tools:

```bash
sudo apt update

sudo apt install \
    gcc-avr \
    avr-libc \
    binutils-avr \
    avrdude \
    make \
    pkg-config \
    libudev-dev
```

---

# 2. Verify the Installation

Ensure the toolchain has been installed correctly:

```bash
avr-gcc --version
avr-objcopy --version
avrdude -?
```

If all commands return version information, the AVR toolchain is ready to use.

---

# 3. Install the Rust Nightly Toolchain

AVR development currently requires the Rust nightly toolchain.

Install it with:

```bash
rustup toolchain install nightly
```

---

## Option A: Use Nightly Globally

Set nightly as the default Rust toolchain for your system:

```bash
rustup default nightly
```

---

## Option B: Use Nightly Only for This Repository

Navigate to your project directory and configure a local override:

```bash
rustup override set nightly
```

This approach is recommended if you also work on projects that use the stable toolchain.

---

# 4. Install the Rust Source Component

Install the Rust source code for the active toolchain:

```bash
rustup component add rust-src
```

This component is required when compiling the Rust standard libraries for custom AVR targets.

---

# 5. Verify Board Detection

Connect the Arduino Uno and verify that Linux detects the board.

## Check Serial Devices

```bash
ls /dev/ttyACM*
```

Expected output:

```text
/dev/ttyACM0
```

Depending on your hardware, the device number may differ.

---

## Alternative: Verify Using lsusb

```bash
lsusb
```

Example output:

```text
Bus 001 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub
Bus 001 Device 006: ID 04d9:a088 Holtek Semiconductor, Inc. USB Gaming Mouse
Bus 001 Device 008: ID 0c45:7603 Microdia USB Keyboard
Bus 001 Device 009: ID 0b05:1996 ASUSTek Computer, Inc. USB Audio
Bus 001 Device 010: ID 2341:0043 Arduino SA Uno R3 (CDC ACM)
Bus 002 Device 001: ID 1d6b:0003 Linux Foundation 3.0 root hub
```

If the Arduino entry is visible, the board has been detected successfully.

---

# 6. Enable Serial Port Access

Most Linux distributions restrict access to serial devices for non-privileged users.

Add your user account to the `dialout` group:

```bash
sudo usermod -aG dialout $USER
```

> ⚠️ After running this command, you must log out and log back in (or reboot) before the change takes effect.

---

## Command Explanation

| Option    | Description                                                                |
|-----------|----------------------------------------------------------------------------|
| `sudo`    | Executes the command with administrative privileges                        |
| `usermod` | Modifies a user account                                                    |
| `-aG`     | Adds the user to an additional group without removing existing memberships |
| `dialout` | Linux group that grants access to serial devices                           |
| `$USER`   | Current logged-in user                                                     |

---

# ✅ Verification Checklist

Before continuing, make sure:

- [ ] AVR toolchain is installed
- [ ] `avr-gcc` is available
- [ ] `avr-objcopy` is available
- [ ] `avrdude` is available
- [ ] Rust nightly is installed
- [ ] `rust-src` component is installed
- [ ] Arduino Uno appears as `/dev/ttyACM*`
- [ ] Your user belongs to the `dialout` group

Once all items are checked, you're ready to build and flash firmware to the ATMega328P.