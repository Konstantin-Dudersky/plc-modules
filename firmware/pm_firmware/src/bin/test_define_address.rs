//! cargo run --release --bin test_define_address

use std::time::Duration;

use esp_idf_svc::hal::{
    gpio::{Level, PinDriver, Pull},
    peripheral::Peripheral,
    prelude::Peripherals,
};
use tokio::time::sleep;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let mut pin_a0 = peripherals.pins.gpio0;
    let pin_a1 = peripherals.pins.gpio1;
    let pin_a2 = peripherals.pins.gpio2;

    let pin_a0_clone = unsafe { pin_a0.clone_unchecked() };

    let mut pin_a0_input = PinDriver::input(pin_a0_clone).unwrap();
    pin_a0_input.set_pull(Pull::Up).unwrap();
    let level_0 = pin_a0_input.get_level();
    println!("Pin A0: {:?}", level_0);

    let mut pin_a0_output = PinDriver::output(pin_a0).unwrap();
    pin_a0_output.set_high().unwrap();

    loop {
        sleep(Duration::from_millis(1000)).await;
    }
}