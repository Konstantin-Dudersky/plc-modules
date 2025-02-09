use esp_idf_svc::hal::{gpio::AnyIOPin, peripheral::Peripheral, spi::Spi, units::FromValueType};
use rsiot::{components::cmp_esp_spi_master, message::Message};

use pm_cnv::PM_CNV__DI16_sink__v0_0_4::Device;

use super::message::Custom;

pub fn config<TSpi, TPeripheral>(
    spi: TSpi,
    pin_mosi: AnyIOPin,
    pin_miso: AnyIOPin,
    pin_sck: AnyIOPin,
    pin_cs_inputs: AnyIOPin,
) -> cmp_esp_spi_master::Config<Custom, TSpi, TPeripheral>
where
    TSpi: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Spi,
{
    cmp_esp_spi_master::Config {
        spi,
        baudrate: 1.MHz().into(),
        pin_miso,
        pin_mosi,
        pin_sck,
        pin_cs: vec![pin_cs_inputs],
        devices: vec![Box::new(Device {
            address: 0,
            fn_output: |buffer| {
                let msgs = [Custom::AllInputs(buffer.all_inputs)];
                let msgs = msgs.iter().map(|m| Message::new_custom(*m)).collect();
                msgs
            },
        })],
    }
}
