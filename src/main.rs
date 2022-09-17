#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod animations;

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

    let mut led_pins = [
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

    // Declare and initialize the interface with the animation logic
    let mut iface: animations::Interface = animations::init_animations();

    /* === LOOP === */

    loop {
        // Animation logic
        iface.leds_state[0] = !iface.leds_state[0];
        arduino_hal::delay_ms(iface.wait_time_ms);

        // Change the LEDs state
        for i in 0..led_pins.len() {
            if iface.leds_state[i] {led_pins[i].set_high()} else {led_pins[i].set_low()}
        }
    }
}