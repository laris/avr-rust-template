#!/usr/bin/env sh
set -e
#set -ex

avrdude_bin="$(which avrdude)"
avr_target="atmega328p"
avr_isp="arduino"
serial_dev="/dev/tty.usbserial-0001"
serial_baud="115200"
target_path="avr-${avr_target}"
target_elf="$1"

${avrdude_bin} -p${avr_target} -c${avr_isp} -P ${serial_dev} -b${serial_baud} -D -Uflash:w:target/${target_path}/release/${target_elf}.elf:e
