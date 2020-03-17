# GD32VF103-hal

## Helps on assembling

The assemble script requires you have `rv32imac-unknown-elf-gcc` installed.

Configure and compile GNU toolchain using:

```shell
../configure --prefix=/opt/riscv32 --with-arch=rv32imac --with-abi=ilp32
```

```shell
make && make install
```

Run assemble script: (run on any path is okay)

```shell
./gd32vf103-hal/asm.sh
```
