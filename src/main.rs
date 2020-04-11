extern crate actix;
extern crate linux_embedded_hal as hal;
extern crate pwm_pca9685 as pca9685;

use actix::prelude::*;

mod ecu;

fn main() {
    let system = System::new("test");

    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let slave = pca9685::SlaveAddr::default();
    let addr = ecu::ECU{pwm: pca9685::Pca9685::new(dev, slave)}.start();

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
