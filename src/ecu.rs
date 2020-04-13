extern crate embedded_hal as hal;
extern crate pwm_pca9685 as pca9685;

use actix::prelude::*;

pub struct ECU<I2C> {
    pwm: pca9685::Pca9685<I2C>,
}


impl<I2C, E> ECU<I2C> where I2C: hal::blocking::i2c::Write<Error = E> + hal::blocking::i2c::WriteRead<Error = E> + std::marker::Unpin + 'static, E: std::fmt::Debug
{
    pub fn new_from_device(dev: I2C) -> ECU<I2C> {
        let slave = pca9685::SlaveAddr::default();
        ECU{pwm: pca9685::Pca9685::new(dev, slave)}
    }

}

pub struct Command(pub String);

impl<I2C, E> Actor for ECU<I2C> where I2C: hal::blocking::i2c::Write<Error = E> + hal::blocking::i2c::WriteRead<Error = E> + std::marker::Unpin + 'static, E: std::fmt::Debug
{
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        // This corresponds to a frequency of 60 Hz.
        self.pwm.set_prescale(100).unwrap();

        // Turn on channel 0 at 0.
        self.pwm.set_channel_on(pca9685::Channel::C0, 0).unwrap();

        // Turn off channel 1 at 2047, which is 50% in
        // the range `[0..4095]`.
        self.pwm.set_channel_off(pca9685::Channel::C1, 2047).unwrap();
    }
}

impl Message for Command {
    type Result = ();
}

impl<I2C, E> Handler<Command> for ECU<I2C> where I2C: hal::blocking::i2c::Write<Error = E> + hal::blocking::i2c::WriteRead<Error = E> + std::marker::Unpin + 'static, E: std::fmt::Debug
{
    type Result = ();

    fn handle(&mut self, msg: Command, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Got {:?}", msg.0);
    }

}

#[cfg(test)]
mod tests {
    extern crate embedded_hal_mock as mock_hal;

    use actix::prelude::*;
    use mock_hal::i2c::{Mock, Transaction};
    use super::*;

    #[test]
    fn test_started() {
         let expectations = [
             Transaction::write(0xaa, vec![1, 2]),
             Transaction::read(0xbb, vec![3, 4]),
         ];

         System::run(move || {
             let mock_dev = Mock::new(&expectations);
             let actor = ECU::new_from_device(mock_dev);
             actor.start();
             System::current().stop();
         }).unwrap();
    }
}
