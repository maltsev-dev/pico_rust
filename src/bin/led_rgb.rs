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

    let pwm_slices: Slices = Slices::new(pac.PWM, &mut pac.RESETS);
    let mut slices_tilple = (pwm_slices.pwm5, pwm_slices.pwm6);

    slices_tilple.0.set_ph_correct();
    slices_tilple.0.set_top(255);
    slices_tilple.0.enable();

    slices_tilple.1.set_ph_correct();
    slices_tilple.1.set_top(255);
    slices_tilple.1.enable();

    let mut channel_5b = slices_tilple.0.channel_b;
    let _pin_5b = channel_5b.output_to(pins.gpio11); // GPIO11 (PWM5B)

    let mut channel_6a = slices_tilple.1.channel_a;
    let _pin_6a = channel_6a.output_to(pins.gpio12); // GPIO12 (PWM6A)

    let mut channel_6b = slices_tilple.1.channel_b;
    let _pin_6b = channel_6b.output_to(pins.gpio13); // GPIO13 (PWM6B)

    let mut t = 0u16;

    loop {
        let red = rainbow_wave(t.wrapping_add(0));
        let green = rainbow_wave(t.wrapping_add(85));
        let blue = rainbow_wave(t.wrapping_add(170));
        let _ = channel_5b.set_duty_cycle(red);
        let _ = channel_6a.set_duty_cycle(green);
        let _ = channel_6b.set_duty_cycle(blue);

        t = t.wrapping_add(1);
        timer.delay_ms(20);
    }

    fn rainbow_wave(t: u16) -> u16 {
        let t = t % 255;
        if t < 85 {
            (t * 3) as u16
        } else if t < 170 {
            (255 - (t - 85) * 3) as u16
        } else {
            ((t - 170) * 3) as u16
        }
    }
}
