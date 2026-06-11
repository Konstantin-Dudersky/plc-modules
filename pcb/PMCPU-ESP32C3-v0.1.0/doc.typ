
#import "../doc/functions.typ": calc_power_consumtion, i2c, listing, table_power_consumtion
#import "../doc/table_options.typ": bom_options, table_options
#import "../doc/load_pcb_data.typ": load_pcb_data

#let article = "PMCPU-ESP32C3"
#let version = "0.1.0"

#pagebreak()

== #article - CPU на базе микроконтроллера ESP32-C3 <PMCPU-ESP32C3>

Основное назначение - система удалённого ввода/вывода

=== Опции

#let options = (
  (
    desc: "Тип дополнительного внешнего интерфейса",
    choices: (
      (name: "x", desc: "Без дополнительного интерфейса", price: 0),
      (name: "RS485", desc: "RS485", price: 5),
      (name: "Eth", desc: "Ethernet", price: 10),
    ),
  ),
  (
    desc: "Мощность преобразователя 24В / 5В",
    choices: (
      (name: "x", desc: "Без преобразователя 24В/5В. Выбрать в случае установки модуля PMPS", price: 0),
      (name: "500", desc: "Преобразователь K7805-500 мощностью 2.5 Вт", price: 2.3),
      (name: "1000", desc: "Преобразователь K7805-1000 мощностью 5 Вт", price: 2.8),
      (name: "2000", desc: "Преобразователь K7805-2000 мощностью 10 Вт", price: 4.2),
    ),
  ),
  (
    desc: "Мощность преобразователя 24В / 3.3В",
    choices: (
      (name: "x", desc: "Без преобразователя 24В/3.3В. Выбрать в случае установки модуля PMPS", price: 0),
      (name: "500", desc: "Преобразователь K7803-500 мощностью 1.65 Вт", price: 2.4),
      (name: "1000", desc: "Преобразователь K7803-1000 мощностью 3.3 Вт", price: 2.9),
      (name: "2000", desc: "Преобразователь K7803-2000 мощностью 6.6 Вт", price: 3.5),
    ),
  ),
  (
    desc: "Коннекторы шины IBus",
    choices: (
      (name: "T", desc: "Только сверху (top)", price: 0.1),
      (name: "B", desc: "Только снизу (bottom)", price: 0.1),
      (name: "TB", desc: "Сверху и снизу (top + bottom)", price: 0.2),
    ),
  ),
)

#table_options(
  options: options,
  module_name: article,
  base_price: 10.0,
)

#bom_options(
  article: article,
  version: version,
  options: (
    (name: "1", value: "RS485"),
    (name: "2", value: "500"),
    (name: "2", value: "1000"),
    (name: "2", value: "2000"),
    (name: "3", value: "500"),
    (name: "3", value: "1000"),
    (name: "3", value: "2000"),
    (name: "4", value: "T"),
    (name: "4", value: "B"),
    (name: "4", value: "TB"),
  ),
)

#load_pcb_data(article: article)
