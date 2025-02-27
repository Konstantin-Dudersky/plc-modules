use esp_idf_svc::hal::{gpio::AnyIOPin, peripheral::Peripheral, spi::Spi};
use pm_mod_firmware::settings::SPI_BAUDRATE;
use rsiot::components::cmp_esp_spi_master;

use pm_cnv::pm_cnv__rq8__v0_0_5::Device;

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
            fn_input: |msg, buffer| {
                let Some(msg) = msg.get_custom_data() else {
                    return;
                };
                match msg {
                    Custom::SetOutputs(outputs) => buffer.outputs = outputs,
                    Custom::ConnectionState(state) if !state => {
                        buffer.outputs = 0x0;
                    }
                    _ => (),
                }
            },
        })],
    }
}
