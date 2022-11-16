PORT = /dev/ttyACM0
ELF = target/avr-atmega328p/release/xmas-tree.elf

default: build

.PHONY: build
build:
	cargo build --release

.PHONY: flash
flash:
	avrdude -p m328p -c arduino -P $(PORT) -b 115200 -U flash:w:$(ELF)