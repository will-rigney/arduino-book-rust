#![no_std]
#![no_main]

use atmega328p_hal::prelude::*;
use panic_halt as _;

#[atmega328p_hal::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // button on pin 2
    let button = pins.d2.into_floating_input(&pins.ddr);

    // green led on pin 3
    let mut green = pins.d3.into_output(&pins.ddr);

    // red leds pins 4 & 5
    let mut red1 = pins.d4.into_output(&pins.ddr);
    let mut red2 = pins.d5.into_output(&pins.ddr);

    loop {
        // if the button is not pressed
        if button.is_low().unwrap_or(false) {
            // green is on
            green.set_high().void_unwrap();
            // both red off
            red1.set_low().void_unwrap();
            red2.set_low().void_unwrap();
        } else {
            // green is off
            green.set_low().void_unwrap();
            // one red off
            red1.set_low().void_unwrap();
            // one red one
            red2.set_high().void_unwrap();
            // wait
            arduino_uno::delay_ms(250);
            // swap
            red1.set_high().void_unwrap();
            red2.set_low().void_unwrap();
            // wait again
            arduino_uno::delay_ms(250);
        }
    }
}
