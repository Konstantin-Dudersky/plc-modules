use esp_idf_svc::hal::{gpio::AnyIOPin, peripheral::Peripheral, spi::Spi, units::FromValueType};
use rsiot::{components::cmp_esp_spi_master, message::Message};

use pm_cnv::pm_cnv__di16_sink__v0_0_4::Device;

use crate::settings::SPI_BAUDRATE;

use super::message::Custom;

pub fn config<TSpi, TPeripheral>(
    spi: TSpi,
    pin_mosi: AnyIOPin,
    pin_miso: AnyIOPin,
    pin_sck: AnyIOPin,
    pin_cs: AnyIOPin,
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
        devices_comm_settings: vec![cmp_esp_spi_master::ConfigDevicesCommSettings {
            pin_cs,
            baudrate: SPI_BAUDRATE,
            spi_mode: cmp_esp_spi_master::ConfigDeviceSpiMode::Mode0,
        }],
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
