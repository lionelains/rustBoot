#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;

extern crate stm32f4xx_hal as mcu;

#[cfg(feature = "defmt")]
use defmt_rtt as _; // global logger

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use mcu::delay::Delay;
use mcu::prelude::*;
use mcu::stm32;
// use panic_probe as _;

use rustBoot_hal::stm::stm32f446::FlashWriterEraser;
use rustBoot_update::update::{update_flash::FlashUpdater, UpdateInterface};

#[entry]
fn main() -> ! {
    if let (Some(peri), Some(cortex_peri)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let gpio = peri.GPIOA.split();
        let mut led = gpio.pa5.into_push_pull_output();

        let rcc = peri.RCC.constrain();
        let clocks1 = rcc.cfgr.sysclk(84.mhz()).freeze();
        let mut delay = Delay::new(cortex_peri.SYST, &clocks1);

        // GPIO Initialization
        let flash1 = peri.FLASH;
        let mut count = 0;

        while count < 6 {
            led.toggle();
            delay.delay_ms(500_u16);
            count = count + 1;
        }

        let flash_writer = FlashWriterEraser { nvm: flash1 };
        let updater = FlashUpdater::new(flash_writer);

        match updater.update_trigger() {
            Ok(_v) => {}
            Err(e) => panic!("couldnt trigger update: {}", e),
        }
    }
    //nvic_systemreset();
    mcu::pac::SCB::sys_reset()
}

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
