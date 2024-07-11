#set text(lang: "ru", font: "PT Serif", hyphenate: true)
#show heading.where(level: 1): it => { pagebreak(weak: true); it }
#show heading.where(level: 2): it => { pagebreak(weak: true); it }
#show link: underline

= Линейка контроллеров и модулей расширения

== PM-CPU-ESP

ЦПУ на базе микроконтроллера ESP32-C3.

Микроконтроллер ESP32 можно программировать:

- на языках C / C++ с помощью фреймворка #link("https://idf.espressif.com")[ESP-IDF]
- языке програмиирования #link("https://docs.esp-rs.org/book/")[Rust], как `std`, так и `no_std`.
- используя #link("https://espressif.github.io/arduino-esp32/")[Arduino IDE]

#pagebreak()

#table(
  columns: (auto, auto),
  stroke: none,

  image("images/PM-CPU-ESP_0.png", height: 200pt),
  image("images/PM-CPU-ESP_180.png", height: 200pt),
  image("images/PM-CPU-ESP_90.png", height: 200pt),
  image("images/PM-CPU-ESP_x_90.png", height: 200pt),

)


== PM-CPU-ESP-front

Фронтальная плата для модуля PM-CPU-ESP.

== PM-CPU-RP

ЦПУ на базе мини-компьютера Raspberry Pi, или совместимого по габаритам и 40-пиновому штекеру.

== PM-RQ8

== PM-RQ8-front

== PM-DI16

== PM-DI16-front-DC24
