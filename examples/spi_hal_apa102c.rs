#![no_main]
#![no_std]

#[allow(unused)]
use panic_halt;

use stm32f0xx_hal as hal;

use crate::hal::prelude::*;
use crate::hal::spi::Spi;
use crate::hal::spi::{Mode, Phase, Polarity};
use crate::hal::stm32;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    const MODE: Mode = Mode {
        polarity: Polarity::IdleHigh,
        phase: Phase::CaptureOnSecondTransition,
    };

    if let Some(p) = stm32::Peripherals::take() {
        let rcc = p.RCC.constrain();
        let clocks = rcc.cfgr.freeze();
        let gpioa = p.GPIOA.split();

        // Configure pins for SPI
        let sck = gpioa.pa5.into_alternate_af0();
        let miso = gpioa.pa6.into_alternate_af0();
        let mosi = gpioa.pa7.into_alternate_af0();

        // Configure SPI with 100kHz rate
        let mut spi = Spi::spi1(p.SPI1, (sck, miso, mosi), MODE, 100_000.hz(), clocks);

        // Cycle through colors on 16 chained APA102C LEDs
        loop {
            for r in 0..255 {
                let _ = spi.write(&[0, 0, 0, 0]);
                for _i in 0..16 {
                    let _ = spi.write(&[0b1110_0001, 0, 0, r]);
                }
                let _ = spi.write(&[0xFF, 0xFF, 0xFF, 0xFF]);
            }
            for b in 0..255 {
                let _ = spi.write(&[0, 0, 0, 0]);
                for _i in 0..16 {
                    let _ = spi.write(&[0b1110_0001, b, 0, 0]);
                }
                let _ = spi.write(&[0xFF, 0xFF, 0xFF, 0xFF]);
            }
            for g in 0..255 {
                let _ = spi.write(&[0, 0, 0, 0]);
                for _i in 0..16 {
                    let _ = spi.write(&[0b1110_0001, 0, g, 0]);
                }
                let _ = spi.write(&[0xFF, 0xFF, 0xFF, 0xFF]);
            }
        }
    }

    loop {
        continue;
    }
}
