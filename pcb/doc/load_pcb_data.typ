#let image_render(
  article: "plate_name",
  render_x0y0z0: "path",
  render_x0y180z0: "path",
) = {
  [
    #set page(
      flipped: true,
    )

    === Внешний вид модуля

    #figure(
      caption: "Внешний вид модуля " + article,
      kind: image,
      table(
        columns: (50%, 50%),
        image(render_x0y0z0), image(render_x0y180z0),
      ),
    )

    #set page(
      flipped: false,
    )

  ]
}

#let image_scheme(
  article: "plate_name",
  path: "path",
) = {
  [
    #set page(
      flipped: true,
    )

    === Принципиальная электрическая схема модуля

    #figure(
      caption: "Принципиальная электрическая схема модуля " + article,
      image(path, width: 90%),
    )

    #set page(
      flipped: false,
    )

  ]
}

// Загрузить данные с перечнем элементов
#let load_bom(path) = {
  let results = csv(path, row-type: dictionary)
  let results = results.map(el => {
    let reference = el.reference.replace(",", ", ")
    let quantity = el.quantity
    let part_ipn = el.part_ipn
    let option = el.option.replace(",", ", ")
    (reference, quantity, part_ipn, option)
  })
  results.flatten()
}

// Вывести таблицу с перечнем элементов
#let table_bom(
  article: "plate_name",
  path: "bom.csv",
) = {
  let bom = load_bom(path)

  [
    === Перечень элементов

    #figure(
      caption: "Перечень элементов модуля " + article,

      table(
        columns: 4,
        table.header(
          repeat: true,
          [*Обозначение*], [*Количество*], [*IPN*], [*Опция*],
        ),
        ..bom,
      ),
    )

  ]
}

#let load_pcb_data(
  article: str,
  version: "0.1.0",
) = {
  let path = "../" + article + "-v" + version + "/export/v" + version + "/"
  let path_render_x0y0z0 = path + "render_x0y0z0.png"
  let path_render_x0y180z0 = path + "render_x0y180z0.png"
  let path_scheme = path + article + ".svg"
  let path_bom = path + article + ".BOM.csv"

  image_render(
    article: article,
    render_x0y0z0: path_render_x0y0z0,
    render_x0y180z0: path_render_x0y180z0,
  )
  image_scheme(
    article: article,
    path: path_scheme,
  )
  table_bom(
    article: article,
    path: path_bom,
  )
}
