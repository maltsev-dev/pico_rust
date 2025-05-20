#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
// use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin};
use hal::pac;
use panic_halt as _;
use rp2040_hal as hal;
// use rp2040_hal::gpio::Pin;

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

    let mut led0 = pins.gpio16.into_push_pull_output();
    let mut led1 = pins.gpio15.into_push_pull_output();
    let mut led2 = pins.gpio18.into_push_pull_output();
    let mut led3 = pins.gpio19.into_push_pull_output();
    let mut led4 = pins.gpio20.into_push_pull_output();
    let mut led5 = pins.gpio21.into_push_pull_output();
    let mut led6 = pins.gpio22.into_push_pull_output();
    let mut led7 = pins.gpio26.into_push_pull_output();
    let mut led8 = pins.gpio27.into_push_pull_output();
    let mut led9 = pins.gpio28.into_push_pull_output();

    let leds: [&mut dyn embedded_hal::digital::OutputPin<Error = core::convert::Infallible>; 10] = [
        &mut led0, &mut led1, &mut led2, &mut led3, &mut led4, &mut led5, &mut led6, &mut led7,
        &mut led8, &mut led9,
    ];

    loop {
        for i in 0..10 {
            leds[i].set_high().unwrap();
            timer.delay_ms(10);
            leds[i].set_low().unwrap();
            timer.delay_ms(10);
        }
        for i in (0..10).rev() {
            leds[i].set_high().unwrap();
            timer.delay_ms(10);
            leds[i].set_low().unwrap();
            timer.delay_ms(10);
        }
    }
}
