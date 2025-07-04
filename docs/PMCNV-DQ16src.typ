#import "functions.typ": all_pcb_data

#let name = "PMCNV-DQ16src"

#pagebreak()

== #name - 16 дискретных выходов <PMCNV-DQ16src>

Модуль для управления 16 дискретными выходами. Питание выходов внешнее, до 50 В постоянного напряжения.

Функциональная схема платы представлена на @PM_CNV-DQ16__function[рисунке].

#figure(
  caption: "Функциональная схема платы",
  image("images/plates_function_scheme/PM_CNV-DQ16.svg", width: 100%),
) <PM_CNV-DQ16__function>

Управляющие сигналы с платы микроконтроллера #link(<PMMCU-ESP32C3>)[PMMCU-ESP32C3] поступают по протоколу SPI на GPIO расширитель MCP23S17 (@MCP23S17[раздел]). Выходы с расширителя поступают на затворы двух транзисторных сборок (@TBD62783AFG[раздел]). Транзисторы коммутируют напряжение, которое подается на клеммы модуля; получается две группы по 8 дискретных выходов. Максимальное напряжение - 50 В постоянного напряжения.

Параллельно все 16 сигналов идут на плату светодиодов #link(<PM-LED_18>)[PM-LED_18], для индикации состояния выходов.

#all_pcb_data(name: name)
