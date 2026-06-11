#import "../doc/functions.typ": i2c
#import "../doc/load_pcb_data.typ": load_pcb_data
#import "../doc/table_options.typ": bom_options, table_options

#let article = "PMPS-20W"
#let version = "0.1.0"

#let i2c0 = [I#super[2]C#sub[0]]
#let i2c2 = [I#super[2]C#sub[2]]
#let spi0 = [SPI#sub[0]]

#pagebreak()

== #article - блок питания 20 Вт 230 В <PMPS-20W>

Блок питания IRM-20-24 или аналог.

=== Опции

#let options = (
  (
    desc: "Мощность блока питания AC230/DC24",
    choices: (
      (name: "x", desc: "Без блока питания AC230/DC24", price: 0),
      (name: "5", desc: "Блок питания AC230/DC24 мощностью 5 Вт", price: 777),
      (name: "10", desc: "Блок питания AC230/DC24 мощностью 10 Вт", price: 777),
      (name: "15", desc: "Блок питания AC230/DC24 мощностью 15 Вт", price: 777),
      (name: "20", desc: "Блок питания AC230/DC24 мощностью 20 Вт", price: 777),
    ),
  ),
  (
    desc: "Мощность преобразователя 24В / 5В",
    choices: (
      (name: "x", desc: "Без преобразователя 24В/5В", price: 0),
      (name: "500", desc: "Преобразователь K7805-500 мощностью 2.5 Вт", price: 2.3),
      (name: "1000", desc: "Преобразователь K7805-1000 мощностью 5 Вт", price: 2.8),
      (name: "2000", desc: "Преобразователь K7805-2000 мощностью 10 Вт", price: 4.2),
    ),
  ),
  (
    desc: "Мощность преобразователя 24В / 5В (2)",
    choices: (
      (name: "x", desc: "Без преобразователя 24В/5В", price: 0),
      (name: "500", desc: "Преобразователь K7805-500 мощностью 2.5 Вт", price: 2.3),
      (name: "1000", desc: "Преобразователь K7805-1000 мощностью 5 Вт", price: 2.8),
      (name: "2000", desc: "Преобразователь K7805-2000 мощностью 10 Вт", price: 4.2),
    ),
  ),
  (
    desc: "Мощность преобразователя 24В / 3.3В",
    choices: (
      (name: "x", desc: "Без преобразователя 24В/3.3В", price: 0),
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
  base_price: 777,
)

#bom_options(
  article: article,
  version: version,
  options: (
    (name: "1", value: "5"),
    (name: "1", value: "10"),
    (name: "1", value: "15"),
    (name: "1", value: "20"),
    (name: "2", value: "500"),
    (name: "2", value: "1000"),
    (name: "2", value: "2000"),
    (name: "3", value: "500"),
    (name: "3", value: "1000"),
    (name: "3", value: "2000"),
    (name: "4", value: "500"),
    (name: "4", value: "1000"),
    (name: "4", value: "2000"),
    (name: "5", value: "T"),
    (name: "5", value: "B"),
    (name: "5", value: "TB"),
  ),
)


#load_pcb_data(
  article: article,
  version: version,
)
