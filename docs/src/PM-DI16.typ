= PM-DI16 <PM-DI16>

Модуль для подключения 16 дискретных входов постоянного напряжения. Уровень напряжения зависит от используемой платы PM-DI16-front-XXX.

Входы гальванически изолированы от внутреннего источника питания.

Схема модуля собрана на базе микросхемы PCA9555. Адрес на шине I#super[2]С задается с помощью трех перемычек на плате.

#figure(
  caption: "Внешний вид PM-DI16",
  table(
    columns: (auto, auto),
    stroke: none,
    image("../../PM-DI16/doc/PM-DI16-base_view.png", fit: "contain"),
    image("../../PM-DI16/doc/PM-DI16-base_view_y90.png", fit: "contain"),
    image("../../PM-DI16/doc/PM-DI16-base_view_x90.png", fit: "contain"),
    image("../../PM-DI16/doc/PM-DI16-base_view_y180.png", fit: "contain"),
  )
)

#figure(
    caption: "Схема внешних подключений PM-DI16",
    image("../../PM-DI16/doc/PM-DI16-base-external_connection.svg")
)
