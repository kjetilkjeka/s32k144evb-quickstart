# s32k144evb-quickstart

> A template for building applications on the NXP S32k144EVB evaluation board

# Dependencies

- Nightly Rust toolchain: `rustup default nightly`
- ARM linker: `sudo apt-get install binutils-arm-none-eabi`
- Cargo `clone` subcommand: `cargo install cargo-clone`
- GDB: `sudo apt-get install gdb-arm-none-eabi`
- Jlink: `sudo apt-get install jlink`
- Xargo: `cargo install xargo`
- Bobbin: `cargo install bobbin`
- [Optional] Cargo `add` subcommand: `cargo install cargo-edit`

# Usage

- Clone this crate

``` text
$ cargo clone s32k144evb_quickstart && cd $_
```

- Change the crate name, author and version

``` text
$ edit Cargo.toml && head $_
[package]
authors = ["Kjetil Kjeka <kjetilkjeka@gmail.com>"]
name = "demo"
version = "0.1.0"
```

### Bobbin

If you have [installed j-link firmware](https://www.segger.com/products/debug-probes/j-link/models/other-j-links/opensda-sda-v2/)
on the OpenSDA debugger you can run `bobbin run` to compile, load and connect to serial to the S32K144EVB board.


# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
