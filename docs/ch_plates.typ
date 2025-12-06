#import "PMCNV-DIx16.typ": pmcnv_di16_power
#import "PMCNV-DQ16src.typ": pmcnv_dq16src_power
#import "PMCNV-RQx8.typ": pmcnv_rq8_power
#import "PMCNV-AI4W.typ": pmcnv_ai4w_power

= Описание плат

В @plates_list[таблице] приведён список всех плат.

#set page(
  flipped: true,
  margin: auto,
)

#show figure: set block(breakable: true)
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

    link(<PMLED-10>)[PMLED-10],
    "Плата на 10 светодиодов",
    "0.0.0",
    [],
    [],

    link(<PMLED-18>)[PMLED-18],
    "Плата на 18 светодиодов",
    "0.0.0",
    [],
    [],

    table.cell(colspan: 5)[Платы конвертирования цифровых и электрических сигналов],

    link(<PMCNV-DIx16>)[PMCNV-DIx16],
    "Подключение 16 дискретных входов постоянного напряжения (PNP / NPN)",
    "0.0.8",
    [#pmcnv_di16_power.total_current_3v3 мА \ #pmcnv_di16_power.total_power_3v3 мВт],
    [#pmcnv_di16_power.total_current_5v мА \ #pmcnv_di16_power.total_power_5v мВт],

    link(<PMCNV-Count6>)[PMCNV-Count6],
    "Подключение 6 сигналов быстрого счёта",
    "0.0.0",
    [],
    [],

    link(<PMCNV-DQ16src>)[PMCNV-DQ16src],
    "Подключение 16 дискретных выходов постоянного напряжения",
    "0.0.0",
    [#pmcnv_dq16src_power.total_current_3v3 мА \ #pmcnv_dq16src_power.total_power_3v3 мВт],
    [#pmcnv_dq16src_power.total_current_5v мА \ #pmcnv_dq16src_power.total_power_5v мВт],

    link(<PMCNV-RQx8>)[PMCNV-RQx8],
    "Подключение 8 релейных выходов",
    "0.0.0",
    [#pmcnv_rq8_power.total_current_3v3 мА \ #pmcnv_rq8_power.total_power_3v3 мВт],
    [#pmcnv_rq8_power.total_current_5v мА \ #pmcnv_rq8_power.total_power_5v мВт],

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
    [#pmcnv_ai4w_power.total_current_3v3 мА \ #pmcnv_ai4w_power.total_power_3v3 мВт],
    [#pmcnv_ai4w_power.total_current_5v мА \ #pmcnv_ai4w_power.total_power_5v мВт],

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

    link(<PMCNV-INA226>)[PMCNV-INA226],
    "Контроль электрических параметров с помощью INA226",
    "0.0.1",
    [],
    [],

    link(<PMCNV-EnMon>)[PMCNV-EnMon],
    "Контроль параметров сети 400В",
    "0.0.0",
    [],
    [],

    link(<PMCNV-PWMx16>)[PMCNV-PWMx16],
    "Управление ШИМ 16 каналов",
    "0.0.1",
    [],
    [],

    table.cell(colspan: 5)[Подключение цифровых интерфейсов],

    link(<PMIFC-IIC8>)[PMIFC-IIC8],
    "Подключение 8 устройств по I2C",
    "0.0.1",
    [],
    [],

    link(<PMIFC-1Wx12>)[PMIFC-1Wx12],
    "Подключение 12 устройств по 1-Wire",
    "0.0.0",
    [],
    [],

    table.cell(colspan: 5)[Блоки питания],

    link(<PMPS-20W>)[PMPS-20W],
    "Блок питания 20 Вт",
    "0.0.4",
    [-],
    [-],


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
) <plates_list>

#set page(
  flipped: false,
  margin: auto,
)

#include "PMCPU-RP.typ"
#include "PMCPU-LLP.typ"
#include "PMCPU-LLU.typ"

#include "PMMCU-ESP32C3.typ"

#include "PMLED-10.typ"
#include "PMLED-18.typ"

#include "PMCNV-DIx16.typ"
#include "PMCNV-DQ16src.typ"
#include "PMCNV-Count6.typ"
#include "PMCNV-RQx8.typ"
#include "PMCNV-AI8IU.typ"
#include "PMCNV-AI4R.typ"
#include "PMCNV-AI4T.typ"
#include "PMCNV-AI4W.typ"
#include "PMCNV-AQ.typ"
#include "PMCNV-INA226.typ"
#include "PMCNV-EnMon.typ"
#include "PMCNV-PWMx16.typ"

#include "PMIFC-IIC8.typ"
#include "PMIFC-1Wx12.typ"

#include "PMPS-20W.typ"

#include "PMHMI-Keyboard.typ"
#include "PMHMI-Touch.typ"

#include "PMDBG-FFC.typ"
#include "PMDBG-FFC20.typ"
