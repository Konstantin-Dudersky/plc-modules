use std::time::Duration;

use esp_idf_svc::hal::{gpio::AnyIOPin, peripheral::Peripheral, uart::Uart};
use rsiot::{
    components::cmp_esp_uart_slave::{self},
    components_config::uart_general::{
        protocol::{Protocol, UartPacket},
        FieldbusRequest,
    },
    executor::Component,
};

use pm_uart_integration::pm_mod__keyboard__v0_0_2::{Request, Response};

use super::message::*;

#[allow(clippy::single_match)]
pub fn cmp<TUart, TPeripheral>(
    address: u8,
    uart: TUart,
    pin_rx: AnyIOPin,
    pin_tx: AnyIOPin,
    pin_rts: AnyIOPin,
) -> Component<cmp_esp_uart_slave::Config<MRoot, TUart, TPeripheral, UartSlaveBuffer>, MRoot>
where
    TUart: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Uart,
{
    let config = cmp_esp_uart_slave::Config {
        uart,
        pin_rx,
        pin_tx,
        pin_rts,
        baudrate: cmp_esp_uart_slave::Baudrate::_115_200,
        data_bits: cmp_esp_uart_slave::DataBits::_8,
        parity: cmp_esp_uart_slave::Parity::None,
        stop_bits: cmp_esp_uart_slave::StopBits::_2,
        buffer_data_default: UartSlaveBuffer {
            protocol: Protocol::new(address),
            ..Default::default()
        },
        fn_input: |msg, buffer| {
            let Some(msg) = msg.get_custom_data() else {
                return;
            };
            match msg {
                MRoot::SpiMaster(msg) => match msg {
                    MSpiMaster::PressedButton(v) => buffer.pressed_button = v,
                    MSpiMaster::PressedTouch(v) => buffer.pressed_touch = v,
                },
            }
        },
        fn_uart_comm: |req: FieldbusRequest, buffer: &mut UartSlaveBuffer| {
            let packet: UartPacket<Request> = buffer.protocol.deserialize_request(req)?;
            if packet.address != buffer.protocol.address {
                return Ok(None);
            }
            let response = match packet.data {
                Request::GetData => Response::Data {
                    pressed_button: buffer.pressed_button,
                    pressed_touch: buffer.pressed_touch,
                },
            };
            let response = buffer.protocol.serialize_response(response)?;
            Ok(Some(response))
        },
        fn_output: |_buffer| vec![],
        fn_output_period: Duration::from_millis(100),
    };

    cmp_esp_uart_slave::Cmp::new(config)
}

#[derive(Clone, Debug, Default)]
pub struct UartSlaveBuffer {
    pub protocol: Protocol,
    pub pressed_button: Option<(u8, u8)>,
    pub pressed_touch: Option<(u32, u32)>,
}
