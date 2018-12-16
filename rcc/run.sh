#!/bin/sh
: ${CC:="gcc"}

cargo run > asm.s && "${CC}" asm.s && ./a.out
