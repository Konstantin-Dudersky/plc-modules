#let table_choices(
  choices: (),
) = {
  let table_body = ()

  for choice in choices {
    table_body.push("☐")
    table_body.push(choice.name)
    table_body.push(choice.desc)
    table_body.push("\u{002B}" + str(choice.price) + "\u{0024}")
  }

  table(
    columns: (auto, auto, 1fr, auto),
    stroke: none,
    align: (center, center, left, right),

    ..for c in table_body {
      (
        [#c],
      )
    },
  )
}

#let table_options(
  options: (),
  module_name: str,
  base_price: float,
) = {
  let option_index = 1
  let table_body = ()

  for option in options {
    table_body.push(option_index)
    option_index = option_index + 1

    let base_price1 = str(base_price) + "\u{0024}"

    let table_choices_body = table_choices(choices: option.choices)
    table_body.push(option.desc + table_choices_body)
  }

  figure(
    caption: "Опции модуля " + module_name,

    table(
      columns: (auto, 1fr),
      stroke: (x: none),
      align: (left + top, left + top),

      table.cell(colspan: 2)[
        #table(
          columns: (1fr, 1fr),
          stroke: none,
          align: (left, right),

          [Базовая цена], [#str(base_price)\u{0024}],
        )
      ],

      ..for c in table_body {
        (
          [#c],
        )
      },
    ),
  )
}

// Загрузить данные с перечнем элементов
#let load_bom(path) = {
  let results = csv(path, row-type: dictionary)
  let results = results.map(el => {
    let part_ipn = el.part_ipn
    let quantity = el.quantity
    let pricing_min = el.pricing_min
    let pricing_max = el.pricing_max
    let pricing_min_all = el.pricing_min_all
    let pricing_max_all = el.pricing_max_all
    (part_ipn, quantity, pricing_min, pricing_max, pricing_min_all, pricing_max_all)
  })
  results.flatten()
}

#let bom_options(
  module_name: str,
  options: (),
) = {
  // Спецификация базовой версии
  let bom = load_bom("pcb/" + module_name + "/inventree_bom/price_base.csv")
  figure(
    caption: "Спецификация базовой версии",
    table(
      columns: (auto, auto, 1fr, 1fr, 1fr, 1fr),
      table.header([IPN], [Кол-во], [Мин.], [Макс.], [Мин. (все)], [Макс. (все)]),
      ..bom,
    ),
  )

  //
  for o in options {
    let bom = load_bom("pcb/" + module_name + "/inventree_bom/price_" + o.name + "_" + o.value + ".csv")

    figure(
      caption: "Спецификация опции " + o.name + " = " + o.value,
      table(
        columns: (auto, auto, 1fr, 1fr, 1fr, 1fr),
        table.header([IPN], [Кол-во], [Мин.], [Макс.], [Мин. (все)], [Макс. (все)]),
        ..bom,
      ),
    )
  }
}
