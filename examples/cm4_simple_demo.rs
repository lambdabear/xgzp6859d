use rppal::{hal::Delay, i2c::I2c};
use std::error::Error;

use xgzp6859d::Xgzp6859d;

fn main() -> Result<(), Box<dyn Error>> {
    let i2c = I2c::new()?;
    let delay = Delay::new();
    let mut sensor = Xgzp6859d::new(i2c, delay)?;

    for _ in 0..10000 {
        let pressure = sensor.read_pressure()?;

        println!("Pressure: {}", pressure);
    }

    Ok(())
}
