#import "functions.typ": all_pcb_data, calc_power_consumtion, table_power_consumtion

#let name = "PMCNV-AI4W"

#pagebreak()

== #name - 4 аналоговых входа (тензодатчики) <PM-CNV_AI4-W>

Плата аналогового ввода для опроса сигналов тензодатчиков. Можно подключить до 4 тензодатчиков.


Гальваническая изоляция от микроконтроллера реализована с помощью 2 микросхем:
- CA-IS3105W - DC / DC преобразователь для цепей питания.
- CA-IS3741HW - преобразователь сигналов.

Схемотехника платы разработана на основе примера отладочной платы от производителя - @CN0102.

=== Расчёт потребления

#let power_consumption = (
  AD7193: (1, 0, 24),
  "Светодиоды": (7, 0, 22)
)
#let pmcnv_ai4w_power = calc_power_consumtion(power_consumption)
#table_power_consumtion(values: pmcnv_ai4w_power)

#all_pcb_data(name: name)
