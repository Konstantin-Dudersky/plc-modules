use std::time::Duration;

use esp_idf_svc::hal::{gpio::AnyIOPin, i2c::I2cSlaveDriver, spi::SpiDriver};
use tokio::time::sleep;
use tracing::{info, trace, warn};

use crate::{error::Error, i2c_slave, postcard_serde, spi_devices::mcp23s17::MCP23S17};

use super::{I2cRequest, I2cResponse};

pub struct Module<'a> {
    pub spi_master_driver: &'a SpiDriver<'static>,
    pub pin_cs_gpio_expander: AnyIOPin,
    pub i2c_slave_driver: I2cSlaveDriver<'a>,
}

impl<'a> Module<'a> {
    pub async fn spawn(mut self) {
        let mut gpio_expander = MCP23S17::new(&self.spi_master_driver, self.pin_cs_gpio_expander);

        // Инициализация модуля
        gpio_expander.set_iodira(0xFF);
        gpio_expander.set_iodirb(0xFF);

        task_read(&mut gpio_expander).await;

        // loop {
        //     let result = module_loop(&mut self.i2c_slave_driver, &mut gpio_expander);
        //     if let Err(err) = result {
        //         warn!("{err}");
        //     }
        // }
    }
}

pub async fn task_read<'a>(gpio_expander: &mut MCP23S17<'a>) {
    loop {
        let read = gpio_expander.get_gpiob();
        info!("Read gpio: {read}");
        sleep(Duration::from_millis(500)).await;
    }
}

fn module_loop<'a>(
    i2c_slave_driver: &mut I2cSlaveDriver<'a>,
    gpio_expander: &mut MCP23S17<'a>,
) -> Result<(), Error> {
    // Ждем, пока появится в буфере байт
    let first_byte = i2c_slave::wait_first_byte(i2c_slave_driver)?;

    // Читаем весь буфер
    let mut request_buffer = i2c_slave::read_i2c_buffer(first_byte, i2c_slave_driver);

    // Десериализация запроса
    let request: I2cRequest = postcard_serde::deserialize(&mut request_buffer)?;

    // Обработка запроса
    trace!("Request: {:?}", request);
    let response = match request {
        I2cRequest::SetOutputs(data) => {
            gpio_expander.set_gpioa(data);
            I2cResponse::Ok
        }
    };
    trace!("Response: {:?}", response);

    // Сериализация ответа
    let response_buffer = postcard_serde::serialize(&response)?;

    // Запись в буфер отправки I2C
    i2c_slave::write_i2c_buffer(i2c_slave_driver, &response_buffer)?;

    Ok(())
}
