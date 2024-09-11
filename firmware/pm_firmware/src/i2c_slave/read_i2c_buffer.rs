use esp_idf_svc::{
    hal::{delay::TickType, i2c::I2cSlaveDriver},
    sys::{i2c_reset_tx_fifo, TickType_t},
};

/// Таймаут ожидания нового символа. Если задать 0, то будут ошибки передачи
const WAIT_SYMBOL_TIMEOUT: TickType_t = TickType::new_millis(5).ticks();

pub fn read_i2c_buffer<'a>(first_byte: u8, i2c_slave_driver: &mut I2cSlaveDriver<'a>) -> Vec<u8> {
    let mut request_buffer = vec![];
    request_buffer.push(first_byte);
    let mut request_byte: [u8; 1] = [0];
    while i2c_slave_driver
        .read(&mut request_byte, WAIT_SYMBOL_TIMEOUT)
        .is_ok()
    {
        request_buffer.push(request_byte[0]);
    }

    // Сбрасываем буфер отправки
    unsafe { i2c_reset_tx_fifo(i2c_slave_driver.port()) };

    request_buffer
}
