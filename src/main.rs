//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!");

    let p = cortex_m::Peripherals::take().unwrap();
    let mut syst = p.SYST;

    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000); // period = 1s
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();




    // // exit QEMU
    // // NOTE do not run this on hardware; it can corrupt OpenOCD state
    // debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

#[exception]
fn SysTick() {
    // NOTE: the exception function name is pre-defined by cortex-m-rt
    static  mut COUNT: u32 = 0;
    *COUNT += 1;
    if *COUNT > 10 {
        debug::exit(debug::EXIT_SUCCESS);
    }
    else {
        hprintln!("system tick triggered");
    }
}
