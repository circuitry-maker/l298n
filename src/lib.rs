#![no_std]

//! Manages a new L298N a Dual H-Bridge Motor Controller module

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    warnings
)]
#![allow(dead_code)]


use embedded_hal::digital::v2::OutputPin;
use embedded_hal::PwmPin;

/// Struct for L298N. Two enable inputs are provided to enable or disable the device
/// independently of the input signals. The emitters of the lower transistors of each
/// bridge are connected together and the corresponding external terminal can be used
/// for the connection of an external sensing resistor. An additional supply input is
/// provided so that the logic works at a lower voltage.
#[derive(Debug)]
pub struct L298N<IN, PWM>
where
    IN: OutputPin,
    PWM: PwmPin,
{
    a: Motor<IN, PWM>,
    b: Motor<IN, PWM>,
}

impl<IN, PWM> L298N<IN, PWM>
where
    IN: OutputPin,
    PWM: PwmPin,
{
    /// Creates a new `L298N` motor controller
    pub fn new(ina1: IN, ina2: IN, pwma:PWM, inb1: IN, inb2: IN, pwmb:PWM) -> L298N<IN, PWM>
    where
        IN: OutputPin,
        PWM: PwmPin,
        {
            L298N {
                a: Motor::new(ina1, ina2, pwma),
                b: Motor::new(inb1, inb2, pwmb)
            }
    }
}

/// Struct for single bridge
#[derive(Debug)]
pub struct Motor<IN, PWM>
where
    IN: OutputPin,
    PWM: PwmPin,
{
    in1: IN,
    in2: IN,
    pwm: PWM,
}

impl<IN, PWM> Motor<IN, PWM>
where
    IN: OutputPin,
    PWM: PwmPin,
{
    /// Creates a new single `Motor` controller
    pub fn new(in1: IN, in2: IN, pwm:PWM) -> Motor<IN, PWM>
    where
        IN: OutputPin,
        PWM: PwmPin,
        {
            Motor {
                in1,
                in2,
                pwm
            }
    }

    /// Brakes the motor - Fast Motor Stop
    /// with Ven = H then C = D Fast Motor Stop
    pub fn brake(&mut self) -> &mut Self {
        self.in1.set_high().ok();
        self.in2.set_high().ok();
        self
    }

    /// Stops the motor - Free Running Motor Stop
    /// Ven = L then with C = X ; D = X
    pub fn stop(&mut self) -> &mut Self {
        self.in1.set_high().ok();
        self.in2.set_high().ok();
        self
    }

    /// Makes the motor forward direction
    /// with Ven = H then C = H ; D = L Forward
    pub fn forward(&mut self) -> &mut Self {
        self.in1.set_low().ok();
        self.in2.set_high().ok();
        self
    }

    /// Makes the motor reverse direction
    /// with Ven = H then C = L ; D = H Reverse
    pub fn reverse(&mut self) -> &mut Self {
        self.in1.set_high().ok();
        self.in2.set_low().ok();
        self
    }

    /// Returns the maximum
    pub fn get_max_duty(&mut self) -> PWM::Duty {
        self.pwm.get_max_duty()
    }

    /// Changes the motor speed
    pub fn set_duty(&mut self, duty: PWM::Duty) -> &mut Self {
        self.pwm.set_duty(duty);
        self
    }

    /// Get the actual motor speed
    pub fn get_current_duty(&mut self) -> PWM::Duty {
        self.pwm.get_duty()
    }
}
