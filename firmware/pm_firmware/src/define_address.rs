use esp_idf_svc::hal::gpio::{AnyIOPin, Level, PinDriver, Pull};
use tracing::info;

pub fn define_address(
    base_address: u8,
    pin_a0: AnyIOPin,
    pin_a1: AnyIOPin,
    pin_a2: AnyIOPin,
) -> u8 {
    let mut address = base_address;

    let mut pin_a0 = PinDriver::input(pin_a0).unwrap();
    pin_a0.set_pull(Pull::Down).unwrap();

    if let Level::High = pin_a0.get_level() {
        address += 1;
    }

    let mut pin_a1 = PinDriver::input(pin_a1).unwrap();
    pin_a1.set_pull(Pull::Down).unwrap();

    if let Level::High = pin_a1.get_level() {
        address += 2;
    }

    let mut pin_a2 = PinDriver::input(pin_a2).unwrap();
    pin_a2.set_pull(Pull::Down).unwrap();
    if let Level::High = pin_a2.get_level() {
        address += 4;
    }

    info!("I2C address based on DIP: {address}");

    address
}
