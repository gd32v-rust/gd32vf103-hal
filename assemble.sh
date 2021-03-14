#!/bin/bash

cur_path=$(cd `dirname $0`; pwd)
o_file=$cur_path/bin/riscv32imac-unknown-none-elf.o
a_file=$cur_path/bin/riscv32imac-unknown-none-elf.a
src_file=$cur_path/asm.S
compiler=""

which riscv64-unknown-elf-gcc >/dev/null 2>&1
if [ "$?" == 0 ]; then
	compiler=riscv64-unknown-elf-gcc
fi
which riscv32-unknown-elf-gcc >/dev/null 2>&1
if [ "$?" == 0 ]; then
	compiler=riscv32-unknown-elf-gcc
fi

if [ "$compiler" == "" ]; then
    echo "error: Cannot detect any assembly compiler!
You may install riscv32-unknown-elf-gcc with: ./configure --prefix=/opt/riscv32 --with-arch=rv32imac --with-abi=ilp32"
    exit
fi

$compiler $src_file -o $o_file -march=rv32imac -mabi=ilp32 -mno-relax -Wa,-mno-relax -c -g
if [ "$?" != 0 ]; then
    exit
fi
ar rcs $a_file $o_file
if [ "$?" != 0 ]; then
    exit
fi
rm $o_file

echo "ASSEMBLY SUCCESS"
