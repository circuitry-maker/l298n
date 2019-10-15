#![deny(missing_docs)]
#![deny(warnings)]
#![allow(dead_code)]
#![no_std]

//! Constructs a new
//!
//! 

use embedded_hal::digital::v2::OutputPin;
use embedded_hal::PwmPin;

/// dummy
pub struct Motor<IN1, IN2, PWM>
where
    IN1: OutputPin,
    IN2: OutputPin,
    PWM: PwmPin,
{
    in1: IN1,
    in2: IN2,
    pwm: PWM,
}


impl<IN1, IN2, PWM> Motor<IN1, IN2, PWM>
where
    IN1: OutputPin,
    IN2: OutputPin,
    PWM: PwmPin,
{
    /// dummy
    pub fn new(in1: IN1, in2: IN2, pwm:PWM) -> Motor<IN1, IN2, PWM>
    where
        IN1: OutputPin,
        IN2: OutputPin,
        PWM: PwmPin,
        {

            Motor {
                in1: in1,
                in2: in2,
                pwm: pwm
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
