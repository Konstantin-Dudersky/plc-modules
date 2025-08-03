#import "functions.typ": all_pcb_data, table_calc_power

#let name = "PMCNV-CountX"

#pagebreak()

== #name - X энкодеров <PMCNV-CountX>

=== Подключение энкодеров разных типов

Поддерживается подключение энкодеров разных следующих типов @encoder_scheme:

- выход по напряжению (Voltage Output) - @voltage_output[раздел]
- выход с открытым коллектором (Open Collector) - @open_collector[раздел]
- двухтактный (каскадный, комплементарный) выход (Push–Pull, Totem Pole) - @push_pull[раздел]
- (TODO) дифференциальный выход (Line Driver, RS-422) - @line_driver[раздел]


==== Выход по напряжению (Voltage Output) <voltage_output>


==== Выход с открытым коллектором (Open Collector) <open_collector>

#figure(
  caption: "Функциональная схема платы",
  image("kicad_images/output/kicad_images-pmcnv_count_open_collector_pnp.svg", width: 100%),
)

Требуется внешний резистор:

#figure(
  table(
    columns: 3,
    table.header(
      repeat: true,
      [*Напряжение, [В]*],
      [*Рекомендуемый диапазон, [Ом]*],
      [*Выбранное значение, [Ом]*]
    ),

    [5], [0,1 - 0,22], [],
    [12], [0,51 - 1,3], [],
    [24], [1,8 - 3,3], []

  )

)

==== Двухтактный (каскадный, комплементарный) выход (Push–Pull, Totem Pole) <push_pull>


==== (TODO) Дифференциальный выход (Line Driver, RS-422) <line_driver>
