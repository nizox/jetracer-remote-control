extern crate actix;
extern crate linux_embedded_hal as hal;
extern crate pwm_pca9685 as pca9685;

use actix::prelude::*;

mod ecu;

fn main() {
    let system = System::new("test");
    let addr = ecu::ECU.start();
    system.run();
}
