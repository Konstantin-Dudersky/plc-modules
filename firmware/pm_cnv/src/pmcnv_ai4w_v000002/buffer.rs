use rsiot::components_config::master_device::BufferBound;
use rsiot_physical_quantities::{self as rpq, DIMENSIONLESS, TEMPERATURE};

use crate::chips::ad7193;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Buffer {
    pub write_registers: WriteRegisters,
    pub read_registers: ReadRegisters,
}
impl Buffer {}
impl BufferBound for Buffer {}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WriteRegisters {
    pub conf_register: ad7193::ConfigurationRegister,
    pub mode_register: ad7193::ModeRegister,
    pub gpocon_register: ad7193::GPOCONRegister,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReadRegisters {
    pub status_register: ad7193::StatusRegister,
    pub mode_register: ad7193::ModeRegister,
    pub conf_register: ad7193::ConfigurationRegister,
    pub data_registers: DataRegisters,
    pub gpocon_register: ad7193::GPOCONRegister,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DataRegisters {
    pub ain1_aincom: Option<rpq::Value<DIMENSIONLESS>>,
    pub ain2_aincom: Option<rpq::Value<DIMENSIONLESS>>,
    pub ain3_aincom: Option<rpq::Value<DIMENSIONLESS>>,
    pub ain4_aincom: Option<rpq::Value<DIMENSIONLESS>>,
    pub ain5_aincom: Option<rpq::Value<DIMENSIONLESS>>,
    pub ain6_aincom: Option<rpq::Value<DIMENSIONLESS>>,
    pub ain7_aincom: Option<rpq::Value<DIMENSIONLESS>>,
    pub ain8_aincom: Option<rpq::Value<DIMENSIONLESS>>,
    pub ain1_ain2: Option<rpq::Value<DIMENSIONLESS>>,
    pub ain3_ain4: Option<rpq::Value<DIMENSIONLESS>>,
    pub ain5_ain6: Option<rpq::Value<DIMENSIONLESS>>,
    pub ain7_ain8: Option<rpq::Value<DIMENSIONLESS>>,
    pub ain2_ain2: Option<rpq::Value<DIMENSIONLESS>>,
    pub aincom_aincom: Option<rpq::Value<DIMENSIONLESS>>,
    pub temperature: Option<rpq::Value<TEMPERATURE>>,
}
impl DataRegisters {
    pub fn set_field(&mut self, key: ad7193::SRChannel, value: f64) {
        match key {
            ad7193::SRChannel::Ain1Ain2 => self.ain1_ain2 = Some(rpq::Value::new_v(value)),
            ad7193::SRChannel::Ain3Ain4 => self.ain3_ain4 = Some(rpq::Value::new_v(value)),
            ad7193::SRChannel::Ain5Ain6 => self.ain5_ain6 = Some(rpq::Value::new_v(value)),
            ad7193::SRChannel::Ain7Ain8 => self.ain7_ain8 = Some(rpq::Value::new_v(value)),
            ad7193::SRChannel::Ain1Aincom => self.ain1_aincom = Some(rpq::Value::new_v(value)),
            ad7193::SRChannel::Ain2Aincom => self.ain2_aincom = Some(rpq::Value::new_v(value)),
            ad7193::SRChannel::Ain3Aincom => self.ain3_aincom = Some(rpq::Value::new_v(value)),
            ad7193::SRChannel::Ain4Aincom => self.ain4_aincom = Some(rpq::Value::new_v(value)),
            ad7193::SRChannel::Ain5Aincom => self.ain5_aincom = Some(rpq::Value::new_v(value)),
            ad7193::SRChannel::Ain6Aincom => self.ain6_aincom = Some(rpq::Value::new_v(value)),
            ad7193::SRChannel::Ain7Aincom => self.ain7_aincom = Some(rpq::Value::new_v(value)),
            ad7193::SRChannel::Ain8Aincom => self.ain8_aincom = Some(rpq::Value::new_v(value)),
            ad7193::SRChannel::Temperature => {
                self.temperature = Some(rpq::Value::new_temperature_K(value))
            }
            ad7193::SRChannel::Ain2Ain2 => self.ain2_ain2 = Some(rpq::Value::new_v(value)),
            ad7193::SRChannel::AincomAincom => self.aincom_aincom = Some(rpq::Value::new_v(value)),
            ad7193::SRChannel::Unknown => (),
        }
    }
}
