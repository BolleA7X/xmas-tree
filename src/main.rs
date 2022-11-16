#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod logic;

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

    let mut digital_pins = [
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
        pins.d13.into_output().downgrade(),
    ];

    // Declare and initialize the interface with the animation logic
    let mut iface: logic::Interface = logic::EMPTY_INTERFACE;

    /* === LOOP === */

    loop {
        // Animation logic
        logic::exec_next_step(&mut iface);

        // Change the LEDs state
        for l in iface.leds_state.iter() {
            let led_pin: usize = l.0;
            let led_state: bool = l.1;

            if led_state {
                digital_pins[led_pin].set_high()
            } else {
                digital_pins[led_pin].set_low()
            }
        }

        // Sleep for the requested time
        arduino_hal::delay_ms(iface.wait_time_ms);
    }
}