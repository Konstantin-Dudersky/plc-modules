use esp_idf_svc::hal::{gpio::AnyIOPin, peripheral::Peripheral, spi::Spi, units::FromValueType};
use rsiot::components::cmp_esp_spi_master;

use pm_cnv::PM_CNV__RQ8__v0_0_5::Device;

use super::message::Custom;

pub fn config<TSpi, TPeripheral>(
    spi: TSpi,
    pin_mosi: AnyIOPin,
    pin_miso: AnyIOPin,
    pin_sck: AnyIOPin,
    pin_cs0: AnyIOPin,
    pin_cs1: AnyIOPin,
) -> cmp_esp_spi_master::Config<Custom, TSpi, TPeripheral>
where
    TSpi: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Spi,
{
    cmp_esp_spi_master::Config {
        spi,
        pin_miso,
        pin_mosi,
        pin_sck,
        pin_cs: vec![pin_cs0, pin_cs1],
        devices: vec![
            Box::new(Device {
                address: 0,
                fn_input: |msg, buffer| {
                    let Some(msg) = msg.get_custom_data() else {
                        return;
                    };
                    if let Custom::SetOutputs1(outputs) = msg {
                        buffer.outputs = outputs
                    }
                },
            }),
            Box::new(Device {
                address: 1,
                fn_input: |msg, buffer| {
                    let Some(msg) = msg.get_custom_data() else {
                        return;
                    };
                    if let Custom::SetOutputs2(outputs) = msg {
                        buffer.outputs = outputs
                    }
                },
            }),
        ],
        baudrate: 5.MHz().into(),
    }
}
