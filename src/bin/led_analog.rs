#![no_std]
#![no_main]

use embedded_hal::{delay::DelayNs, pwm::SetDutyCycle};
use hal::pac;
use panic_halt as _;
use rp2040_hal::{self as hal, pwm::Slices};

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

    // PWM Slices Init
    let pwm_slices = Slices::new(pac.PWM, &mut pac.RESETS);
    // Get 7th slice (controls GPIO 14 (A) and GPIO 15 (B))
    let mut pwm_slice = pwm_slices.pwm7;

    pwm_slice.set_ph_correct(); // Phase correct mode = smoother signal
    pwm_slice.set_top(255); // 8-bit duty cycle precision
    pwm_slice.enable(); // enable slice

    // Get channel_b from 7th slice
    let mut channel_b = pwm_slice.channel_b;
    // Make channel_b output to 15th pin
    let _pin = channel_b.output_to(pins.gpio15);

    let mut duty: i16 = 0;
    let mut step: i16 = 1;

    loop {
        let _ = channel_b.set_duty_cycle(duty as u16);
        timer.delay_us(1000);

        duty += step;
        if duty >= 255 || duty <= 0 {
            step = -step;
        }
    }
}
