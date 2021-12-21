# `l298n`

> no_std driver for [L298N](https://www.st.com/resource/en/datasheet/l298.pdf) (Dual H-Bridge Motor Controller module)

[![Build Status](https://github.com/lucazulian/l298n/workflows/l298n-ci/badge.svg)](https://github.com/lucazulian/l298n/actions?query=workflow%3Al298n-ci)
[![crates.io](https://img.shields.io/crates/v/l298n.svg)](https://crates.io/crates/l298n)
[![Docs](https://docs.rs/l298n/badge.svg)](https://docs.rs/l298n)
[![Coverage Status](https://coveralls.io/repos/github/lucazulian/l298n/badge.svg?branch=master)](https://coveralls.io/github/lucazulian/l298n?branch=master)


## Basic usage

Include this [library](https://crates.io/crates/l298n) as a dependency in your `Cargo.toml`:

```rust
[dependencies.l298n]
version = "<version>"
```
Use [embedded-hal](https://github.com/rust-embedded/embedded-hal) implementation to get PINA, PINB and PWM and then create a single motor:

```rust
extern crate l298n;

let motor = l298n::Motor::new(PINA, PINB, PWM);

motor.set_duty(12);

motor.brake();

```
or a l298 dual bridge motor:
```rust
extern crate l298n;

let motor = l298n::L298N::new(PIN1A, PIN2A, PWM1B, IN2B, PIN1B, PWMB);

motor.a.set_duty(12);

motor.a.brake();

```

## License

[MIT license](http://opensource.org/licenses/MIT)
