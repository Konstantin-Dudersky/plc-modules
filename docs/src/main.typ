#set text(lang: "ru", font: "PT Serif", hyphenate: true)
#show heading.where(level: 1): it => { pagebreak(weak: true); it }
#show heading.where(level: 2): it => { pagebreak(weak: true); it }
#show link: underline

= Линейка контроллеров и модулей расширения

= PM-CPU-ESP

ЦПУ на базе микроконтроллера ESP32-C3.

Микроконтроллер ESP32 можно программировать:

- на языках C / C++ с помощью фреймворка #link("https://idf.espressif.com")[ESP-IDF]
- на языке програмиирования #link("https://docs.esp-rs.org/book/")[Rust], как `std`, так и `no_std`.
- используя #link("https://espressif.github.io/arduino-esp32/")[Arduino IDE]


#figure(
  caption: "Внешний вид PM-CPU-ESP",
  table(
    columns: (auto, auto),
    stroke: none,
    image("../../PM-CPU-ESP/doc/PM-CPU-ESP_view.png", fit: "contain"),
    image("../../PM-CPU-ESP/doc/PM-CPU-ESP_view_y90.png", fit: "contain"),
    image("../../PM-CPU-ESP/doc/PM-CPU-ESP_view_x90.png", fit: "contain"),
    image("../../PM-CPU-ESP/doc/PM-CPU-ESP_view_y180.png", fit: "contain"),
  )
)

#figure(
    caption: "Схема внешних подключений PM-CPU-ESP",
    image("../../PM-CPU-ESP/doc/PM-CPU-ESP-external_connection.svg")
)

#figure(
    caption: "Размеры печатной платы PM-CPU-ESP",
    image("../../PM-CPU-ESP/doc/PM-CPU-ESP-User_Comments.svg")
)

= PM-CPU-ESP-front

Фронтальная плата для модуля PM-CPU-ESP.

#figure(
  caption: "Внешний вид PM-CPU-ESP-front",
  table(
    columns: (auto, auto),
    stroke: none,
    image("../../PM-CPU-ESP-front/doc/PM-CPU-ESP-front_view.png", fit: "contain"),
    image("../../PM-CPU-ESP-front/doc/PM-CPU-ESP-front_view_y90.png", fit: "contain"),
    image("../../PM-CPU-ESP-front/doc/PM-CPU-ESP-front_view_x90.png", fit: "contain"),
    image("../../PM-CPU-ESP-front/doc/PM-CPU-ESP-front_view_y180.png", fit: "contain"),
  )
)

= PM-CPU-RP

ЦПУ на базе мини-компьютера Raspberry Pi, или совместимого по габаритам и 40-пиновому штекеру.

#figure(
  caption: "Внешний вид PM-CPU-RP",
  table(
    columns: (auto, auto),
    stroke: none,
    image("../../PM-CPU-RP/doc/PM-CPU-RP_view.png", fit: "contain"),
    image("../../PM-CPU-RP/doc/PM-CPU-RP_view_y90.png", fit: "contain"),
    image("../../PM-CPU-RP/doc/PM-CPU-RP_view_x90.png", fit: "contain"),
    image("../../PM-CPU-RP/doc/PM-CPU-RP_view_y180.png", fit: "contain"),
  )
)

#figure(
    caption: "Схема внешних подключений PM-CPU-RP",
    image("../../PM-CPU-RP/doc/PM-CPU-RP-external_connection.svg")
)

#figure(
    caption: "Размеры печатной платы PM-CPU-RP",
    image("../../PM-CPU-RP/doc/PM-CPU-RP-User_Comments.svg")
)


= PM-DI16

#figure(
  caption: "Внешний вид PM-DI16",
  table(
    columns: (auto, auto),
    stroke: none,
    image("../../PM-DI16-base/doc/PM-DI16-base_view.png", fit: "contain"),
    image("../../PM-DI16-base/doc/PM-DI16-base_view_y90.png", fit: "contain"),
    image("../../PM-DI16-base/doc/PM-DI16-base_view_x90.png", fit: "contain"),
    image("../../PM-DI16-base/doc/PM-DI16-base_view_y180.png", fit: "contain"),
  )
)

= PM-DI16-front-DC24

#figure(
  caption: "Внешний вид PM-DI16-front-DC24",
  table(
    columns: (auto, auto),
    stroke: none,
    image("../../PM-DI16-front-DC24/doc/PM-DI16-front-DC24_view.png", fit: "contain"),
    image("../../PM-DI16-front-DC24/doc/PM-DI16-front-DC24_view_y90.png", fit: "contain"),
    image("../../PM-DI16-front-DC24/doc/PM-DI16-front-DC24_view_x90.png", fit: "contain"),
    image("../../PM-DI16-front-DC24/doc/PM-DI16-front-DC24_view_y180.png", fit: "contain"),
  )
)

= PM-RQ8

Модуль релейных выходов. Допустимая подключаемая нагрузка на канал - 2А.

#figure(
  caption: "Внешний вид PM-RQ8",
  table(
    columns: (auto, auto),
    stroke: none,
    image("../../PM-RQ8-base/doc/PM-RQ8-base_view.png", fit: "contain"),
    image("../../PM-RQ8-base/doc/PM-RQ8-base_view_y_90.png", fit: "contain"),
    image("../../PM-RQ8-base/doc/PM-RQ8-base_view_x_90.png", fit: "contain"),
    image("../../PM-RQ8-base/doc/PM-RQ8-base_view_y_180.png", fit: "contain"),
  )
)

= PM-RQ8-front

#figure(
  caption: "Внешний вид PM-RQ8-front",
  table(
    columns: (auto, auto),
    stroke: none,
    image("../../PM-RQ8-front/doc/PM-RQ8-front_view.png", fit: "contain"),
    image("../../PM-RQ8-front/doc/PM-RQ8-front_view_y_90.png", fit: "contain"),
    image("../../PM-RQ8-front/doc/PM-RQ8-front_view_x_90.png", fit: "contain"),
    image("../../PM-RQ8-front/doc/PM-RQ8-front_view_y_180.png", fit: "contain"),
  )
)
