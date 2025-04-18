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

use pm_uart_integration::pm_mod__ai4_w__v_0_0_1::{Request, Response};

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
                    MSpiMaster::ValueCh0(v) => buffer.value_ch0 = v,
                    MSpiMaster::ValueCh1(v) => buffer.value_ch1 = v,
                    MSpiMaster::ValueCh2(v) => buffer.value_ch2 = v,
                    MSpiMaster::ValueCh3(v) => buffer.value_ch3 = v,
                },
                _ => (),
            }
        },
        fn_uart_comm: |req: FieldbusRequest, buffer: &mut UartSlaveBuffer| {
            let packet: UartPacket<Request> = buffer.protocol.deserialize_request(req)?;
            if packet.address != buffer.protocol.address {
                return Ok(None);
            }
            let response = match packet.data {
                Request::GetData => Response::Data {
                    ch0: buffer.value_ch0,
                    ch1: buffer.value_ch1,
                    ch2: buffer.value_ch2,
                    ch3: buffer.value_ch3,
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
    pub value_ch0: f64,
    pub value_ch1: f64,
    pub value_ch2: f64,
    pub value_ch3: f64,
}
