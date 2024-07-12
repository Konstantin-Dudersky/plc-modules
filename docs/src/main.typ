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
  caption: "Размеры основной печатной платы",
  image("../../PM-CPU-ESP/doc/PM-CPU-ESP-User_Comments.svg")
)

#figure(
  caption: "Размеры фронтальной печатной платы",
  image("../../PM-RQ8-front/doc/PM-RQ8-front-User_Comments.svg")
)


= PM-CPU-ESP

ЦПУ на базе микроконтроллера ESP32-C3.

Микроконтроллер ESP32 можно программировать:

- на языке програмиирования #link("https://docs.esp-rs.org/book/")[Rust], как `std`, так и `no_std`.
- на языках C / C++ с помощью фреймворка #link("https://idf.espressif.com")[ESP-IDF]
- используя #link("https://espressif.github.io/arduino-esp32/")[Arduino IDE]

Инетрфейсы модуля:

- WiFi
- Ethernet
- USB Type-C - для загрузки программы и отладки
- USB A Female - для питания внешних устройств

Программно микроконтроллер ESP32-C3 может предоставлять данные:

- как HTTP-сервер
- как Websocket-сервер

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

ЦПУ на базе мини-компьютера Raspberry Pi, или совместимого по габаритам, креплению и 40-пиновому штекеру.

Программировать можно практически на всех языках программирования, поддерживающих архитектуру процессора ARM64.

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

= PM-DI16

Модуль для подключения 16 дискретных входов постоянного напряжения. Уровень напряжения зависит от используемой платы PM-DI16-front-XXX.

Входы гальванически изолированы от внутреннего источника питания.

Схема модуля собрана на базе микросхемы PCA9555. Адрес на шине I#super[2]С задается с помощью трех перемычек на плате.

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

#figure(
    caption: "Схема внешних подключений PM-DI16",
    image("../../PM-DI16-base/doc/PM-DI16-base-external_connection.svg")
)

= PM-DI16-front-DC24

Фронтальная плата для модуля PM-DI16, для работы с источником напряжения DC24В.

Схема модуля собрана на базе микросхемы PCA9555. Адрес на шине I#super[2]С задается с помощью трех перемычек на плате.

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

Модуль релейных выходов. Допустимая подключаемая нагрузка на канал 2А. Контакты реле подключаются независимо, что позволяет подключать нагрузку от разных источников.

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

#figure(
    caption: "Схема внешних подключений PM-RQ8",
    image("../../PM-RQ8-base/doc/PM-RQ8-base-external_connection.svg")
)


= PM-RQ8-front

Фронтальная плата для модуля PM-RQ8.

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
