#!/usr/bin/env sh
avr-objcopy -S -j .text -j .data -O ihex target/avr-atmega328p/debug/*.elf target/avr-atmega328p/debug/1.hex
avr-objcopy -S -j .text -j .data -O ihex target/avr-atmega328p/release/*.elf target/avr-atmega328p/release/1.hex

#BYTES=$(avr-size "$ELF" | tail -1 | cut -f4 | bc)
#echo "$ELF:" >&2
#echo "    $(numfmt --to=si "$BYTES") Bytes used ($BYTES exact)." >&2
