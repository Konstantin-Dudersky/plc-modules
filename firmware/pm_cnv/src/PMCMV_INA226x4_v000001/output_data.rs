use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OutputData {
    pub shunt_voltage: f64,
    pub bus_voltage: f64,
    pub power: f64,
    pub current: f64,
}

impl Display for OutputData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "shunt_voltage: {:.5} mV, bus_voltage: {:.5} V, power: {:.5} W, current: {:.5} A",
            self.shunt_voltage * 1000.0,
            self.bus_voltage,
            self.power,
            self.current
        )
    }
}
