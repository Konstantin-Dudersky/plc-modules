use rsiot::components_config::spi_master::Operation;

const WRITE_ADDR: u8 = 0x40;
const READ_ADDR: u8 = 0x41;

const REG_IODIRA: u8 = 0x00;
const REG_IODIRB: u8 = 0x01;
const REG_IPOLA: u8 = 0x02;
const REG_IPOLB: u8 = 0x03;
const REG_GPPUA: u8 = 0x0C;
const REG_GPPUB: u8 = 0x0D;
const REG_GPIOA: u8 = 0x12;
const REG_GPIOB: u8 = 0x13;

pub struct MCP23S17 {}

#[allow(dead_code)]
impl MCP23S17 {
    /// Чтение всех регистров
    pub fn read_all() -> Operation {
        Operation::WriteRead([READ_ADDR, REG_IODIRA].to_vec(), 22)
    }

    /// Установить режим работы как вход или как выход:
    /// - 0x00 - выходы
    /// - 0xFF - входы
    pub fn write_iodir_a(value: u8) -> Operation {
        Operation::Write([WRITE_ADDR, REG_IODIRA, value].to_vec())
    }

    /// Установить режим работы как вход или как выход:
    /// - 0x00 - выходы
    /// - 0xFF - входы
    pub fn write_iodir_b(value: u8) -> Operation {
        Operation::Write([WRITE_ADDR, REG_IODIRB, value].to_vec())
    }

    /// Полярность входов:
    /// - 0x00 - прямая полярность
    /// - 0xFF - обратная полярность
    pub fn write_ipol_a(value: u8) -> Operation {
        Operation::Write([WRITE_ADDR, REG_IPOLA, value].to_vec())
    }

    /// Полярность входов:
    /// - 0x00 - прямая полярность
    /// - 0xFF - обратная полярность
    pub fn write_ipol_b(value: u8) -> Operation {
        Operation::Write([WRITE_ADDR, REG_IPOLB, value].to_vec())
    }

    /// Резисторы подтяжки Pull-Up:
    /// - 0x00 - резисторы отключены
    /// - 0xFF - резисторы подключены
    pub fn write_gppua(value: u8) -> Operation {
        Operation::Write([WRITE_ADDR, REG_GPPUA, value].to_vec())
    }

    /// Резисторы подтяжки Pull-Up:
    /// - 0x00 - резисторы отключены
    /// - 0xFF - резисторы подключены
    pub fn write_gppub(value: u8) -> Operation {
        Operation::Write([WRITE_ADDR, REG_GPPUB, value].to_vec())
    }

    /// Установить состояние GPIOA в режиме вывода
    pub fn write_gpio_a(value: u8) -> Operation {
        Operation::Write([WRITE_ADDR, REG_GPIOA, value].to_vec())
    }

    /// Прочитать значение GPIOA
    pub fn read_gpio_a() -> Operation {
        Operation::WriteRead([READ_ADDR, REG_GPIOA].to_vec(), 1)
    }

    /// Установить состояние GPIOB в режиме вывода
    pub fn write_gpio_b(value: u8) -> Operation {
        Operation::Write([WRITE_ADDR, REG_GPIOB, value].to_vec())
    }

    /// Прочитать состояние GPIOB
    pub fn read_gpio_b() -> Operation {
        Operation::WriteRead([READ_ADDR, REG_GPIOB].to_vec(), 1)
    }
}
