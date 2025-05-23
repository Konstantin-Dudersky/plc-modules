#import "functions.typ": all_pcb_data

#let name = "PM_CNV-RQ8"

#pagebreak()

== #name - 8 релейных выходов <PM_CNV-RQ8>

Модуль для подключения 8 релейных выходов. Допустимая подключаемая нагрузка на канал 2А. Контакты реле подключаются независимо, что позволяет подключать нагрузку от разных источников.

Функциональная схема платы представлена на @PM_CNV-RQ8__function.

#figure(
  caption: "Функциональная схема платы",
  image("images/plates_function_scheme/PM_CNV-RQ8.svg", width: 100%),
) <PM_CNV-RQ8__function>

Управляющие сигналы с платы микроконтроллера #link(<PM_MCU-ESP32_C3>)[PM_MCU-ESP32_C3] поступают по протоколу SPI на GPIO расширитель MCP23S17 (@MCP23S17[раздел]). Поскольку катушки реле потребляют значительную по меркам микроэлектроники мощность, сигналы с расширителя управляют только транзисторами (@TBD62783AFG[раздел]), а уже с транзисторов идет питание на катушки реле. С расширителя также выходит параллельно 8 сигналов на плату светодиодов #link(<PM_LED-10>)[PM_LED-10], для индикации состояния реле.

Использованы реле G5NB-1A-E-DC5 компании Omron. Основные характеристики:

- Напряжение катушки 5 В постоянного напряжения.
- Ток потребления катушки 40 мА.
- Номинальная нагрузка:
  - 5 А 230 В переменного напряжения.
  - 3 А 30 В постоянного напряжения.
- 100000 операций при коммутации 5 А 230 В переменного напряжения.

#all_pcb_data(name: name)
