// Переменные

#let i2c = [I#super[2]C]

#let decimal_sep(num, decimal: ",", thousands: "") = {
  let parts = str(num).split(".")
  let decimal_part = if parts.len() == 2 { parts.at(1) }
  let integer_part = parts.at(0).rev().clusters().enumerate()
    .map((item) => {
      let (index, value) = item
      return value + if calc.rem(index, 3) == 0 and index != 0 {
        thousands
      }
    }).rev().join("")
  return integer_part + if decimal_part != none { decimal + decimal_part }
}


#let image_render(
  name: "plate_name",
  render_x0y0z0: "path",
  render_x0y180z0: "path",
) = {
  set page(
    flipped: true,
    margin: 1cm,
  )

  figure(
    caption: "Внешний вид платы " + name,
    kind: image,
    table(
      columns: (50%, 50%),
      image(render_x0y0z0), image(render_x0y180z0),
    ),
  )

  set page(
    flipped: false,
    margin: auto,
  )
}

#let image_scheme(
  name: "plate_name",
  path: "path",
) = {
  set page(
    flipped: true,
    margin: 1cm,
  )

  figure(
    caption: "Принципиальная электрическая схема платы " + name,
    image(path, width: 95%),
  )

  set page(
    flipped: false,
    margin: auto,
  )
}

// Загрузить данные с перечнем элементов
#let load_bom(path) = {
  let results = csv(path, row-type: dictionary)
  let results = results.map(el => {
    let reference = el.reference.replace(",", ", ")
    let quantity = el.quantity
    let part_ipn = el.part_ipn
    (reference, quantity, part_ipn)
  })
  results.flatten()
}

// Вывести таблицу с перечнем элементов
#let table_bom(
  name: "plate_name",
  path: "bom.csv",
) = {
  let bom = load_bom(path)

  figure(
    caption: "Перечень элементов платы " + name,

    table(
      columns: (35%, 15%, 50%),
      table.header(
        repeat: true,
        [*Обозначение*],
        [*Количество*],
        [*part_ipn*],
      ),
      ..bom,
    ),
  )
}

#let all_pcb_data(
  name: "plate_name",
) = {
  let path_render_x0y0z0 = "pcb/" + name + "/render_x0y0z0.png"
  let path_render_x0y180z0 = "pcb/" + name + "/render_x0y180z0.png"
  let path_scheme = "pcb/" + name + "/" + name + ".svg"
  let path_bom = "./pcb/" + name + "/BOM.csv"

  image_render(
    name: name,
    render_x0y0z0: path_render_x0y0z0,
    render_x0y180z0: path_render_x0y180z0,
  )
  image_scheme(
    name: name,
    path: path_scheme,
  )
  table_bom(
    name: name,
    path: path_bom,
  )
}

/// Отображение листинга кода.
#let listing(
  content: str,
  caption: str,
  label: [any],
  breakable: false
) = {
  set par(justify: false)
  [
    #show figure: set block(breakable: true) if breakable
    #figure(
      block(
        fill: luma(250),
        radius: 3pt,
        stroke: .6pt + luma(200),
        inset:	(x: .45em, y: .65em),
        width: 100%,
        clip: false,
        [#align(left)[#content]]
      ),
      caption: figure.caption(position: bottom)[#caption],
      supplement: "Листинг",
      kind: "code",
    ) #label
  ]
}


/// Расчет потребления
#let calc_power_consumtion(d) = {
  let table = ()
  let total_power_3v3 = 0
  let total_power_5v = 0

  for item in d.pairs() {
    let name = item.at(0)
    let count = item.at(1).at(0)
    let power_3v3 = item.at(1).at(1)
    let power_5v = item.at(1).at(2)

    let power_3v3_count = count * power_3v3
    let power_5v_count = count * power_5v

    total_power_3v3 += power_3v3_count
    total_power_5v += power_5v_count

    power_3v3_count = decimal_sep(calc.round(power_3v3_count, digits: 1))
    power_5v_count = decimal_sep(calc.round(power_5v_count, digits: 1))

    table.push(name)
    table.push([#count])
    table.push([#decimal_sep(power_3v3)])
    table.push([#decimal_sep(power_5v)])
    table.push([#power_3v3_count])
    table.push([#power_5v_count])

  }

  total_power_3v3 = calc.round(total_power_3v3, digits: 1)
  let total_current_3v3 = calc.round(total_power_3v3 / 3.3, digits: 1)
  total_power_5v = calc.round(total_power_5v, digits: 1)
  let total_current_5v = calc.round(total_power_5v / 5, digits: 1)

  (
    table: table,
    total_power_3v3: decimal_sep(total_power_3v3),
    total_power_5v: decimal_sep(total_power_5v),
    total_current_3v3: decimal_sep(total_current_3v3),
    total_current_5v: decimal_sep(total_current_5v),
  )
}

#let table_power_consumtion(
  values: (),
) = {

  figure(
    caption: "Расчёт потребления",
    table(
      columns: (25%, 15%, 15%, 15%, 15%, 15%),
      align: (center, center, center, center, center, center),

      table.cell(rowspan: 2, align: horizon)[Элемент],
      table.cell(rowspan: 2, align: horizon)[Кол-во],
      table.cell(colspan: 2)[Потребление, мВт],
      table.cell(colspan: 2)[Итого, мВт],
      "3,3 В",
      "5 В",
      "3,3 В",
      "5 В",

      ..for c in values.table {(
        [#c],
      )},

      table.cell(colspan: 4, align: left)[Итого, мВт: ],//#decimal_sep(values.)],
      [#decimal_sep(values.total_power_3v3)],
      [#decimal_sep(values.total_power_5v)],

      table.cell(colspan: 4, align: left)[Итого, мА],
      [#decimal_sep(values.total_current_3v3)],
      [#decimal_sep(values.total_current_5v)],
    )
  )
}
