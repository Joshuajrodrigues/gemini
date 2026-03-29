#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use defmt::info;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_println as _;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.2.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let last = Output::new(peripherals.GPIO12, Level::High, OutputConfig::default());
    let mid = Output::new(peripherals.GPIO32, Level::High, OutputConfig::default());
    let first = Output::new(peripherals.GPIO13, Level::High, OutputConfig::default());

    let mut led_array = [first, mid, last];

    let mut loop_count = 0;
    let mut speed = 500;
    loop {
        if loop_count % 5 == 0 && speed != 100 {
            speed -= 100;
        }
        if loop_count > 200 {
            speed = 500;
            loop_count = 0;
        }
        led_array.iter_mut().for_each(|led| {
            led.toggle();
            let delay_start = Instant::now();
            while delay_start.elapsed() < Duration::from_millis(speed) {}
            led.toggle();
        });

        for led in led_array.iter_mut().rev().skip(1).take(1) {
            led.toggle();
            let delay_start = Instant::now();
            while delay_start.elapsed() < Duration::from_millis(speed) {}
            led.toggle();
            loop_count += 1;
        }
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0/examples
}
