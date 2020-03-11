#![no_main]
#![no_std]

#[allow(unused_extern_crates)]
extern crate panic_halt;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprint;
use stm32f3::stm32f303::{interrupt, Interrupt, NVIC};

static F_CPU: u32 = 8_000_000;

#[entry]
fn main() -> ! {
    let p = cortex_m::Peripherals::take().unwrap();

    let mut syst = p.SYST;

    unsafe {
        NVIC::unmask(Interrupt::EXTI0);
    }

    // configure the system timer to wrap around every second
    syst.set_clock_source(SystClkSource::External);
    syst.set_reload(F_CPU - 1); // 1s
    syst.enable_counter();

    loop {
        // busy wait until the timer wraps around
        while !syst.has_wrapped() {}

        // trigger the `EXTI0` interrupt
        NVIC::pend(Interrupt::EXTI0);
    }
}

#[interrupt]
fn EXTI0() {
    hprint!(".").unwrap();
}
