#import "doc/style.typ": style
#show: style

#outline()

#include "doc/ch_terms.typ"

= Описание системы управления

#figure(
  caption: "Концепция модульной системы",
  image("./doc/images/overview.svg"),
)

Система управления состоит из набора совместимых по подключению и внешним габаритам модулей.

Отдельные модули связаны между собой общей шиной. Шина выполнена в виде стандратного кабеля с двумя витыми парами, обжатого двумя разъемами RJ11. Модули необходимо располагать так, чтобы суммарная длина шины была минимальной. По шине передаётся питание (GND, +5V) и сигналы интерфейса I#super[2]C (SDA, SCL).

Каждый модуль на шине имеет свой адрес, который задается DIP-переключателями на каждом модуле. Возможно использование до 8 модулей одного типа.

На данный момент реализованы модули:

// - Модуль блока питания 230В (#link(<PM-PS>)[PM-PS])
// - Модуль ЦПУ на базе Raspberry Pi (#link(<PM-CPU-RP>)[PM-CPU-RP])
// - Модуль 16 дискретных входов 24 В (#link(<PM-DI16-DC24sink>)[PM-DI16-DC24sink])
// - Модуль 8 релейных выходов 2A (#link(<PM-RQ8>)[PM-RQ8])

Можно дополнительно реализовать:

- Модули аналоговых входов: #list([Тензодатчики], [Ток 4-20мА], [Напряжение 0-10В], [Термосопротивление], [Термопары])
- Модули аналоговых выходов: #list([Ток 4-20мА], [Напряжение 0-10В])
- Модуль с аккумуляторными батареями
- Модуль управления RGB светодиодными лентами



#include "doc/ch_module_overview.typ"
#include "doc/ch_galvanic.typ"
#include "doc/ch_protocol_betweenmodule.typ"
#include "doc/ch_protocol_insidemodule.typ"
#include "doc/ch_components.typ"
#include "doc/ch_drc.typ"

#import "doc/PMCNV-DQx16.typ": pmcnv_dq16src_power
#import "doc/PMCNV-RQx8.typ": pmcnv_rq8_power
#import "doc/PMCNV-AI4W.typ": pmcnv_ai4w_power
#import "PMCNV-AI8T-v0.1.0/doc.typ" as PMCNV-AI8T
#import "PMCNV-DIx32-v0.1.0/doc.typ" as PMCNV-DIx32
#import "PMCPU-ESP32C3-v0.1.0/doc.typ" as PMCPU-ESP32C3
#import "PMCPU-LLP-v0.1.0/doc.typ" as PMCPU-LLP


= Описание модулей

В @modules_list[таблице] приведён список всех модулей.

#set page(
  flipped: true,
  margin: auto,
)

#show figure: set block(breakable: true)
#figure(
  caption: "Перечень модулей",
  table(
    columns: 5,
    align: (center + horizon, left + horizon, center + horizon, center + horizon, center + horizon),

    table.header("Артикул", "Описание", "Версия", "3,3 В", "5 В"),

    table.cell(colspan: 5)[Модули ЦПУ],

    link(<PMCPU-ESP32C3>)[PMCPU-ESP32C3.article],
    "ЦПУ на базе микроконтроллера ESP32-C3",
    PMCPU-ESP32C3.version,
    [],
    [],

    link(<PMCPU-LLP>)[PMCPU-LLP.article],
    "ЦПУ на базе мини-компьютера Luckfox Lyra / Luckfox Lyra Plus",
    PMCPU-LLP.version,
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

    link(<PMCNV-DIx32>)[PMCNV-DIx32],
    "Подключение 32 дискретных входов постоянного напряжения (PNP / NPN)",
    PMCNV-DIx32.version,
    [PMCNV-DIx32.total_current_3v3 мА \ PMCNV-DIx32.total_power_3v3 мВт],
    [PMCNV-DIx32.total_current_5v мА \ PMCNV-DIx32.total_power_5v мВт],

    link(<PMCNV-Count6>)[PMCNV-Count6],
    "Подключение 6 сигналов быстрого счёта",
    "0.0.0",
    [],
    [],

    link(<PMCNV-DQx16>)[PMCNV-DQx16],
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

    link(<PMCNV-AI8T>)[PMCNV-AI8T.article],
    "Подключение 8 термопар",
    PMCNV-AI8T.version,
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

    link(<PMIFC-I2Cx8>)[PMIFC-I2Cx8],
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
) <modules_list>

#set page(
  flipped: false,
  margin: auto,
)

#include "PMCNV-AI8T-v0.1.0/doc.typ"
#include "PMCNV-DIx32-v0.1.0/doc.typ"
#include "PMCPU-ESP32C3-v0.1.0/doc.typ"
#include "PMCPU-LLP-v0.1.0/doc.typ"

#include "doc/PMCPU-RP.typ"
#include "doc/PMCPU-LLU.typ"

#include "doc/PMMCU-ESP32C3.typ"

#include "doc/PMLED-10.typ"
#include "doc/PMLED-18.typ"

#include "doc/PMCNV-DQx16.typ"
#include "doc/PMCNV-Count6.typ"
#include "doc/PMCNV-RQx8.typ"
#include "doc/PMCNV-AI8IU.typ"
#include "doc/PMCNV-AI4R.typ"
#include "doc/PMCNV-AI4W.typ"
#include "doc/PMCNV-AQ.typ"
#include "doc/PMCNV-INA226.typ"
#include "doc/PMCNV-EnMon.typ"
#include "doc/PMCNV-PWMx16.typ"

#include "doc/PMIFC-I2Cx8.typ"
#include "doc/PMIFC-1Wx12.typ"

#include "doc/PMPS-20W.typ"

#include "doc/PMHMI-Keyboard.typ"
#include "doc/PMHMI-Touch.typ"

#include "doc/PMDBG-FFC.typ"
#include "doc/PMDBG-FFC20.typ"



= Комбинирование плат в модули

#include "doc/ch_dimensions.typ"


#bibliography("doc/bibliography.yml")
