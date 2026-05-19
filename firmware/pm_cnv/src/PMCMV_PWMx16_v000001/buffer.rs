use rsiot::components_config::{i2c_master::I2cAddress, master_device::BufferBound};

/// Частота осциллятора - 25 МГц.
const OSC_CLOCK: f32 = 25_000_000.0;

const CYCLE: f32 = 4096.0;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Buffer {
    pub address: I2cAddress,
    pub update_rate: f32,

    pub write: Write,
}
impl Buffer {
    pub fn prescale(&self) -> u8 {
        let prescale = (OSC_CLOCK / (CYCLE * self.update_rate) - 1.0).round() as i32;

        if prescale > 0xFF {
            0xFF
        } else if prescale < 0x03 {
            0x03
        } else {
            prescale as u8
        }
    }
}

/// Настройка канала
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Channel {
    /// Задержка от начала цикла, [%]. По умолчанию 0.
    pub delay_time: f32,

    /// Длительность импульса, [%].
    pub pwm_duty_cycle: f32,
}
impl Channel {
    // Возвращает регистры для записи в чип в виде массива [ON_L, ON_H, OFF_L, OFF_H]
    pub fn registers(&self) -> [u8; 4] {
        // TODO - крайние случаи - 100% и 0%

        if self.pwm_duty_cycle == 0.0 {
            return [0x00, 0x00, 0x00, 0x10];
        }
        if self.pwm_duty_cycle == 100.0 {
            return [0x00, 0x10, 0x00, 0x00];
        }

        let led_on: u16 = (self.delay_time * (CYCLE - 1.0) / 100.0).round() as u16;
        let led_on_bytes = led_on.to_le_bytes();

        let led_off: u16 =
            ((self.pwm_duty_cycle + self.delay_time) * (CYCLE - 1.0) / 100.0).round() as u16;

        let led_off: u16 = if led_off as f32 >= CYCLE {
            led_off - CYCLE as u16
        } else {
            led_off
        };

        let led_off_bytes = led_off.to_le_bytes();

        [
            led_on_bytes[0],
            led_on_bytes[1],
            led_off_bytes[0],
            led_off_bytes[1],
        ]
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Write {
    pub ch00: Channel,
    pub ch01: Channel,
    pub ch02: Channel,
    pub ch03: Channel,
    pub ch04: Channel,
    pub ch05: Channel,
    pub ch06: Channel,
    pub ch07: Channel,
    pub ch08: Channel,
    pub ch09: Channel,
    pub ch10: Channel,
    pub ch11: Channel,
    pub ch12: Channel,
    pub ch13: Channel,
    pub ch14: Channel,
    pub ch15: Channel,
}

impl BufferBound for Buffer {}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    /// Пример 1 из документации
    #[test]
    fn channel_example_1() {
        let ch = Channel {
            delay_time: 10.0,
            pwm_duty_cycle: 20.0,
        };

        assert_eq!(ch.registers(), [0x99, 0x01, 0xCB, 0x04]);
    }

    #[test]
    fn prescale() {
        let buf = Buffer {
            update_rate: 200.0,
            ..Default::default()
        };

        assert_eq!(buf.prescale(), 30);
    }
}
