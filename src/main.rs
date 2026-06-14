#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

use arduino_hal::adc;

#[arduino_hal::entry]
fn main() -> ! {
    let dp: arduino_hal::Peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let mut input_pin = pins.a0.into_analog_input(&mut adc);

    loop {
        arduino_hal::delay_ms(150);
    }
}
