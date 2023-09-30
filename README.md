# MIDI Switcher

Firmware for a small little prototype board, which implements switching 
channels on a electric guitar amplifier. Switching channels cna be controlled through MIDI.

## Setup
First install rustup and then execute the following commands:
```shell
rustup target add thumbv6m-none-eabi
rustup component add llvm-tools-preview
cargo install uf2conv cargo-binutils
```

## Flash firmware binary to microcontroller
```shell
cargo objcopy --release --locked -- -O binary midi_switcher.bin
uf2conv midi_switcher.bin --base 0x2000 --output midi_switcher.uf2
cp midi_switcher.uf2 /Volumes/PYGAMERBOOT/
```
