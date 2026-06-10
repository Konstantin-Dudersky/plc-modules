#import "functions.typ": all_pcb_data, i2c, listing

#let name = "PMIFC-I2Cx8"

#pagebreak()

== #name - Подключение 8 устройств по I2C <PMIFC-I2Cx8>

https://arduino.stackexchange.com/questions/33174/detecting-i2c-transfer-with-led

#figure(
  caption: "Адрес микросхемы TCA9548",
  table(
    columns: (auto, auto, auto, auto),
    align: (center, center, center, center),

    "A2", "A1", "A0", "Адрес",
    "-", "-", "-", "0x70",
    "-", "-", "+", "0x71",
    "-", "+", "-", "0x72",
    "-", "+", "+", "0x73",
    "+", "-", "-", "0x74",
    "+", "-", "+", "0x75",
    "+", "+", "-", "0x76",
    "+", "+", "+", "0x77",
  ),
)<TCA9548_address>

=== Подключение по #i2c

Проверка работы из консоли. Устройство с адресом 0x70.

#let code = ```sh
# Проверить доступность
i2cdetect -y 0

# Открыть каналы с 0 по 7
i2ctransfer -y 0 w1@0x70 0x01
i2ctransfer -y 0 w1@0x70 0x02
i2ctransfer -y 0 w1@0x70 0x04
i2ctransfer -y 0 w1@0x70 0x08
i2ctransfer -y 0 w1@0x70 0x10
i2ctransfer -y 0 w1@0x70 0x20
i2ctransfer -y 0 w1@0x70 0x40
i2ctransfer -y 0 w1@0x70 0x80

# Открыть все каналы
i2ctransfer -y 0 w1@0x70 0xFF

# Закрыть все каналы
i2ctransfer -y 0 w1@0x70 0x00
```
#listing(
  content: code,
  caption: [Проверка работы из консоли],
  label: <pmcnv_dq16src_i2c_test>,
  breakable: true,
)

#all_pcb_data(name: name)
