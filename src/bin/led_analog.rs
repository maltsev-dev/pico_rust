#![no_std]
#![no_main]

use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin};
use embedded_hal::pwm::SetDutyCycle;
use hal::pac;
use panic_halt as _;
use rp2040_hal as hal;
use rp2040_hal::pwm::Slice;
use rp2040_hal::{
    prelude::*,
    pwm::{InputHighRunning, Slices},
};

#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

// fn reverse_led(led: &mut (impl OutputPin + StatefulOutputPin)) {
//     if led.is_set_high().unwrap() {
//         led.set_low().unwrap();
//     } else {
//         led.set_high().unwrap();
//     }
// }

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

    // use rp2040_hal::{
    //     prelude::*,
    //     pwm::{InputHighRunning, Slices},
    // };

    let mut timer = rp2040_hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Init PWMs
    let pwm_slices = Slices::new(pac.PWM, &mut pac.RESETS);

    // Configure PWM7
    let mut pwm = pwm_slices.pwm7;
    pwm.set_ph_correct();
    pwm.enable();

    // Set the PWM frequency to 10kHz
    let system_freq = clocks.system_clock.freq(); // in Hz
    let pwm_freq = 10_000u32; // 10kHz
    let divider = 1u8; // default divider

    let top = (system_freq / (pwm_freq as u32 * divider as u32)) - 1;

    pwm.set_top(top as u16);
    pwm.set_div_int(divider);

    // Connect GPIO15 to PWM7 B channel
    let _channel_b = pwm.channel_b.output_to(pins.gpio15);

    // Set duty cycle ( 50%)
    pwm.channel_b.set_duty(top as u16 / 2);

    loop {}
}
