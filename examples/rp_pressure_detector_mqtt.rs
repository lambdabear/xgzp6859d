use librumqttd::{Broker, Config};
use rppal::{hal::Delay, i2c::I2c};
use std::error::Error;
use std::thread;
use std::time::Duration;

use xgzp6859d::Xgzp6859d;

const DELAY_MS: u64 = 100;

fn main() -> Result<(), Box<dyn Error>> {
    let i2c = I2c::new()?;
    let delay = Delay::new();
    let mut sensor = Xgzp6859d::new(i2c, delay)?;

    let config: Config = confy::load_path("/home/pi/rumqttd.conf").unwrap();
    let mut broker = Broker::new(config);
    let mut tx = broker
        .link("localclient")
        .expect("Local publisher link broker failed");

    thread::spawn(move || broker.start().expect("Broker start failed"));

    let mut _rx = tx.connect(200).expect("Connect broker failed");
    tx.subscribe("#").unwrap();

    loop {
        let pressure = sensor.read_pressure()?;
        // println!("Pressure: {}", pressure);

        tx.publish("pressure/data", false, pressure.to_le_bytes())
            .expect("Publish failed");

        thread::sleep(Duration::from_millis(DELAY_MS));
    }
}
