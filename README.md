# rust and XIAO RP2040

## setup

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

## [fading effect](https://github.com/tracyspacy/neopixel-ws2812-led-pico-fading-rs)

the code has one main color for all LEDs. This color will fade. Then it changes to another color, and fade again.

## reference;

* [neopixel-ws2812-led-pico-fading-rs](https://github.com/tracyspacy/neopixel-ws2812-led-pico-fading-rs) this rust code is built for rpi-pico

