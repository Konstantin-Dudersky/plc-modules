= PM-CPU-ESP <PM-CPU-ESP>

ЦПУ на базе микроконтроллера ESP32-C3.

Микроконтроллер ESP32 можно программировать:

- на языке програмиирования #link("https://docs.esp-rs.org/book/")[Rust], как `std`, так и `no_std`.
- на языках C / C++ с помощью фреймворка #link("https://idf.espressif.com")[ESP-IDF]
- используя #link("https://espressif.github.io/arduino-esp32/")[Arduino IDE]

Инетрфейсы модуля:

- WiFi
- Ethernet (на базе микросхемы W5500)
- USB Type-C - для загрузки программы и отладки
- USB A Female - для питания внешних устройств

Программно микроконтроллер ESP32-C3 может предоставлять данные:

- как HTTP-сервер
- как Websocket-сервер
- как MQTT-клиент

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
