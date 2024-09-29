#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m_rt::entry;

use stm32f4xx_hal::{
    pac::{self},
    prelude::*,
    spi::{Mode, NoMiso, Phase, Polarity},
    gpio::Speed,
    spi::*,
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpiob = dp.GPIOB.split();

    let sclk = gpiob
        .pb13
        .into_alternate()
        .speed(Speed::VeryHigh)
        .internal_pull_up(true);

    let mosi = gpiob
        .pb15
        .into_alternate()
        .speed(Speed::VeryHigh);

    let mut cs = gpiob.pb1.into_push_pull_output();

    let mode = Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    };

    let mut spi = Spi::new(dp.SPI2, (sclk, NoMiso::new(), mosi), mode, 200.kHz(), &clocks);

    let mut delay = dp.TIM1.delay_ms(&clocks);

    loop {
        cs.set_low();
        let spi_send_data: [u8; 5] = [0x0A, 0x0B, 0x0C, 0x0D, 0x0E];
        spi.write(&spi_send_data).unwrap();
        cs.set_high();

        delay.delay_ms(100);
    }
}
