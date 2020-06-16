$source = "$PSScriptRoot/asm.S"
$o_file = "$PSScriptRoot/bin/riscv32imac-unknown-none-elf.o"
$a_file = "$PSScriptRoot/bin/riscv32imac-unknown-none-elf.a"

riscv64-unknown-elf-gcc $source -o $o_file -march=rv32imac -mabi=ilp32 -c -g
riscv64-unknown-elf-ar rcs $a_file $o_file
Remove-Item $o_file
