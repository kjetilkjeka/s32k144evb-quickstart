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
