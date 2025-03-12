= Описание плат

#figure(
  caption: "Перечень плат",
  table(
    columns: (auto, auto),
    align: (center + horizon, left),

    table.cell(colspan: 2)[ЦПУ],

    link(<PM_CPU-RP>)[PM_CPU-RP],
    "ЦПУ на базе мини-компьютера Raspberry Pi или совместимого с ним",

    link(<PM_CPU-ESP32>)[PM_CPU-ESP32],
    "ЦПУ на базе микроконтроллера ESP32",

    table.cell(colspan: 2)[Платы микроконтроллеров],

    link(<PM_MCU-ESP32_C3>)[PM_MCU-ESP32_C3],
    "Плата на базе микроконтроллера ES32-C3 для работы плат конвертирования и передачи данных на ЦПУ",

    table.cell(colspan: 2)[Платы светодиодов],

    link(<PM_LED-10>)[PM_LED-10],
    "Плата на 10 светодиодов",

    link(<PM_LED-18>)[PM_LED-18],
    "Плата на 18 светодиодов",

    table.cell(colspan: 2)[Платы конвертирования цифровых и электрических сигналов],

    link(<PM_CNV-DI16_sink>)[PM_CNV-DI16_sink],
    "Подключение 16 дискретных входов постоянного напряжения",

    link(<PM_CNV-DQ16_src>)[PM_CNV-DQ16_src],
    "Подключение 16 дискретных выходов постоянного напряжения",

    link(<PM_CNV-RQ8>)[PM_CNV-RQ8],
    "Подключение 8 релейных выходов",

    link(<PM_CNV-AI4_RTD>)[PM_CNV-AI4_RTD],
    "Подключение 4 термосопротивлений",

    link(<PM_CNV-AI4_TC>)[PM_CNV-AI4_TC],
    "Подключение 4 термопар",

    link(<PM_CNV-AI4_W>)[PM_CNV-AI4_W],
    "Подключение 4 тензодатчиков",

    link(<PM_CNV-AI8_IU>)[PM_CNV-AI8_IU],
    "Подключение 8 датчиков 4 .. 20 мА или 0 .. 10 В",

    link(<PM_CNV-AQ>)[PM_CNV-AQ],
    "Подключение аналоговых выходов",

    table.cell(colspan: 2)[Платы для создания интерфейса оператора],

    link(<PM_HMI-Keyboard>)[PM_HMI-Keyboard],
    "Клавиатура 7x7",

    link(<PM_HMI-Touch>)[PM_HMI-Touch],
    "Подключение touch-контроллера дисплея",

    table.cell(colspan: 2)[Платы для отладки],

    link(<PM_DBG-FFC>)[PM_DBG-FFC],
    "Для подключения осциллографа в разрыв кабеля FFC",
  ),
)

#include "PM_CPU-RP.typ"
#include "PM_CPU-ESP32.typ"

#include "PM_MCU-ESP32_C3.typ"

#include "PM_LED-10.typ"
#include "PM_LED-18.typ"

#include "PM_CNV-DI16_sink.typ"
#include "PM_CNV-DQ16_src.typ"
#include "PM_CNV-RQ8.typ"
#include "PM_CNV-AI4_RTD.typ"
#include "PM_CNV-AI4_TC.typ"
#include "PM_CNV-AI4_W.typ"
#include "PM_CNV-AI8_IU.typ"
#include "PM_CNV-AQ.typ"

#include "PM_HMI-Keyboard.typ"
#include "PM_HMI-Touch.typ"

#include "PM_DBG-FFC.typ"
