#import "functions.typ": i2c

= Основные микроэлектронные компоненты

Ниже перечислены основные электронные компоненты, которые применяются в платах.

== AD7193 - 24-разрядный \u{03a3} - \u{0394} АЦП <AD7193>

АЦП AD7193 компании Analog Devices хорошо подходит для измерения показаний датчиков с небольшим полным диапазоном измерения. Основные характеристики АЦП:

- Тип АЦП - 24-разрядный сигма-дельта (\u{03a3} - \u{0394}).
- Встроенный усилитель с программируемым коэффициентом усиления 1 .. 128. Позволяет измерять напряжения в диапазоне 40 мВ .. 5 В.
- Низкий уровень шумов. До 22 разрядов при низкой частоте дискретизации.
- 4 дифференциальных канала измерения. Автоматический мультиплексор.
- Подключается к микроконтроллеру по интерфейсу SPI.

Функциональная схема показана на @AD7193_functional[рисунке].

#figure(
  caption: "Функциональная схема АЦП AD7193",
  image("images/AD7193.png", height: 200pt),
) <AD7193_functional>


Используется в платах:

- #link(<PM-CNV_AI4-W>)[PM-CNV_AI4-W]
- #link(<PMCNV-AI4R>)[PMCNV-AI4R]
- #link(<PMCNV-AI4T>)[PMCNV-AI4T]

== ADS8688 <ADS8688>

== B2405 - DC / DC преобразователи 24 В / 5 В

- B2405LS-1WR3 - мощность 1 Вт
- B2405S-2WR3 - мощность 2 Вт
- B2405S-3WR3 - мощность 3 Вт

Основные характеристики:



Используется в платах:

- #link(<PMMCU-ESP32C3>)[PMMCU-ESP32C3]

== CA-IS3098W - приемопередатчик интерфейса RS-485 <CA-IS3098W>

В микроконтроллерах / мини-компьютерах присутствует интерфейс UART. Для преобразования в протокол RS-485 используются передатчики CA-IS3098W @CA-IS3098W_datasheet со встроенной гальванической изоляцией.

Основные параметры:

- Скорость передачи - до 20Mbps
- До 256 устройств в одном сегменте сети
- Гальваническая изоляция: 5 $"kV"_("RMS")$ и $±150 "kV"/"µs"$ CMTI
- Защита от короткого замыкания и перегрева

На @CA-IS3098W_functional[рисунке] показана функциональная схема передатчика.

#figure(
  caption: "Функциональная схема передатчика RS-485",
  image("images/CA-IS3098W.png", height: 200pt),
) <CA-IS3098W_functional>

Обозначения выводов:

- DE (Driver Output Enable) - Разрешение выходов передатчика
- DI (Driver Input) — Вход передатчика
- RO (Receiver Output) - Выход приемника
- #overline[RE] (Receiver Output Enable) - Разрешение выхода приемника
- A - Неинвертирующий вход/выход
- B - Инвертирующий вход/выход

Используется в платах:

- #link(<PMCPU-RP>)[PM_CPU-RP]
- #link(<PMMCU-ESP32C3>)[PMMCU-ESP32C3]

== CA-IS3105W - изоляция питания

== CA-IS3741HW - изоляция SPI

== CA-IS3980S - изоляция цифровых входов <CA-IS3980S>

В промышленности для подключения цифровых датчиков наиболее часто применяется постоянное напряжение 24 В, тогда как микроконтроллеры работают, как правило, на напряжении 3,3 В или 5 В.

В данном проекте в качестве изолятора применяется чип CA-IS3980S @CA-IS3980S_datasheet. Фунциональная схема показана на @CA-IS3980S_functional[рисунке].

#figure(
  caption: "Функциональная схема изолятора цифровых входов",
  image("images/CA-IS3980P.png", height: 200pt),
) <CA-IS3980S_functional>

Основные характеристики:

