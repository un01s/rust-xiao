//! # 1st example code GPIO Blink
//! # 2nd LED fading effect 

#![no_std]
#![no_main]

use bsp::entry;
use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    pio::PIOExt,
    sio::Sio,
    timer::Timer,
    watchdog::Watchdog,
};
use defmt::*;
use defmt_rtt as _;
use micromath::F32Ext;
use panic_probe as _;
use rp_pico as bsp;
use smart_leds::{brightness, SmartLedsWrite, RGB8};
use ws2812_pio::Ws2812;

fn pick_color(color_index: usize, step: usize, list_len: usize) -> usize {
    if step > 0 && step % COLOR_CHANGE_INTERVAL == 0 {
        return (color_index + 1) % list_len;
    }
    return color_index;
}

//The brightness is calculated using a sine wave equation
// Max Brightness is 255, Min is 55
//https://www.desmos.com/calculator/mrsdidnqhu
fn calculate_brightness(step: usize) -> f32 {
    100.0 * (2.0 * PI * step as f32 / BRIGHTNESS_CYCLE + PI / 2.0).sin() + 155.0
}

pub const PI: f32 = 3.14159274f32;
pub const NUM_PIXELS: usize = 25; // Number of pixels in your LED strip
pub const BRIGHTNESS_CYCLE: f32 = 160.0; // sine wave cycle, for ex from crest to the next crest
pub const COLOR_CHANGE_INTERVAL: usize = 800; // 5 complete cycles starting from crest

#[entry]
fn main() -> ! {
    info!("Program start");
    let core = pac::CorePeripherals::take().unwrap();
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let mut ws = Ws2812::new(
        pins.gpio27.into_function(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    let colors_array: [RGB8; 6] = [
        (250, 0, 10).into(),
        (84, 3, 117).into(),
        (20, 111, 210).into(),
        (68, 218, 0).into(),
        (219, 224, 0).into(),
        (0, 250, 15).into(),
    ];
    let mut counter: usize = 0;
    let mut color_index = 0;
    let mut colors = [RGB8::default(); NUM_PIXELS];
    for i in 0..NUM_PIXELS {
        colors[i] = colors_array[0];
        ws.write(brightness(
            colors.iter().copied(),
            calculate_brightness(counter) as u8,
        ))
        .unwrap();
        delay.delay_ms(10);
    }

    loop {
        for j in 0..NUM_PIXELS {
            // Set the color for the current LED
            colors[j] = colors_array[color_index];
        }
        // Write the current state of the colors array and brightness level to the LED strip
        ws.write(brightness(
            colors.iter().copied(),
            calculate_brightness(counter) as u8,
        ))
        .unwrap();

        delay.delay_ms(50);
        //get color for a cycle
        color_index = pick_color(color_index, counter, colors_array.len());
        counter += 1;
    }
}
