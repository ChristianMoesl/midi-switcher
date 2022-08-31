# MIDI Switcher

## Setup
First install rustup and then execute the following commands:
```shell
rustup target add thumbv6m-none-eabi
rustup component add llvm-tools-preview
cargo install uf2conv cargo-binutils
```

## Flash firmware binary to microcontroller
```shell
cp midi_switcher.uf2 /Volumes/PYGAMERBOOT/
```