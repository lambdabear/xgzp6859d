use embedded_hal::{
    delay::blocking::DelayMs,
    i2c::blocking::{Write, WriteRead},
};

const ADDR: u8 = 0x6D;
const K: u8 = 64;
const SYS_CONFIG_REG: u8 = 0xA5;
const CMD_REG: u8 = 0x30;
const DATA_MSB_REG: u8 = 0x06;
const DATA_CSB_REG: u8 = 0x07;
const DATA_LSB_REG: u8 = 0x08;
const MEASURE_PRESSURE_CMD: u8 = 0x0A;

pub struct Xgzp6859d<I2C, DELAY> {
    i2c: I2C,
    delay: DELAY,
}

impl<I2C, DELAY, E: core::fmt::Debug> Xgzp6859d<I2C, DELAY>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
    DELAY: DelayMs<u8>,
{
    pub fn new(mut i2c: I2C, delay: DELAY) -> Result<Self, E> {
        let mut sys_config = [0];
        i2c.write_read(ADDR, &[SYS_CONFIG_REG], &mut sys_config)?;
        let config = sys_config[0] & 0b11111101;
        i2c.write(ADDR, &[SYS_CONFIG_REG, config])?;

        Ok(Self { i2c, delay })
    }

    pub fn read_pressure(&mut self) -> Result<i32, E> {
        self.i2c.write(ADDR, &[CMD_REG, MEASURE_PRESSURE_CMD])?;

        let mut buffer = [0_u8];

        self.i2c.write_read(ADDR, &[CMD_REG], &mut buffer)?;
        if buffer[0] & 0b00001000 != 0 {
            self.delay.delay_ms(12).ok();
        }

        self.i2c.write_read(ADDR, &[DATA_MSB_REG], &mut buffer)?;
        let data_msb = buffer[0];
        self.i2c.write_read(ADDR, &[DATA_CSB_REG], &mut buffer)?;
        let data_csb = buffer[0];
        self.i2c.write_read(ADDR, &[DATA_LSB_REG], &mut buffer)?;
        let data_lsb = buffer[0];

        let adc = data_msb as i32 * 65536 + data_csb as i32 * 256 + data_lsb as i32;

        if data_msb < 0x80 {
            Ok(adc / K as i32)
        } else {
            Ok((adc - 16777216) / K as i32)
        }
    }
}
