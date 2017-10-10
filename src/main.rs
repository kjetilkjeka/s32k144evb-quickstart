//! A template for building applications for ARM Cortex-M microcontrollers
//!
//! # Dependencies
//!
//! - Nightly Rust toolchain: `rustup default nightly`
//! - ARM linker: `sudo apt-get install binutils-arm-none-eabi`
//! - Cargo `clone` subcommand: `cargo install cargo-clone`
//! - GDB: `sudo apt-get install gdb-arm-none-eabi`
//! - Jlink: `sudo apt-get install jlink`
//! - Xargo: `cargo install xargo`
//! - Bobbin: `cargo install bobbin`
//! - [Optional] Cargo `add` subcommand: `cargo install cargo-edit`
//!
//! # Usage
//!
//! - Clone this crate
//!
//! ``` text
//! $ cargo clone cortex-m-quickstart && cd $_
//! ```
//!
//! - Change the crate name, author and version
//!
//! ``` text
//! $ edit Cargo.toml && head $_
//! [package]
//! authors = ["Kjetil Kjeka <kjetilkjeka@gmail.com>"]
//! name = "demo"
//! version = "0.1.0"
//! ```
//!
//! ### Bobbin
//!
//! If you have [installed j-link firmware](https://www.segger.com/products/debug-probes/j-link/models/other-j-links/opensda-sda-v2/)
//! on the OpenSDA debugger you can run `bobbin run` to compile, load and connect to serial to the S32K144EVB board.


#![feature(used)]
#![no_std]

#[macro_use]
extern crate cortex_m;
#[macro_use]
extern crate s32k144evb;

use cortex_m::asm;

use s32k144evb::wdog;

fn main() {
    let mut wdog_settings = wdog::WatchdogSettings::default();
    wdog_settings.enable = false;
    wdog::configure(wdog_settings);

    s32k144evb::serial::init();
    println!("Hello, world!");
    loop{}
}

// As we are not using interrupts, we just register a dummy catch all handler
#[allow(dead_code)]
#[used]
#[link_section = ".vector_table.interrupts"]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