- Совместимость со стандартом IEC 61131-2, тип входов 1, 2 и 3.
- 8 параллельных каналов.
- Скорость передачи до 2 Mbps.
- $±300 "kV"/"µs"$ CMTI.
- $2500 V_("RMS")$ изоляция.

На @CA-IS3980S_channel[рисунке] показана упрощенная схема отдельного канала. The internal LED emulator output drives an ON-OFF keying (OOK) modulator, to transfer digital signals across the SiO2 based isolation barrier between circuits with different power domains. In many applications, this capacitive isolation technology is replacing optocoupler-based solution because it can reduce the power requirements and take up less board space while offering the same isolation capability.

#figure(
  caption: "Упрощенная схема для отдельного канала",
  image("images/CA-IS3980P_channel.png", width: 100%),
) <CA-IS3980S_channel>

On the output side, the signal is either passed directly to the output stage in the case of a high-speed channel (BHx), or the signal is routed through a debounce filter block in the case of a low-speed channel (Bx) for robust operation in industrial environments.

Используется в платах:
- #link(<PMCNV-DIx16>)[PMCNV-DIx16].

== DS3231 - часы реального времени <DS3231>



== MCP23x17 - 16-битный расширитель входов / выходов <MCP23S17>

Применяется для опроса состояния входов (на платах входов) и управления выходами (на платах выходов).

Основные характеристики:

- 16 двунаправленных входов / выходов
- Связь с микроконтроллером по протоколу SPI (MCP32S17) или #i2c (MCP32017).
- Выходы с прерываниями по событию срабатывания входов.

Функциональная схема показана на @MCP23S17_channel[рисунке].

#figure(
  caption: "Функциональная схема MCP23x17",
  image("images/MCP23S17.png", height: 250pt),
) <MCP23S17_channel>

Адрес устройства определяется замыканием пинов A0..A2 на "-" или "+".

#figure(
  caption: "Адрес микросхемы MCP23x17",
  table(
    columns: (auto, auto, auto, auto),
    align: (center, center, center, center),

    "A2", "A1", "A0", "Адрес",
    "-", "-", "-", "32",
    "-", "-", "+", "33",
    "-", "+", "-", "34",
    "-", "+", "+", "35",
    "+", "-", "-", "36",
    "+", "-", "+", "37",
    "+", "+", "-", "38",
    "+", "+", "+", "39",
  )
)<MCP23x17_address>

Используется в платах:

- #link(<PMCNV-DIx16>)[PMCNV-DIx16]
- #link(<PMCNV-DQ16src>)[PMCNV-DQ16src]
- #link(<PMCNV-RQx8>)[PMCNV-RQx8]
- #link(<PM-HMI_Keyboard>)[PM-HMI_Keyboard]

== TBD62783AFG - матрица из 8 DMOS транзисторов <TBD62783AFG>

Используется для управления силовыми нагрузками, посколько нагрузочная способность выходов микроконтроллера довольно низкая.

Основные характеристики:

- напряжение питания до 50 В
- выходной ток на каждый канал до 500 мА

На @TBD62783AFG_channel[рисунке] показана эквивалентная схема для отдельного канала.

#figure(
  caption: "Эквивалентная схема для отдельного канала",
  image("images/TBD62783AFG.png", width: 60%),
) <TBD62783AFG_channel>

Используется в платах:

- #link(<PMCNV-DQ16src>)[PMCNV-DQ16src]
- #link(<PMCNV-RQx8>)[PMCNV-RQx8]




== Светодиоды серии XL-1606 <XL-1606>

Серия светодиодов бокового свечения компании XinLight. Устанавливаются в посадочное место размера 0602.

#figure(
  table(
    columns: (50%, 50%),
    table.header(
      repeat: true,
      [*Артикул*],
      [*Цвет*],
    ),

    [XL-1606UYC], [желтый],
    [XL-1606UWC], [белый],
    [XL-1606UOC], [оранжевый],
    [XL-1606UGC], [изумрудно-зеленый],
    [XL-1606UBC], [синий],
    [XL-1606SYGC], [зеленый],
    [XL-1606SURC], [красный],
  ),
)
