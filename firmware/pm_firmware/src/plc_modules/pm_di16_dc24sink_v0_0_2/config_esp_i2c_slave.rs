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
        fn_input,
        fn_output,
        fn_output_period: Duration::from_millis(100),
        fn_i2c_comm,
        buffer_data_default: I2cBuffer::default(),
        start_delay: Duration::from_millis(2000),
    }
}

fn fn_input(msg: &Message<Custom>, buffer: &mut I2cBuffer) {
    let Some(msg) = msg.get_custom_data() else {
        return;
    };
    match msg {
        Custom::InputsState { a, b } => {
            buffer.input_state_a = a;
            buffer.input_state_b = b;
        }
        Custom::GetInput => (),
        Custom::GpioExpanderInt(_) => (),
        Custom::LedState { a: _, b: _ } => (),
    }
}

fn fn_output(_buffer: &I2cBuffer) -> Vec<Message<Custom>> {
    vec![]
}

fn fn_i2c_comm(req: I2cRequest, buffer: &mut I2cBuffer) -> Result<I2cResponse, anyhow::Error> {
    let response = match req {
        I2cRequest::GetInput => I2cResponse::InputsState {
            a0: (buffer.input_state_a >> 0 & 1) == 1,
            a1: (buffer.input_state_a >> 1 & 1) == 1,
            a2: (buffer.input_state_a >> 2 & 1) == 1,
            a3: (buffer.input_state_a >> 3 & 1) == 1,
            a4: (buffer.input_state_a >> 4 & 1) == 1,
            a5: (buffer.input_state_a >> 5 & 1) == 1,
            a6: (buffer.input_state_a >> 6 & 1) == 1,
            a7: (buffer.input_state_a >> 7 & 1) == 1,
            b0: (buffer.input_state_b >> 0 & 1) == 1,
            b1: (buffer.input_state_b >> 1 & 1) == 1,
            b2: (buffer.input_state_b >> 2 & 1) == 1,
            b3: (buffer.input_state_b >> 3 & 1) == 1,
            b4: (buffer.input_state_b >> 4 & 1) == 1,
            b5: (buffer.input_state_b >> 5 & 1) == 1,
            b6: (buffer.input_state_b >> 6 & 1) == 1,
            b7: (buffer.input_state_b >> 7 & 1) == 1,
        },
    };
    Ok(response)
}

#[derive(Clone, Debug, Default)]
pub struct I2cBuffer {
    pub input_state_a: u8,
    pub input_state_b: u8,
}

impl BufferData for I2cBuffer {}
