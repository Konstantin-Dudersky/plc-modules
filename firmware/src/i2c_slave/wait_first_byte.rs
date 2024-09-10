use esp_idf_svc::hal::{delay::BLOCK, i2c::I2cSlaveDriver};

use crate::error::Error;

pub fn wait_first_byte<'a>(i2c_slave_driver: &mut I2cSlaveDriver<'a>) -> Result<u8, Error> {
    let mut request_byte: [u8; 1] = [0];
    i2c_slave_driver
        .read(&mut request_byte, BLOCK)
        .map_err(Error::I2cSlave)?;
    Ok(request_byte[0])
}
