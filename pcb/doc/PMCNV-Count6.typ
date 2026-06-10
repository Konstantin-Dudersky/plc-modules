#import "functions.typ": all_pcb_data

#let name = "PMCNV-Count6"

#pagebreak()

== #name - подключение 6 сигналов быстрого счёта <PMCNV-Count6>


=== Подключение энкодеров разных типов

Поддерживается подключение энкодеров разных следующих типов @encoder_scheme:

- выход с открытым коллектором (Open Collector) - @open_collector[раздел]
- двухтактный (каскадный, комплементарный) выход (Push–Pull, Totem Pole) - @push_pull[раздел]
- выход по напряжению (Voltage Output) - @voltage_output[раздел]
- (TODO) дифференциальный выход (Line Driver, RS-422) - @line_driver[раздел]


==== Выход с открытым коллектором (Open Collector) <open_collector>

На @open_collector_pnp[рисунке] отображена схема подключения энкодера с типом выхода "Открытый коллектор" по схеме PNP.

#figure(
  caption: "Подключение энкодера с выходом открытый коллектор, по схеме PNP",
  image("kicad_images/output/kicad_images-pmcnv_count_open_collector_pnp.svg", width: 100%),
) <open_collector_pnp>

На @open_collector_npn[рисунке] отображена схема подключения энкодера с типом выхода "Открытый коллектор" по схеме NPN.

#figure(
  caption: "Подключение энкодера с выходом открытый коллектор, по схеме NPN",
  image("kicad_images/output/kicad_images-pmcnv_count_open_collector_npn.svg", width: 100%),
) <open_collector_npn>

==== Двухтактный (каскадный, комплементарный) выход (Push–Pull, Totem Pole) <push_pull>

На @push_pull_schema[рисунке] отображена схема подключения энкодера с типом выхода "Открытый коллектор" по схеме NPN.

#figure(
  caption: "Подключение энкодера с двухтактным выходом, по схеме PNP",
  image("kicad_images/output/kicad_images-pmcnv_count_push_pull.svg", width: 100%),
) <push_pull_schema>


==== Выход по напряжению (Voltage Output) <voltage_output>

На @voltage_output_scheme[рисунке] отображена схема подключения энкодера с типом выхода "Выход по напряжению".

#figure(
  caption: "Подключение энкодера с выходом по напряжению",
  image("kicad_images/output/kicad_images-pmcnv_count_voltage_output.svg", width: 100%),
) <voltage_output_scheme>


==== (TODO) Дифференциальный выход (Line Driver, RS-422) <line_driver>

#all_pcb_data(name: name)
