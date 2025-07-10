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

#let table_calc_power(
  values: (),
  total_3V3: 0,
  total_5V: 0,
) = {

  let total_power = total_3V3 + total_5V
  let total_current_3v3 = total_3V3 / 3.3
  let total_current_5V = total_5V / 5

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

      ..for c in values {(
        [#c],
      )},

      table.cell(colspan: 4, align: left)[Итого, мВт: #decimal_sep(total_power)],
      [#decimal_sep(total_3V3)],
      [#decimal_sep(total_5V)],

      table.cell(colspan: 4, align: left)[Итого, мА],
      [#decimal_sep(total_current_3v3)],
      [#decimal_sep(total_current_5V)],
    )
  )
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
