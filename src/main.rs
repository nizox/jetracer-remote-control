extern crate actix;
extern crate actix_web;
extern crate linux_embedded_hal as linux_hal;
extern crate gstreamer as gst;

use actix::prelude::*;

mod ecu;
mod web;

fn main() {
    env_logger::init();

    gst::init().unwrap();

    let system = System::new("test");

    let dev = linux_hal::I2cdev::new("/dev/i2c-1").unwrap();
    let addr = ecu::ECU::new_from_device(dev).start();

    web::start(addr);

    system.run().unwrap();
}
