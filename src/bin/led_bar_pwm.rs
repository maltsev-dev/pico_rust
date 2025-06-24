#![no_std]
#![no_main]

use core::slice;

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

    let mut slices_tilple = (
        pwm_slices.pwm0,
        pwm_slices.pwm1,
        pwm_slices.pwm2,
        pwm_slices.pwm3,
        pwm_slices.pwm5,
        pwm_slices.pwm6,
    );

    slices_tilple.0.set_ph_correct();
    slices_tilple.0.set_top(255);
    slices_tilple.0.enable();

    slices_tilple.1.set_ph_correct();
    slices_tilple.1.set_top(255);
    slices_tilple.1.enable();

    slices_tilple.2.set_ph_correct();
    slices_tilple.2.set_top(255);
    slices_tilple.2.enable();

    slices_tilple.3.set_ph_correct();
    slices_tilple.3.set_top(255);
    slices_tilple.3.enable();

    slices_tilple.4.set_ph_correct();
    slices_tilple.4.set_top(255);
    slices_tilple.4.enable();

    slices_tilple.5.set_ph_correct();
    slices_tilple.5.set_top(255);
    slices_tilple.5.enable();

    let _pin_16 = slices_tilple.0.channel_a.output_to(pins.gpio16);
    let _pin_17 = slices_tilple.0.channel_b.output_to(pins.gpio17);
    let _pin_18 = slices_tilple.1.channel_a.output_to(pins.gpio18);
    let _pin_19 = slices_tilple.1.channel_b.output_to(pins.gpio19);
    let _pin_20 = slices_tilple.2.channel_a.output_to(pins.gpio20);
    let _pin_21 = slices_tilple.2.channel_b.output_to(pins.gpio21);
    let _pin_22 = slices_tilple.3.channel_a.output_to(pins.gpio22);
    let _pin_26 = slices_tilple.4.channel_a.output_to(pins.gpio26);
    let _pin_27 = slices_tilple.4.channel_b.output_to(pins.gpio27);
    let _pin_28 = slices_tilple.5.channel_a.output_to(pins.gpio28);

    let channels: [&mut dyn SetDutyCycle<Error = core::convert::Infallible>; 10] = [
        &mut slices_tilple.0.channel_a,
        &mut slices_tilple.0.channel_b,
        &mut slices_tilple.1.channel_a,
        &mut slices_tilple.1.channel_b,
        &mut slices_tilple.2.channel_a,
        &mut slices_tilple.2.channel_b,
        &mut slices_tilple.3.channel_a,
        &mut slices_tilple.4.channel_a,
        &mut slices_tilple.4.channel_b,
        &mut slices_tilple.5.channel_a,
    ];

    let mut wave: [u16; 30] = [
        0, 0, 10, 30, 50, 70, 90, 120, 145, 180, 215, 255, 215, 180, 145, 120, 90, 70, 50, 30, 10,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    loop {
        for i in 0..10 {
            let value = wave[i];
            let _ = channels[i].set_duty_cycle(value);
        }
        wave.rotate_right(1);
        timer.delay_ms(50);
    }
}
