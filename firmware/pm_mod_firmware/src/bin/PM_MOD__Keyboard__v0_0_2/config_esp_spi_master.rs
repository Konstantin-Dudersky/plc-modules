use std::time::Duration;

use esp_idf_svc::hal::{gpio::AnyIOPin, peripheral::Peripheral, spi::Spi};
use rsiot::{components::cmp_esp_spi_master, message::Message};

use pm_cnv::pm_cnv__keyboard__v0_0_2;
use pm_mod_firmware::settings::SPI_BAUDRATE;
use rsiot_devices::spi::xpt2046;

use super::message::*;

pub fn config<TSpi, TPeripheral>(
    spi: TSpi,
    pin_mosi: AnyIOPin,
    pin_miso: AnyIOPin,
    pin_sck: AnyIOPin,
    pin_cs0: AnyIOPin,
    pin_cs1: AnyIOPin,
) -> cmp_esp_spi_master::Config<MRoot, TSpi, TPeripheral>
where
    TSpi: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Spi,
{
    let keyboard_config = cmp_esp_spi_master::ConfigDevicesCommSettings {
        pin_cs: pin_cs0,
        baudrate: SPI_BAUDRATE,
        spi_mode: cmp_esp_spi_master::ConfigDeviceSpiMode::Mode0,
    };
    let keyboard_device = pm_cnv__keyboard__v0_0_2::Device {
        fn_output: |buffer| {
            let msg = match buffer.pressed_button {
                Some(v) => MSpiMaster::PressedButton(Some((v.0, v.1))),
                None => MSpiMaster::PressedButton(None),
            };
            buffer.pressed_button = None;

            let msg = Message::new_custom(MRoot::SpiMaster(msg));

            vec![msg]
        },
    };

    let touch_config = cmp_esp_spi_master::ConfigDevicesCommSettings {
        pin_cs: pin_cs1,
        baudrate: SPI_BAUDRATE,
        spi_mode: cmp_esp_spi_master::ConfigDeviceSpiMode::Mode0,
    };

    let touch_device = xpt2046::Device {
        request_period: Duration::from_millis(20),
        fn_output: |buffer| {
            let msg = if buffer.x == 128 && buffer.y == 248 {
                MSpiMaster::PressedTouch(None)
            } else {
                MSpiMaster::PressedTouch(Some((buffer.x, buffer.y)))
            };
            vec![Message::new_custom(MRoot::SpiMaster(msg))]
        },
    };

    cmp_esp_spi_master::Config {
        spi,
        pin_miso,
        pin_mosi,
        pin_sck,
        devices_comm_settings: vec![keyboard_config, touch_config],
        devices: vec![Box::new(keyboard_device), Box::new(touch_device)],
    }
}
