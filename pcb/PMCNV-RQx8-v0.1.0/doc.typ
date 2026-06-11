#import "../doc/functions.typ": all_pcb_data, calc_power_consumtion, i2c, listing, table_power_consumtion
#import "../doc/table_options.typ": bom_options, table_options
#import "../doc/load_pcb_data.typ": load_pcb_data

#let article = "PMCNV-RQx8"
#let version = "0.1.0"

#pagebreak()

== #article - 8 релейных выходов <PMCNV-RQx8>

Модуль для подключения 8 релейных выходов. Допустимая подключаемая нагрузка на канал 2А. Контакты реле подключаются независимо, что позволяет подключать нагрузку от разных источников.

Функциональная схема платы представлена на @PM_CNV-RQ8__function[рисунке].

#figure(
  caption: "Функциональная схема платы",
  image("../doc/images/plates_function_scheme/PM_CNV-RQ8.svg", width: 100%),
) <PM_CNV-RQ8__function>

Управляющие сигналы с платы микроконтроллера #link(<PMMCU-ESP32C3>)[PMMCU-ESP32C3] поступают по протоколу SPI на GPIO расширитель MCP23S17 (@MCP23S17[раздел]). Поскольку катушки реле потребляют значительную по меркам микроэлектроники мощность, сигналы с расширителя управляют только транзисторами (@TBD62783AFG[раздел]), а уже с транзисторов идет питание на катушки реле. С расширителя также выходит параллельно 8 сигналов на плату светодиодов #link(<PMLED-10>)[PMLED-10], для индикации состояния реле.

Использованы реле G5NB-1A-E-DC5 компании Omron. Основные характеристики:

- Напряжение катушки 5 В постоянного напряжения.
- Ток потребления катушки 40 мА.
- Номинальная нагрузка:
  - 5 А 230 В переменного напряжения.
  - 3 А 30 В постоянного напряжения.
- 100000 операций при коммутации 5 А 230 В переменного напряжения.

=== Коммуникация с микроконтроллером

Для связи с микроконтроллером можно использовать интерфейс SPI или #i2c.

Для использования интерфейса SPI:
- установить коннекторы JM1 и JM2;
- выбрать сигнал CS, установив одну из перемычек CS0..CS3;
- замкнуть сигналы A0..A2 на "-";
- установить в посадочное место U1 микросхему MCP23S17/SS (@MCP23S17[раздел]).

Для использования интерфейса #i2c:
- установить коннекторы JM3 и JM4;
- замкнуть сигналы A0..A2 на "-" или "+". Адрес устройства определяется по @MCP23x17_address[таблице];
- установить в посадочное место U1 микросхему MCP23017/SS (@MCP23S17[раздел])

=== Подключение по #i2c

Проверка работы из консоли. Устройство с адресом 0x20.

#let code = ```sh
# Проверить доступность
i2cdetect -y 0

# GPIOA устанавливаем в режим выхода
i2ctransfer -y 0 w2@0x20 0x00 0x00

# GPIOB устанавливаем в режим выхода
i2ctransfer -y 0 w2@0x20 0x01 0x00

# Включаем все реле
i2ctransfer -y 0 w2@0x20 0x12 0xFF

# Включаем реле R0, R1 .. R7
i2ctransfer -y 0 w2@0x20 0x12 0x80
i2ctransfer -y 0 w2@0x20 0x12 0x40
i2ctransfer -y 0 w2@0x20 0x12 0x20
i2ctransfer -y 0 w2@0x20 0x12 0x10
i2ctransfer -y 0 w2@0x20 0x12 0x08
i2ctransfer -y 0 w2@0x20 0x12 0x04
i2ctransfer -y 0 w2@0x20 0x12 0x02
i2ctransfer -y 0 w2@0x20 0x12 0x01


# Отключаем все реле
i2ctransfer -y 0 w2@0x20 0x12 0x00
```
#listing(
  content: code,
  caption: [Проверка работы из консоли],
  label: <pmcnv_dq16src_i2c_test>,
  breakable: true,
)

=== Расчёт потребления

#let power_consumption = (
  G5NB-1A-E-DC5: (8, 0, 200),
  "MCP23S17-E/SO": (1, 3.3, 0),
  TBD62783AFG: (2, 0, 60),
  "Светодиоды": (10, 9.9, 0),
)
#let pmcnv_rq8_power = calc_power_consumtion(power_consumption)
#table_power_consumtion(values: pmcnv_rq8_power)

=== Опции

#let options = (
  (
    desc: "Количество реле",
    choices: (
      (name: "1", desc: "1 реле", price: 777),
      (name: "2", desc: "2 реле", price: 777),
      (name: "3", desc: "3 реле", price: 777),
      (name: "4", desc: "4 реле", price: 777),
      (name: "5", desc: "5 реле", price: 777),
      (name: "6", desc: "6 реле", price: 777),
      (name: "7", desc: "7 реле", price: 777),
      (name: "8", desc: "8 реле", price: 777),
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
    (name: "1", value: "1"),
    (name: "1", value: "2"),
    (name: "1", value: "3"),
    (name: "1", value: "4"),
    (name: "1", value: "5"),
    (name: "1", value: "6"),
    (name: "1", value: "7"),
    (name: "1", value: "8"),
  ),
)


#load_pcb_data(
  article: article,
  version: version,
)
