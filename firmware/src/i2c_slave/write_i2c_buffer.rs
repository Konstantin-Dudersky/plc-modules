use esp_idf_svc::hal::{delay::TickType, i2c::I2cSlaveDriver};

use crate::error::Error;

pub fn write_i2c_buffer<'a>(
    i2c_slave_driver: &mut I2cSlaveDriver<'a>,
    buffer: &[u8],
) -> Result<(), Error> {
    // Запись в буфер отправки I2C
    let timeout = TickType::new_millis(5000).ticks();
    i2c_slave_driver
        .write(&buffer, timeout)
        .map_err(Error::I2cSlave)?;

    Ok(())
}
