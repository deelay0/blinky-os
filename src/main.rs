#![no_std]
#![no_main]

//Modules
mod func;
mod gpio;
mod panic;
mod uart;

mod boot {
    use core::arch::global_asm;

    global_asm! {
        ".section .text._start"
    }
}

//Constants
const BAUD_RATE: u32 = 115200;
const LED_PIN: usize = 29;

// Pins 14 & 15 take TXD & RXD function at alternative function 0 (BCM2837 docs page 102)
const TXD_PIN: u32 = 14;
const RXD_PIN: u32 = 15;

fn blink_led(times: u32, delay: u32) {
    for _ in 0..times {
        unsafe {
            gpio::output_pin_clear(LED_PIN);
            func::simple_wait(delay);
            gpio::output_pin_clear(LED_PIN);
            func::simple_wait(delay);
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Init code

    let test_pin = 20;
    let mut level: bool;

    unsafe {
        gpio::funsel(LED_PIN, 1);
        gpio::funsel(test_pin, 1);
    }

    blink_led(3, 500);

    // Main loop
    loop {
        unsafe {
            gpio::output_pin_clear(LED_PIN);
            gpio::output_pin_set(test_pin);
            level = gpio::pin_level(test_pin);

            func::simple_wait(2000);

            if level {
                blink_led(5, 1000);
            } else if !level {
                blink_led(3, 1000);
            }

            gpio::output_pin_clear(test_pin);
            level = gpio::pin_level(test_pin);

            func::simple_wait(2000);

            if level {
                blink_led(5, 1000);
            } else if !level {
                blink_led(3, 1000);
            }
        }
    }
}
