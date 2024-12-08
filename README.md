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

Another interesting crate is [palette](https://docs.rs/palette/latest/palette/). Not sure if it could be used for embedded systems.

* [srgb v0.3.3](https://crates.io/crates/srgb)
* [blend_srgb v0.1.1](https://docs.rs/blend-srgb/latest/blend_srgb/) 
* [fast-srgb8 v1.0.0](https://crates.io/crates/fast-srgb8)

[Another example for 8x8 ws2812b arrary](https://github.com/9names/rp2040_rust_playground/blob/main/ws2812_8x8/src/main.rs), check it out.

## notes on the crates

* [ws2812-pio](https://crates.io/crates/ws2812-pio): Driver for WS2812 LED using RP2024 PIO peripheral. This crate refers to [rp-hal](https://github.com/rp-rs/rp-hal) for examples.

* [trait smart_leds_trait::SmartLedsWrite](https://docs.rs/smart-leds-trait/0.2.1/smart_leds_trait/trait.SmartLedsWrite.html)

```
fn write<T, I>(&mut self, iterator: T) -> Result<(), Self::Error>
where
    T: Iterator<Item = I>,
    I: Into<Self::Color>, 
```

* [smart-leds](https://github.com/smart-leds-rs/smart-leds) is another crate.

* [smart-led-effects v0.1.7](https://docs.rs/smart_led_effects/latest/smart_led_effects/)

* [spatial LED](https://github.com/davjcosby/sled/) std

* [rust stm32f411](https://github.com/blaz-r/STM32F411-rust-neopixel/)

## snowflake LED numbering

The center is the start, index of 0. Then the inner circle is from 1 to 6.

The 7th goes to the furthest, then goes down its branch. From the outer to the inner, from 7 to 9. Other branches follows the same pattern. 

So the center is LED0. the inner circle is LED1 to LED6.
The furthest circle is LED7, LED10, LED13, LED16, LED19, and LED22.
The mid in branches is LED8, LED11, LED14, LED17, LED20, and LED23.
The nearest in branches is LED9, LED12, LED15, LED18, LED21, and LED24.

| circles | LEDs |
| ------- | ---- |
| 0 (center) | LED0 |
| 1 | LED1 ~ LED6 |
| 2 | LED9, LED12, LED15, LED18, LED21, LED24 |
| 3 | LED8, LED11, LED14, LED17, LED20, LED23 |
| 4 | LED7, LED10, LED13, LED16, LED19, LED22 |

## [non_std](https://docs.rs/crate/non_std/0.1.4)

there is one ```non_std``` crate available.

```
$ cargo add non_std
$ cargo remove non_std
```

## reference;

* [smart_led_effects crate page](https://docs.rs/crate/smart_led_effects/latest)study it here. sRGB vectors for all the pixels.

* [neopixel-ws2812-led-pico-fading-rs](https://github.com/tracyspacy/neopixel-ws2812-led-pico-fading-rs) this rust code is built for rpi-pico

* [st7567 driver in rust](https://github.com/tracyspacy/st7567_rs)

* [rpi-ws2812-rs](https://github.com/bitbrain-za/rpi_ws2812-rs) SPI for LED on RPI
