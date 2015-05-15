#!/bin/bash

set -exuo pipefail

gcc -s -c tinyasm.s

objdump -dr tinyasm.o
echo

ld --gc-sections -e main -T script.ld -o payload tinyasm.o
objcopy -j combined -O binary payload payload.bin

ENTRY=$(nm -f posix payload | grep '^main ' | awk '{print $3}')
nasm -f bin -o tinyasm -D entry=0x$ENTRY elf.s

chmod +x tinyasm
hd tinyasm
wc -c tinyasm
