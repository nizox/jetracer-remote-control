use actix::prelude::*;

pub struct ECU {
    pub pwm: pca9685::Pca9685<hal::I2cdev>,
}

pub struct Command(pub String);

impl Actor for ECU {
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

impl Handler<Command> for ECU {
    type Result = ();

    fn handle(&mut self, msg: Command, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Got {:?}", msg.0);
    }

}
