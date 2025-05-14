use esp_idf_svc::hal::{gpio::AnyIOPin, peripheral::Peripheral, spi::Spi};
use rsiot::{components::cmp_esp_spi_master, message::Message};

use pm_cnv::{chips::ad7190::SRChannel, pm_cnv__ai4_w__v0_0_1::Device};

use pm_mod_firmware::settings::SPI_BAUDRATE;
use tracing::info;

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
    let adc1_comm = cmp_esp_spi_master::ConfigDevicesCommSettings {
        pin_cs: pin_cs0,
        baudrate: 1_000_000,
        spi_mode: cmp_esp_spi_master::ConfigDeviceSpiMode::Mode2,
    };
    let adc2_comm = cmp_esp_spi_master::ConfigDevicesCommSettings {
        pin_cs: pin_cs1,
        baudrate: 100_000,
        spi_mode: cmp_esp_spi_master::ConfigDeviceSpiMode::Mode2,
    };

    let adc1 = Device {
        fn_input: |_msg, _buffer| {},
        fn_output: |buffer| {
            let ch_0 = *buffer
                .read_registers
                .data_registers
                .get(&SRChannel::Ain1Ain2)
                .unwrap_or(&0.0);
            let ch_0 = Message::new_custom(MRoot::SpiMaster(MSpiMaster::ValueCh0(ch_0)));

            let ch_1 = *buffer
                .read_registers
                .data_registers
                .get(&SRChannel::Ain3Ain4)
                .unwrap_or(&0.0);
            let ch_1 = Message::new_custom(MRoot::SpiMaster(MSpiMaster::ValueCh1(ch_1)));

            vec![ch_0, ch_1]
        },
    };

    let adc2 = Device {
        fn_input: |_msg, _buffer| {},
        fn_output: |buffer| {
            let ch_2 = *buffer
                .read_registers
                .data_registers
                .get(&SRChannel::Ain1Ain2)
                .unwrap_or(&0.0);

            let ch_2 = Message::new_custom(MRoot::SpiMaster(MSpiMaster::ValueCh2(ch_2)));

            let ch_3 = *buffer
                .read_registers
                .data_registers
                .get(&SRChannel::Ain3Ain4)
                .unwrap_or(&0.0);
            let ch_3 = Message::new_custom(MRoot::SpiMaster(MSpiMaster::ValueCh3(ch_3)));

            vec![ch_2, ch_3]
        },
    };

    cmp_esp_spi_master::Config {
        spi,
        pin_miso,
        pin_mosi,
        pin_sck,
        devices_comm_settings: vec![adc1_comm, adc2_comm],
        devices: vec![
            Box::new(adc1),
            // Box::new(adc2)
        ],
    }
}
