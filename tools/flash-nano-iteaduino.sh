#!/usr/bin/env sh
set -e
#set -ex

avrdude_bin="$(which avrdude)"
avr_target="atmega328p"
avr_isp="arduino"
serial_dev="/dev/tty.usbserial-0001"
serial_baud="115200"
target_path="avr-${avr_target}"
target_elf="$(ls target/${target_path}/release/*.elf)"
target_elf_fname=${fname_ext%.*}
# append additional args $*
${avrdude_bin} $* -D -p${avr_target} -b${serial_baud} -c${avr_isp} -P${serial_dev} -Uflash:w:${target_elf}:e
