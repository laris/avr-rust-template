#!/usr/bin/env sh
set -e
#set -ex

CRATE_NAME="$(grep name Cargo.toml | cut -f2 -d"=" | cut -f2 -d"\"")"
AVR_TARGET="atmega328p"
AVR_TARGET_JSON="avr-$AVR_TARGET.json"

cargo build --target $AVR_TARGET_JSON -Z build-std=core --all --release

#if [ $# -gt 2 -o $1 = "--help" ]; then
#    echo "usage: $0 [--release|--debug] <elf-name>" >&2
#    exit 1
#fi
#
#BUILD="debug"
#if [ "$1" = "--debug" ]; then
#    shift 1
#    BUILD="debug"
#elif [ "$1" = "--release" ]; then
#    shift 1
#    BUILD="release"
#fi

TARGET="$(realpath --relative-to="$(pwd)" "$(dirname "$0")/target")"
#HEX="$TARGET/$1.hex"
#HEX="$1.hex"
HEX="$(grep name Cargo.toml | cut -f2 -d"=" | cut -f2 -d"\"")"".hex"
#ELF="$(echo "$TARGET"/avr-*/"$BUILD/examples/$1.elf")"
ELF="$(echo "$TARGET"/avr-$AVR_TARGET/"release/$CRATE_NAME.elf")"

if [ ! -e "$ELF" ]; then
    echo "No $1.elf found.  The following binaries exist:" >&2
    for target_dir in "$TARGET"/avr-*; do
        for bin in "$target_dir/$BUILD/examples"/*.elf; do
            echo "  - $(basename -s.elf "$bin")" >&2
        done
    done
    exit 1
fi

ln -sf $ELF .
avr-objcopy -S -j .text -j .data -O ihex "$ELF" "$HEX"

BYTES=$(avr-size "$ELF" | tail -1 | cut -f4 | bc)
echo "$ELF:" >&2
echo "    $(numfmt --to=si "$BYTES") Bytes used ($BYTES exact)." >&2
