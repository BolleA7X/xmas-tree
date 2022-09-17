#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
    /* === SETUP === */

    // Extract HAL components
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Create list of leds
    // (downgrade() is necessary to make them be of the same type, for the array)

    let mut leds = [
        pins.d0.into_output().downgrade(),
        pins.d1.into_output().downgrade(),
        pins.d2.into_output().downgrade(),
        pins.d3.into_output().downgrade(),
        pins.d4.into_output().downgrade(),
        pins.d5.into_output().downgrade(),
        pins.d6.into_output().downgrade(),
        pins.d7.into_output().downgrade(),
        pins.d8.into_output().downgrade(),
        pins.d9.into_output().downgrade(),
        pins.d10.into_output().downgrade(),
        pins.d11.into_output().downgrade(),
        pins.d12.into_output().downgrade(),
        pins.d13.into_output().downgrade()
    ];

    // Create list of leds state
    let mut leds_state = [false; 14];

    /* === LOOP === */

    loop {
        // Animation logic
        leds_state[0] = !leds_state[0];
        arduino_hal::delay_ms(500);

        // Change the LEDs state
        for i in 0..leds.len() {
            if leds_state[i] {leds[i].set_high()} else {leds[i].set_low()}
        }
    }
}