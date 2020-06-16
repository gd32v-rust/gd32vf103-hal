# `gd32vf103-hal`

> Hardware abstract layer ([HAL]) for the GD32VF103 RISC-V microcontroller

[HAL]: https://crates.io/crates/embedded-hal

[![crates.io](https://img.shields.io/crates/v/gd32vf103-hal.svg)](https://crates.io/crates/gd32vf103-hal)
[![Released API docs](https://docs.rs/gd32vf103-hal/badge.svg)](https://docs.rs/gd32vf103-hal)

This project is under early stage development; you may find example project and images [here](https://github.com/luojia65/example-gd32vf103).

Matrix: [#gd32v-rust:matrix.org](https://matrix.to/#/#gd32v-rust:matrix.org)

See also: [gd32vf103xx-hal](https://github.com/riscv-rust/gd32vf103xx-hal)

## Use this project

To use this HAL project, you may need Rust installed. Checkout `rustup.rs` if you don't have one.
You do not need to install GNU toolchain if you are an application developer.

Examples may be found at [`gd32vf103-example`](https://github.com/gd32v-rust/gd32vf103-exmpale).

## Helps on assembling

ALERT: this section is only for HAL project itself. If you are an application developer, you do not
need to build this binary by yourself.

The assemble script requires you have `riscv32imac-unknown-elf-gcc` installed.

Configure and compile GNU toolchain using:

```shell
../configure --prefix=/opt/riscv32 --with-arch=rv32imac --with-abi=ilp32
```

```shell
make && make install
```

Run assemble script: (run on any path is okay)

```shell
./assemble.sh
```

## License

This project is licensed under either of

- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Mulan PSL v1 ([LICENSE-MULAN](LICENSE-MULAN) or [http://license.coscl.org.cn/MulanPSL](http://license.coscl.org.cn/MulanPSL))
