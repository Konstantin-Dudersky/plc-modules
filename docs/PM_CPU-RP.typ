#import "functions.typ": all_pcb_data

#let name = "PM_CPU-RP"
#let path_render_x0y0z0 = "pcb/" + name + "/render_x0y0z0.png"
#let path_render_x0y180z0 = "pcb/" + name + "/render_x0y180z0.png"
#let path_scheme = "pcb/" + name + "/" + name + ".svg"
#let file_bom = "./pcb/" + name + "/BOM.csv"


== #name - контроллер на базе Raspberry Pi <PM-CPU-RP>

ЦПУ на базе мини-компьютера Raspberry Pi, или совместимого по габаритам, креплению и 40-пиновому штекеру.

Программировать можно практически на всех языках программирования, поддерживающих архитектуру процессора ARM64.

#all_pcb_data(
  name: name,
  render_x0y0z0: path_render_x0y0z0,
  render_x0y180z0: path_render_x0y180z0,
  scheme: path_scheme,
  bom: file_bom,
)
