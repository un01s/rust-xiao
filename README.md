# rust and XIAO RP2040

## 1. setup

```
$ cargo new rust-xiao
$ tree -a
.
├── .cargo
│   └── config.toml
├── .git
├── .gitignore
├── Cargo.toml
├── README.md
├── memory.x
└── src
    └── main.rs
```

The code is blinky.rs from [rp2040-hal-examples](https://github.com/rp-rs/rp-hal/tree/main/rp2040-hal-examples). The changes are made to cater XIAO RP2040 board.

Search [crates.io](https://crates.io/) to get the latest version.

## 2. [fading effect](https://github.com/tracyspacy/neopixel-ws2812-led-pico-fading-rs)

A1 is the pin to drive the LEDs data line. It is GPIO27 on XIAO RP2040.

the code has one main color for all LEDs. This color will fade. Then it changes to another color, and fade again.

## 3. [pico WS2812B LED](https://github.com/rp-rs/rp-hal-boards/blob/main/boards/rp-pico/examples/pico_ws2812_led.rs)

This is an example from ```rp-rs/rp-hal-boards/rp-pico```. Use the Cargo.toml from last project of fading LEDs. And add panic-halt as a dependency. Works fine for snowflake which has 25 LEDs. 

## 4. effects

[srgb crate](https://docs.rs/srgb/latest/srgb/) has some example code to use it. Check it out because it is used in ```smart_led_effects```. 

Another interesting crate is [palette](https://docs.rs/palette/latest/palette/). However, it is used for desktop. Not sure if it could be used for embedded systems.

* [srgb v0.3.3](https://crates.io/crates/srgb)
* [blend_srgb v0.1.1](https://docs.rs/blend-srgb/latest/blend_srgb/) 
* [fast-srgb8 v1.0.0](https://crates.io/crates/fast-srgb8)

[Another example for 8x8 ws2812b arrary](https://github.com/9names/rp2040_rust_playground/blob/main/ws2812_8x8/src/main.rs), check it out.

## reference;

* [smart_led_effects crate page](https://docs.rs/crate/smart_led_effects/latest)study it here. sRGB vectors for all the pixels.

* [neopixel-ws2812-led-pico-fading-rs](https://github.com/tracyspacy/neopixel-ws2812-led-pico-fading-rs) this rust code is built for rpi-pico

* [st7567 driver in rust](https://github.com/tracyspacy/st7567_rs)

* [rpi-ws2812-rs](https://github.com/bitbrain-za/rpi_ws2812-rs) SPI for LED on RPI
