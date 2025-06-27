#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use hal::pac;
use panic_halt as _;
use rp2040_hal::{self as hal, Clock, pio::PIOExt};
use smart_leds::hsv::{Hsv, hsv2rgb};
use smart_leds::{RGB8, SmartLedsWrite};
use ws2812_pio::Ws2812;

#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[rp2040_hal::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    let mut timer = rp2040_hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Initialize the PIO and the WS2812 driver
    // Use GPIO16 for the WS2812 data line
    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let mut ws = Ws2812::new(
        pins.gpio16.into_mode(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    let mut hue: u8 = 0;
    // Create a loop to continuously update the LED colors
    loop {
        let data: [RGB8; 8] = core::array::from_fn(|i| {
            // Calculate the shifted hue for each of 8 LED Module
            // The hue is shifted by 20 degrees for each LED
            let shifted_hue = hue.wrapping_add((i * 20) as u8);
            hsv2rgb(Hsv {
                hue: shifted_hue,
                sat: 255,
                val: 64,
            })
        });
        // Write the data to the WS2812 LEDs
        // The `write` method takes an iterator of RGB8 values
        // and sends them to the WS2812 LEDs
        ws.write(data.iter().copied()).unwrap();
        // Delay to control the speed of the color shift
        timer.delay_ms(10u32);
        // Increment the hue to create a shifting effect
        hue = hue.wrapping_add(1);
    }
}
