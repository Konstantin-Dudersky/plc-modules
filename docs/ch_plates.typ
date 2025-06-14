= Описание плат

#figure(
  caption: "Перечень плат",
  table(
    columns: (auto, auto),
    align: (center + horizon, left),

    table.cell(colspan: 2)[Платы ЦПУ],

    link(<PMCPU-RP>)[PMCPU-RP],
    "ЦПУ на базе мини-компьютера Raspberry Pi или совместимого с ним",

    link(<PMCPU-LLP>)[PMCPU-LLP],
    "ЦПУ на базе мини-компьютера Luckfox Lyra / Luckfox Lyra Plus",

    link(<PMCPU-LLU>)[PMCPU-LLU],
    "ЦПУ на базе мини-компьютера Luckfox Lyra Ultra",

    link(<PMCPU_ESP32>)[PMCPU-ESP32],
    "ЦПУ на базе микроконтроллера ESP32",

    table.cell(colspan: 2)[Платы микроконтроллеров],

    link(<PMMCU-ESP32C3>)[PMMCU-ESP32C3],
    "Плата на базе микроконтроллера ES32-C3 для работы плат конвертирования и передачи данных на ЦПУ",

    table.cell(colspan: 2)[Платы светодиодов],

    link(<PM-LED_10>)[PM-LED_10],
    "Плата на 10 светодиодов",

    link(<PM-LED_18>)[PM-LED_18],
    "Плата на 18 светодиодов",

    table.cell(colspan: 2)[Платы конвертирования цифровых и электрических сигналов],

    link(<PM-CNV_DI16-sink>)[PM-CNV_DI16-sink],
    "Подключение 16 дискретных входов постоянного напряжения",

    link(<PMCNV-DQ16src>)[PMCNV-DQ16src],
    "Подключение 16 дискретных выходов постоянного напряжения",

    link(<PMCNV-RQ8>)[PMCNV-RQ8],
    "Подключение 8 релейных выходов",

    link(<PMCNV-AI4R>)[PMCNV-AI4R],
    "Подключение 4 термосопротивлений",

    link(<PMCNV-AI4T>)[PMCNV-AI4T],
    "Подключение 4 термопар",

    link(<PM-CNV_AI4-W>)[PM-CNV_AI4-W],
    "Подключение 4 тензодатчиков",

    link(<PMCNV-AI8IU>)[PMCNV-AI8IU],
    "Подключение 8 датчиков 4 .. 20 мА или 0 .. 10 В",

    link(<PMCNV-AQ>)[PMCNV-AQ],
    "Подключение аналоговых выходов",

    table.cell(colspan: 2)[Платы для создания интерфейса оператора],

    link(<PM-HMI_Keyboard>)[PM-HMI_Keyboard],
    "Клавиатура 7x7",

    link(<PM-HMI_Touch>)[PM-HMI_Touch],
    "Подключение touch-контроллера дисплея",

    table.cell(colspan: 2)[Платы для отладки],

    link(<PM-DBG_FFC>)[PM-DBG_FFC],
    "Для подключения осциллографа в разрыв кабеля FFC",
  ),
)

#include "PMCPU-RP.typ"
#include "PMCPU-LLP.typ"
#include "PMCPU-LLU.typ"
#include "PMCPU-ESP32.typ"

#include "PMMCU-ESP32C3.typ"

#include "PMLED-10.typ"
#include "PMLED-18.typ"

#include "PMCNV-DI16sink.typ"
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
