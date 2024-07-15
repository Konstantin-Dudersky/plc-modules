#set text(lang: "ru", font: "PT Serif", hyphenate: true)
#show heading.where(level: 1): it => {
  pagebreak(weak: true);
  it.body;
  v(1em);
}
#show heading.where(level: 2): it => { pagebreak(weak: true); it }
#show link: underline

= Линейка контроллеров и модулей расширения

Линейка простых модулей ввода/вывода с гальванической изоляцией. Модули подключаются по шине I#super[2]C к контроллеру. Контроллером может выступать любое устройство с поддержкой данного протокола. Для унификации разрботаны модули CPU на базе микроконтроллера ESP32-C3 и мини-компьютера Raspberry Pi.

Модули PM-DI16 и PM-RQ8 собраны на базе одной микросхемы PCA9555. В одной сборке допустимо использовать суммарно до 8 модулей такого типа.

Размеры всех плат одинаковы. Отверстия для крепления также располагаются одинакого.

#figure(
  caption: "Перечень модулей",
  table(
    columns: (auto, 1fr),
    align: (center + horizon, left),

    table.cell(colspan: 2)[ЦПУ],

    link(<PM-CPU-ESP>)[PM-CPU-ESP],
    "ЦПУ на базе микроконтроллера ESP32-C3",

    link(<PM-CPU-RP>)[PM-CPU-RP],
    "ЦПУ на базе мини-компьютера Raspberry Pi",

    "PM-MUX",
    "",

    table.cell(colspan: 2)[Модули ввода / вывода],

    link(<PM-DI16>)[PM-DI16],
    "Модуль для подключения 16 дискретных входов постоянного напряжения",

    link(<PM-RQ8>)[PM-RQ8],
    "Модуль для подключение 8 релейных выходов",

    "PM-AI-UI",
    "",

    "PM-AI-TC",
    "",

    "PM-AI-RTD",
    "",

    "PM-AQ",
    "",

    table.cell(colspan: 2)[Фронтальные платы],

    link(<PM-CPU-ESP-front>)[PM-CPU-ESP-front],
    [Фронтальная плата для модуля #link(<PM-CPU-ESP>)[PM-CPU-ESP]],

    link(<PM-DI16-front-DC24>)[PM-DI16-front-DC24],
    [Фронтальная плата для модуля #link(<PM-DI16>)[PM-DI16], для работы с источником напряжения DC24],

    link(<PM-RQ8-front>)[PM-RQ8-front],
    [Фронтальная плата для модуля #link(<PM-RQ8>)[PM-RQ8]],

    table.cell(colspan: 2)[Аксесуары],

    link(<PM-Bus>)[PM-Bus],
    "Шинный соединитель между модулями"
  )
)

#figure(
  caption: "Размеры основной печатной платы",
  image("../../PM-CPU-ESP/doc/PM-CPU-ESP-User_Comments.svg")
)

#figure(
  caption: "Размеры фронтальной печатной платы",
  image("../../PM-RQ8-front/doc/PM-RQ8-front-User_Comments.svg")
)


#include "PM-CPU-ESP.typ"
#include "PM-CPU-ESP-front.typ"

#include "PM-CPU-RP.typ"

#include "PM-DI16.typ"
#include "PM-DI16-front-DC24.typ"

#include "PM-RQ8.typ"
#include "PM-RQ8-front.typ"

#include "PM-Bus.typ"
