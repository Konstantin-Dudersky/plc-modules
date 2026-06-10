#import "functions.typ": all_pcb_data

#let name = "PMCNV-INA226"

#pagebreak()

== #name - Контроль электрических параметров с помощью INA226 <PMCNV-INA226>


$ I_"max" = V_"shunt" / R_"shunt" $

$V_"shunt"$ = 81.92 мВ

#figure(
  table(
    columns: (50%, 50%),

    [$R_"shunt"$ - сопротивление шунта, \[мОм\]], [$I_"max"$ - макс. ток, \[А\]],
    [10], [8,192],
    [50], [1,6384],
  )
)
