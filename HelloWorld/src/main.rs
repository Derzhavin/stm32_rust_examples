#![no_std]
#![no_main]

use stm32f4::stm32f446;

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;

// use `main` as the entry point of this application
#[entry]
fn main() -> ! {
    // get handles to the hardware
    let peripherals = stm32f446::Peripherals::take().unwrap();
    let gpioa = &peripherals.GPIOA;
    let rcc = &peripherals.RCC;

    rcc.ahb1enr.modify(|_, w| w.gpioaen().set_bit());
    
    gpioa.moder.write(|w| {
        w.moder5().bits(0b01)
    });

    loop{
        gpioa.bsrr.write(|w| w.bs5().set_bit());
        cortex_m::asm::delay(2000000);
        
        gpioa.bsrr.write(|w| w.br5().set_bit());
        cortex_m::asm::delay(2000000);
    }
}