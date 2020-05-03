extern crate log;

extern crate actix;
extern crate actix_web;
extern crate linux_embedded_hal as linux_hal;
extern crate gstreamer as gst;

extern crate embedded_hal_mock as mock_hal;
use mock_hal::i2c::{Mock, Transaction};

use actix::prelude::*;


mod ecu;
mod web;


fn main() {
    env_logger::init();

    gst::init().unwrap();

    let system = System::new("test");

    let expectations = [
        Transaction::write(0x40, vec![254, 100]),
        Transaction::write(0x40, vec![0, 49]),
        Transaction::write(0x40, vec![6, 0, 0]),
        Transaction::write(0x40, vec![12, 255, 7]),
    ];
    let dev = Mock::new(&expectations);
    //let dev = linux_hal::I2cdev::new("/dev/i2c-1").unwrap();
    let addr = ecu::ECU::new_from_device(dev).start();

    web::start(addr);

    system.run().unwrap();
}
