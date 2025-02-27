use esp_idf_svc::hal::{gpio::AnyIOPin, peripheral::Peripheral, spi::Spi};
use rsiot::components::cmp_esp_spi_master;

use pm_cnv::pm_cnv__ai4_w__v0_0_1::Device;

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
            spi_mode: cmp_esp_spi_master::ConfigDeviceSpiMode::Mode2,
        }],
        devices: vec![Box::new(Device {
            address: 0,
            fn_input: |_msg, _buffer| {},
        })],
    }
}
