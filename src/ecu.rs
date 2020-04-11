use actix::prelude::*;

pub struct ECU;

impl Actor for ECU {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
        let address = pca9685::SlaveAddr::default();
        let mut pwm = pca9685::Pca9685::new(dev, address);
        // This corresponds to a frequency of 60 Hz.
        pwm.set_prescale(100).unwrap();

        // Turn on channel 0 at 0.
        pwm.set_channel_on(pca9685::Channel::C0, 0).unwrap();

        // Turn off channel 1 at 2047, which is 50% in
        // the range `[0..4095]`.
        pwm.set_channel_off(pca9685::Channel::C1, 2047).unwrap();
    }
}
