extern crate actix;
extern crate linux_embedded_hal as linux_hal;

use actix::prelude::*;

mod ecu;

fn main() {
    let system = System::new("test");

    let dev = linux_hal::I2cdev::new("/dev/i2c-1").unwrap();
    let addr = ecu::ECU::new_from_device(dev).start();

    Arbiter::spawn(async {
        std::thread::spawn(move || loop {
            let mut cmd = String::new();
            if std::io::stdin().read_line(&mut cmd).is_err() {
                println!("error");
                return;
            }
            addr.do_send(ecu::Command(cmd));
        });
    });

    system.run().unwrap();
}
