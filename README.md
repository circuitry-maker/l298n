# `l298n`

> no_std driver for [L298N](https://www.st.com/resource/en/datasheet/l298.pdf) (Dual H-Bridge Motor Controller module)

[![Build Status](https://travis-ci.org/lucazulian/l298n.svg?branch=master)](https://travis-ci.org/lucazulian/l298n)
[![crates.io](http://meritbadge.herokuapp.com/l298n?style=flat-square)](https://crates.io/crates/l298n)

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



## Documentation

API Docs available on [docs.rs](https://docs.rs/l298n/0.1.0/l298n/)

## License

[MIT license](http://opensource.org/licenses/MIT)
