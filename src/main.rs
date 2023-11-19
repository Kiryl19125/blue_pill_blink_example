//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![deny(unsafe_code)]
#![no_std]
#![cfg_attr(not(doc), no_main)]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use nb::block;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    // Init buffers for debug printing
    rtt_init_print!();
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let mut gpioc = dp.GPIOC.split();
    let mut gpiob = dp.GPIOB.split();

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    let mut led_2 = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    
    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(1.Hz()).unwrap();

    rprintln!("Hello, Rust!");
    // Wait for the timer to trigger an update and change the state of the LED
    let mut i = 0;
    rprintln!("I ma starting blinking!!!");
    loop {
        block!(timer.wait()).unwrap();
        // led.set_high();
        led_2.set_high();
        block!(timer.wait()).unwrap();
        // led.set_low();
        led_2.set_low();
        i += 1;
        rprintln!("I have blinked {} times.", i);
        if i == 10 {
            panic!("Yow, 10 times is enough!");
        }
    }
}
