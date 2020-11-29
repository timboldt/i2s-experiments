#![no_main]
#![no_std]

use crate::hal::delay::Delay;
use crate::hal::prelude::*;
use stm32l4xx_hal as hal;

use i2s_rs as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello, world!");

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Default clock rate is 8MHz.
    // TODO: Correctly configure the clocks to run at full speed.
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut led = gpiob
        .pb3
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut timer = Delay::new(cp.SYST, clocks);
    loop {
        timer.delay_ms(500 as u32);

        defmt::info!("On.");
        led.set_high().ok();

        timer.delay_ms(500 as u32);

        defmt::info!("Off.");
        led.set_low().ok();
    }
}
