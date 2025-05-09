use esp_idf_svc::hal::gpio::{AnyIOPin, Level, PinDriver, Pull};
use tracing::info;

/// Определить адрес по DIP-переключателям
pub fn define_address(pin_a0: &mut AnyIOPin, pin_a1: &mut AnyIOPin, pin_a2: &mut AnyIOPin) -> u8 {
    let mut address = 0;

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

    info!("ESP address based on DIP: {address}");

    address
}
