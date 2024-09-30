#set text(lang: "ru", font: "PT Serif", hyphenate: true)
#show heading.where(level: 1): it => {
  pagebreak(weak: true);
  it.body;
  v(1em);
}
#show heading.where(level: 2): it => { pagebreak(weak: true); it }
#show link: underline

#figure(
  caption: "Концепция модульной системы",
  image("../images/overview.svg"),
)

На данный момент реализованы модули:

- Модуль блока питания 230В
- Модуль ЦПУ на базе Raspberry Pi
- Модуль 16 дискретных входов 24 В (#link(<PM-DI16-DC24sink>)[PM-DI16-DC24sink])
- Модуль 8 релейных выходов 2A (#link(<PM-RQ8>)[PM-RQ8])

Можно дополнительно реализовать:

- Модули аналоговых входов (ток, напряжение, вес, температура)
- Модули аналоговых выходов (ток, напряжение)
- Модуль с аккумуляторными батареями
- Модуль управления светодиодными лентами


#figure(
  caption: "Функциональная cхема модуля ввода / вывода",
  image("../images/module_overview.svg"),
)


= Описание модулей и вспомогательных плат

// Линейка простых модулей ввода/вывода с гальванической изоляцией. Модули подключаются по шине I#super[2]C к контроллеру. Контроллером может выступать любое устройство с поддержкой данного протокола. Для унификации разрботаны модули CPU на базе микроконтроллера ESP32-C3 и мини-компьютера Raspberry Pi.

// Модули PM-DI16 и PM-RQ8 собраны на базе одной микросхемы PCA9555. В одной сборке допустимо использовать суммарно до 8 модулей такого типа.

// Размеры всех плат одинаковы. Отверстия для крепления также располагаются одинакого.

#figure(
  caption: "Перечень модулей",
  table(
    columns: (auto, 1fr),
    align: (center + horizon, left),

    table.cell(colspan: 2)[ЦПУ],

    link(<PM-CPU-RP>)[PM-CPU-RP],
    "ЦПУ на базе мини-компьютера Raspberry Pi или совместимого с ним",

    table.cell(colspan: 2)[Модули ввода / вывода],

    link(<PM-DI16-DC24sink>)[PM-DI16-DC24sink],
    "Модуль для подключения 16 дискретных входов постоянного напряжения",

    link(<PM-RQ8>)[PM-RQ8],
    "Модуль для подключение 8 релейных выходов",

    table.cell(colspan: 2)[Блоки питания],

    link(<PM-PS>)[PM-PS],
    "Модуль блока питания",

    table.cell(colspan: 2)[Вспомогательные платы],

    link(<PM-ESP32C3>)[PM-ESP32C3],
    "Плата с микроконтроллером управления модулем на базе ESP32-C3",

    link(<PM-LED8>)[PM-LED8],
    "Плата с 8 светодиодами",

    link(<PM-LED16>)[PM-LED16],
    "Плата с 16 светодиодами",
  )
)

// #figure(
//   caption: "Размеры основной печатной платы",
//   image("../../PM-CPU-ESP/doc/PM-CPU-ESP-User_Comments.svg")
// )

// #figure(
//   caption: "Размеры фронтальной печатной платы",
//   image("../../PM-RQ8-front/doc/PM-RQ8-front-User_Comments.svg")
// )


#include "PM-CPU-RP.typ"

#include "PM-DI16-DC24sink.typ"
#include "PM-RQ8.typ"

#include "PM-PS.typ"

#include "PM-ESP32C3.typ"
#include "PM-LED8.typ"
#include "PM-LED16.typ"
