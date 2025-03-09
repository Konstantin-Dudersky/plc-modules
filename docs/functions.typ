#let image_render(
  name: "plate_name",
  render_x0y0z0: "path",
  render_x0y180z0: "path",
) = {
  set page(
    paper: "a4",
    flipped: true,
    margin: 1cm,
  )

  figure(
    caption: "Внешний вид платы " + name,
    table(
      columns: (50%, 50%),
      stroke: none,
      gutter: 1em,
      image(render_x0y0z0), image(render_x0y180z0),
    ),
  )

  set page(
    paper: "a4",
    flipped: false,
    margin: auto,
  )
}

#let image_scheme(
  name: "plate_name",
  path: "path",
) = {
  set page(
    paper: "a4",
    flipped: true,
    margin: 1cm,
  )

  figure(
    caption: "Принципиальная электрическая схема платы " + name,
    image(path, width: 95%),
  )
  set page(
    paper: "a4",
    flipped: false,
    margin: auto,
  )
}

// Загрузить данные с перечнем элементов
#let load_bom(path) = {
  let results = csv(path, row-type: dictionary)
  let results = results.map(el => (el.reference, el.quantity, el.part_ipn))
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
      columns: 3,
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
  render_x0y0z0: "path",
  render_x0y180z0: "path",
  scheme: "path",
  bom: "bom.csv",
) = {
  image_render(
    name: name,
    render_x0y0z0: render_x0y0z0,
    render_x0y180z0: render_x0y180z0,
  )
  image_scheme(
    name: name,
    path: scheme,
  )
  table_bom(
    name: name,
    path: bom,
  )
}
