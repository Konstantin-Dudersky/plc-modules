use std::time::Duration;

use esp_idf_svc::hal::{gpio::AnyIOPin, peripheral::Peripheral, uart::Uart};
use rsiot::{
    components::cmp_esp_uart_slave::{self},
    message::Message,
};

use super::{Custom, UartRequest, UartResponse};

pub fn config<TUart, TPeripheral, const MESSAGE_LEN: usize>(
    address: u8,
    uart: TUart,
    pin_rx: AnyIOPin,
    pin_tx: AnyIOPin,
    pin_rts: AnyIOPin,
) -> cmp_esp_uart_slave::Config<
    Custom,
    TUart,
    TPeripheral,
    UartRequest,
    UartResponse,
    UartSlaveBuffer,
    MESSAGE_LEN,
>
where
    TUart: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Uart,
{
    cmp_esp_uart_slave::Config {
        address,
        uart,
        pin_rx,
        pin_tx,
        pin_rts,
        baudrate: cmp_esp_uart_slave::Baudrate::_115_200,
        data_bits: cmp_esp_uart_slave::DataBits::_8,
        parity: cmp_esp_uart_slave::Parity::None,
        stop_bits: cmp_esp_uart_slave::StopBits::_2,
        buffer_data_default: UartSlaveBuffer::default(),
        fn_input: |_, _| (),
        fn_uart_comm: |req: UartRequest, buffer: &mut UartSlaveBuffer| match req {
            UartRequest::SetOutputs(data) => {
                buffer.output = data;
                Ok(UartResponse::Ok)
            }
        },
        fn_output: |buffer: &UartSlaveBuffer| {
            vec![Message::new_custom(Custom::SetOutput(buffer.output))]
        },
        fn_output_period: Duration::from_millis(100),
    }
}

#[derive(Clone, Debug, Default)]
pub struct UartSlaveBuffer {
    pub output: u8,
}
