use std::time::Duration;

use esp_idf_svc::hal::{gpio::AnyIOPin, peripheral::Peripheral, uart::Uart};
use pm_mod_firmware::settings::{UART_BAUDRATE, UART_DATABITS, UART_PARITY, UART_STOPBITS};
use rsiot::{
    components::cmp_esp_uart_slave::{self},
    components_config::uart_general::{UartRequest, UartResponse},
    message::Message,
};

use pm_uart_integration::pm_rq8_v0_0_5::{Request, Response};

use super::message::Custom;

pub fn config<TUart, TPeripheral>(
    address: u8,
    uart: TUart,
    pin_rx: AnyIOPin,
    pin_tx: AnyIOPin,
    pin_rts: AnyIOPin,
) -> cmp_esp_uart_slave::Config<Custom, TUart, TPeripheral, UartSlaveBuffer>
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
        baudrate: UART_BAUDRATE,
        data_bits: UART_DATABITS,
        parity: UART_PARITY,
        stop_bits: UART_STOPBITS,
        buffer_data_default: UartSlaveBuffer::default(),
        fn_input: |msg, buffer| {
            let Some(msg) = msg.get_custom_data() else {
                return;
            };
            if let Custom::SlaveLiveCounter(live_counter) = msg {
                buffer.slave_live_counter = live_counter
            }
        },
        fn_uart_comm: |mut uart_request: UartRequest, buffer: &mut UartSlaveBuffer| {
            let request: Request = uart_request.get_payload()?;
            let response = match request {
                Request::SetOutputs(data) => {
                    buffer.outputs = data;
                    Response::Ok
                }
                Request::SetMasterLiveCounter(live_counter) => {
                    buffer.master_live_counter = live_counter;
                    Response::Ok
                }
                Request::GetSlaveLiveCounter => {
                    Response::SlaveLiveCounter(buffer.slave_live_counter)
                }
            };
            let uart_response = UartResponse::new(response);
            Ok(uart_response)
        },
        fn_output: |buffer: &UartSlaveBuffer| {
            vec![
                Message::new_custom(Custom::SetOutputs(buffer.outputs)),
                Message::new_custom(Custom::MasterLiveCounter(buffer.master_live_counter)),
            ]
        },
        fn_output_period: Duration::from_millis(100),
    }
}

#[derive(Clone, Debug, Default)]
pub struct UartSlaveBuffer {
    pub outputs: u8,
    pub master_live_counter: u8,
    pub slave_live_counter: u8,
}
