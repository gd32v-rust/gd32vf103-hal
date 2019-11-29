@echo off
riscv64-unknown-elf-gcc asm.S -o bin/riscv32imac-unknown-none-elf.o -march=rv32imac -mabi=ilp32 -c -g
wsl ar rcs bin/riscv32imac-unknown-none-elf.a bin/riscv32imac-unknown-none-elf.o
wsl rm bin/*.o
