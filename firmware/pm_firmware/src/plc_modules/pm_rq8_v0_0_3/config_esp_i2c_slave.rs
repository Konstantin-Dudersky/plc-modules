use std::time::Duration;

use esp_idf_svc::hal::{gpio::AnyIOPin, i2c::I2c, peripheral::Peripheral};
use rsiot::{
    components::cmp_esp_i2c_slave::{self, BufferData},
    message::Message,
};

use super::{Custom, I2cRequest, I2cResponse};

pub fn config<TI2c, TPeripheral>(
    i2c: TI2c,
    pin_sda: AnyIOPin,
    pin_scl: AnyIOPin,
    slave_address: u8,
) -> cmp_esp_i2c_slave::Config<Custom, TI2c, TPeripheral, I2cRequest, I2cResponse, I2cBuffer>
where
    TI2c: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: I2c,
{
    cmp_esp_i2c_slave::Config {
        i2c,
        sda: pin_sda,
        scl: pin_scl,
        slave_address,
        fn_input: |_, _| (),
        fn_output: |buffer: &I2cBuffer| vec![Message::new_custom(Custom::SetOutput(buffer.output))],
        fn_output_period: Duration::from_millis(100),
        fn_i2c_comm: |req: I2cRequest, buffer: &mut I2cBuffer| match req {
            I2cRequest::SetOutputs(data) => {
                buffer.output = data;
                Ok(I2cResponse::Ok(123))
            }
        },
        buffer_data_default: I2cBuffer { output: 0x00 },
        start_delay: Duration::from_millis(2000),
    }
}

#[derive(Clone, Debug, Default)]
pub struct I2cBuffer {
    pub output: u8,
}

impl BufferData for I2cBuffer {}
