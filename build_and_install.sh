#!/bin/bash

counter=1
dirs=()

# Searching directories
for dir in */; do
  # Directories that start with "__" is used for documentation
  if [[ $dir =~ ^__ ]]; then
          continue
    fi

    # Add directory to array
    dirs+=("$dir")

    echo "$counter) $dir"
    ((counter++))
done

# Require project to build:
declare -i choice
read -r -p "Enter a number of project do you want build:" choice

# Input Validation
if (( choice < 1 || choice > ${#dirs[@]} )); then
    echo "Not valid input. Quitting..."
    exit 1
fi

# Removing last slash and assign value to var
selected_dir="${dirs[$((choice - 1))]%/}"

echo "Selected project: $selected_dir"

# Move into selected directory
cd "$selected_dir" || exit 1

# Set nightly toolchain
#rustup override set nightly

# Build
cargo build --release --target ../atmega328p.json -Zjson-target-spec

# Generate hex
avr-objcopy \
    -O ihex \
    target/atmega328p/release/"$selected_dir".elf \
    firmware.hex

devices=()
counter=1

# Check the device attached to system
for dev in /dev/ttyACM*; do
    [[ -e "$dev" ]] || continue

    devices+=("$dev")
    echo "$counter) $dev"
    ((counter++))
done

if (( ${#devices[@]} == 0 )); then
    echo "Board not found."
    exit 1
fi

read -p "select serial port: " choice

if (( choice < 1 || choice > ${#devices[@]} )); then
    echo "Choice not valid."
    exit 1
fi

DEVICE="${devices[$((choice-1))]}"

#sudo usermod -aG dialout $USER

echo "Selected device: $DEVICE ."
echo "Flashing firmware..."

# Write on board
avrdude \
    -p m328p \
    -c arduino \
    -P "$DEVICE" \
    -b 115200 \
    -U flash:w:firmware.hex:i
