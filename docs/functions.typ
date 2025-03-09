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
      columns: (30%, 20%, 50%),
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
