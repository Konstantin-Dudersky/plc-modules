#import "../doc/functions.typ": all_pcb_data, calc_power_consumtion, i2c, listing, table_power_consumtion
#import "../doc/table_options.typ": bom_options, table_options
#import "../doc/load_pcb_data.typ": load_pcb_data

#let article = "PMCNV-DQx16"
#let version = "0.1.0"

#pagebreak()

== #article - 16 дискретных выходов <PMCNV-DQx16>

Модуль для управления 16 дискретными выходами. Питание выходов внешнее, до 50 В постоянного напряжения.

Функциональная схема платы представлена на @PM_CNV-DQ16__function[рисунке].

#figure(
  caption: "Функциональная схема платы",
  image("../doc/images/plates_function_scheme/PM_CNV-DQ16.svg", width: 100%),
) <PM_CNV-DQ16__function>

Управляющие сигналы с платы микроконтроллера #link(<PMMCU-ESP32C3>)[PMMCU-ESP32C3] поступают по протоколу SPI на GPIO расширитель MCP23S17 (@MCP23S17[раздел]). Выходы с расширителя поступают на затворы двух транзисторных сборок (@TBD62783AFG[раздел]). Транзисторы коммутируют напряжение, которое подается на клеммы модуля; получается две группы по 8 дискретных выходов. Максимальное напряжение - 50 В постоянного напряжения.

Параллельно все 16 сигналов идут на плату светодиодов #link(<PMLED-18>)[PMLED-18], для индикации состояния выходов.

=== Подключение по #i2c

Проверка работы из консоли. Устройство с адресом 0x20.

#let code = ```sh
# Проверить доступность
i2cdetect -y 0

# GPIOA устанавливаем в режим выхода
i2ctransfer -y 0 w2@0x20 0x00 0x00

# GPIOB устанавливаем в режим выхода
i2ctransfer -y 0 w2@0x20 0x01 0x00

# Включаем все выходы GPIOA
i2ctransfer -y 0 w2@0x20 0x12 0xFF

# Отключаем все выходы GPIOA
i2ctransfer -y 0 w2@0x20 0x12 0x00

# Включаем все выходы GPIOB
i2ctransfer -y 0 w2@0x20 0x13 0xFF

# Отключаем все выходы GPIOB
i2ctransfer -y 0 w2@0x20 0x13 0x00
```
#listing(
  content: code,
  caption: [Проверка работы из консоли],
  label: <pmcnv_dq16src_i2c_test>,
  breakable: true,
)

=== Расчёт потребления

#let power_consumption = (
  B0505LS-1W: (1, 0, 400),
  CA-IS3741HW: (1, 22.77, 70),
  "MCP23S17-E/SO": (1, 0, 5),
  TBD62783AFG: (2, 0, 60),
  "Светодиоды": (18, 0, 22),
)
#let pmcnv_dq16src_power = calc_power_consumtion(power_consumption)
#table_power_consumtion(values: pmcnv_dq16src_power)

=== Опции

#let options = (
  (
    desc: "Количество каналов выходов",
    choices: (
      (name: "8", desc: "8 каналов", price: 777),
      (name: "16", desc: "16 каналов", price: 777),
    ),
  ),
  (
    desc: "Коннекторы шины IBus",
    choices: (
      (name: "T", desc: "Только сверху (top)", price: 0.1),
      (name: "B", desc: "Только снизу (bottom)", price: 0.1),
      (name: "TB", desc: "Сверху и снизу (top + bottom)", price: 0.2),
    ),
  ),
)

#table_options(
  options: options,
  module_name: article,
  base_price: 777,
)

#bom_options(
  article: article,
  version: version,
  options: (
    (name: "1", value: "8"),
    (name: "1", value: "16"),
    (name: "2", value: "T"),
    (name: "2", value: "B"),
    (name: "2", value: "TB"),
  ),
)

#load_pcb_data(
  article: article,
  version: version,
)
