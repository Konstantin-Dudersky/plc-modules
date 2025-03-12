= Основные микроэлектронные компоненты

Ниже перечислены основные электронные компоненты, которые применяются в платах.


== AD7193 - 24-разрядный \u{03a3} - \u{0394} АЦП <AD7193>

АЦП AD7193 компании Analog Devices хорошо подходит для измерения показаний датчиков с небольшим полным диапазоном измерения. Основные характеристики АЦП:

- Тип АЦП - 24-разрядный сигма-дельта (\u{03a3} - \u{0394}).
- Встроенный усилитель с программируемым коэффициентом усиления 1 .. 128. Позволяет измерять напряжения в диапазоне 40 мВ .. 5 В.
- Низкий уровень шумов. До 22 разрядов при низкой частоте дискретизации.
- 4 дифференциальных канала измерения. Автоматический мультиплексор.
- Подключается к микроконтроллеру по интерфейсу SPI.

Используется в платах:

- #link(<PM_CNV-AI4_W>)[PM_CNV-AI4_W]
- #link(<PM_CNV-AI4_RTD>)[PM_CNV-AI4_RTD]
- #link(<PM_CNV-AI4_TC>)[PM_CNV-AI4_TC]

== ADS8688 <ADS8688>

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

- #link(<PM_CPU-RP>)[PM_CPU-RP]
- #link(<PM_MCU-ESP32_C3>)[PM_MCU-ESP32_C3]

== CA-IS3105W - изоляция питания

== CA-IS3741HW - изоляция SPI

== CA-IS3980S

В промышленности для подключения цифровых датчиков наиболее часто применяется постоянное напряжение 24 В, тогда как микроконтроллеры работают, как правило, на напряжении 3,3 В или 5 В.

В данном проекте в качестве изолятора применяется чип CA-IS3980S @CA-IS3980S_datasheet. Чип обеспечивает изоляцию 2500 $V_("RMS")$ и $±300 "kV"/"µs"$ CMTI. Фунциональная схема показана на @CA-IS3980S[рисунке].

#figure(
  caption: "Функциональная схема изолятора цифровых входов",
  image("images/CA-IS3980P.png", height: 200pt),
) <CA-IS3980S>

На @CA-IS3980S_channel[рисунке] показана упрощенная схема отдельного канала. The internal LED emulator output drives an ON-OFF keying (OOK) modulator, to transfer digital signals across the SiO2 based isolation barrier between circuits with different power domains. In many applications, this capacitive isolation technology is replacing optocoupler-based solution because it can reduce the power requirements and take up less board space while offering the same isolation capability.

#figure(
  caption: "Упрощенная схема для отдельного канала",
  image("images/CA-IS3980P_channel.png", width: 100%),
) <CA-IS3980S_channel>

On the output side, the signal is either passed directly to the output stage in the case of a high-speed channel (BHx), or the signal is routed through a debounce filter block in the case of a low-speed channel (Bx) for robust operation in industrial environments.

Данный чип применен на плате #link(<PM_CNV-DI16_sink>)[PM_CNV-DI16_sink].

== MCP23S17/SS

== TBD62783AFG

== DC / DC преобразователи 24 В / 5 В

@SLOA327A:

== DS3231

== Светодиоды серии XL-1606

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
