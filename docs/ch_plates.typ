= Описание плат

#figure(
  caption: "Перечень плат",
  table(
    columns: (auto, auto, auto, auto, auto),
    align: (center + horizon, left + horizon, center + horizon, center + horizon, center + horizon),

    table.header("Артикул", "Описание", "Версия", "3,3 В", "5 В"),

    table.cell(colspan: 5)[Платы ЦПУ],

    link(<PMCPU-RP>)[PMCPU-RP],
    "ЦПУ на базе мини-компьютера Raspberry Pi или совместимого с ним",
    "0.0.0",
    [],
    [],

    link(<PMCPU-LLP>)[PMCPU-LLP],
    "ЦПУ на базе мини-компьютера Luckfox Lyra / Luckfox Lyra Plus",
    "0.0.0",
    [],
    [],

    link(<PMCPU-LLU>)[PMCPU-LLU],
    "ЦПУ на базе мини-компьютера Luckfox Lyra Ultra",
    "0.0.0",
    [],
    [],

    table.cell(colspan: 5)[Платы микроконтроллеров],

    link(<PMMCU-ESP32C3>)[PMMCU-ESP32C3],
    "Плата на базе микроконтроллера ES32-C3 для работы плат конвертирования и передачи данных на ЦПУ",
    "0.0.7",
    [],
    [],

    table.cell(colspan: 5)[Платы светодиодов],

    link(<PM-LED_10>)[PMLED-10],
    "Плата на 10 светодиодов",
    "0.0.0",
    [],
    [],

    link(<PM-LED_18>)[PMLED-18],
    "Плата на 18 светодиодов",
    "0.0.0",
    [],
    [],

    table.cell(colspan: 5)[Платы конвертирования цифровых и электрических сигналов],

    link(<PMCNV-DI16sink>)[PMCNV-DI16sink],
    "Подключение 16 дискретных входов постоянного напряжения",
    "0.0.0",
    [70 мА \ 232 мВт],
    [-],

    link(<PMCNV-DI16src>)[PMCNV-DI16src],
    "Подключение 16 дискретных выходов постоянного напряжения",
    "0.0.0",
    [70 мА \ 232 мВт],
    [-],

    link(<PMCNV-DQ16src>)[PMCNV-DQ16src],
    "Подключение 16 дискретных выходов постоянного напряжения",
    "0.0.0",
    [7 мА \ 23 мВт],
    [198 мА \ 991 мВт],

    link(<PMCNV-RQ8>)[PMCNV-RQ8],
    "Подключение 8 релейных выходов",
    "0.0.0",
    [31 мА \ 102 мВт],
    [344 мА \ 1720 мВт],

    link(<PMCNV-AI4R>)[PMCNV-AI4R],
    "Подключение 4 термосопротивлений",
    "0.0.0",
    [],
    [],

    link(<PMCNV-AI4T>)[PMCNV-AI4T],
    "Подключение 4 термопар",
    "0.0.2",
    [],
    [],

    link(<PM-CNV_AI4-W>)[PMCNV-AI4W],
    "Подключение 4 тензодатчиков",
    "0.0.3",
    [],
    [],

    link(<PMCNV-AI8IU>)[PMCNV-AI8IU],
    "Подключение 8 датчиков 4 .. 20 мА или 0 .. 10 В",
    "0.0.0",
    [],
    [],

    link(<PMCNV-AQ>)[PMCNV-AQ],
    "Подключение аналоговых выходов",
    "0.0.0",
    [],
    [],

    "PMCNV-IIC8",
    "Подключение 8 устройств по I2C",
    "0.0.0",
    [],
    [],

    table.cell(colspan: 5)[Платы для создания интерфейса оператора],

    link(<PM-HMI_Keyboard>)[PM-HMI_Keyboard],
    "Клавиатура 7x7",
    "0.0.0",
    [],
    [],

    link(<PM-HMI_Touch>)[PM-HMI_Touch],
    "Подключение touch-контроллера дисплея",
    "0.0.0",
    [],
    [],

    table.cell(colspan: 5)[Платы для отладки],

    link(<PM-DBG_FFC>)[PMDBG-FFC],
    "Для подключения осциллографа в разрыв кабеля FFC",
    "0.0.0",
    [],
    [],

    link(<PMDBG-FFC20>)[PMDBG-FFC20],
    "Для подключения осциллографа в разрыв кабеля FFC 20-пинов",
    "0.0.1",
    [],
    [],
  ),
)

#include "PMCPU-RP.typ"
#include "PMCPU-LLP.typ"
#include "PMCPU-LLU.typ"

#include "PMMCU-ESP32C3.typ"

#include "PMLED-10.typ"
#include "PMLED-18.typ"

#include "PMCNV-DI16sink.typ"
#include "PMCNV-DI16src.typ"
#include "PMCNV-DQ16src.typ"
#include "PMCNV-RQ8.typ"
#include "PMCNV-AI8IU.typ"
#include "PMCNV-AI4R.typ"
#include "PMCNV-AI4T.typ"
#include "PMCNV-AI4W.typ"
#include "PMCNV-AQ.typ"

#include "PMHMI-Keyboard.typ"
#include "PMHMI-Touch.typ"

#include "PMDBG-FFC.typ"
#include "PMDBG-FFC20.typ"
