#import "functions.typ": all_pcb_data

#let name = "PMCNV-AI4T"

#pagebreak()

== #name - 4 аналоговых входа (термопары) <PMCNV-AI4T>

Измерение построено на базе АЦП AD7193. Производитель разработал пример для измерения температуры с помощью термопар - CN0287 @CN0287.

=== Преобразование измеренного напряжения в температуру

Значение температуры можно получить по градуировочным таблицам ГОСТ 8.585-2001 @GOST-8.585-2001.

=== Компенсация холодного спая

#figure(
  caption: "Компенсация холодного спая",
  image("images/cold_junction.png", width: 100%),
) <cold_junction>

As shown in Figure 1, the thermocouple has a “hot” (measuring) end and two “cold” connections. During the
measuring process, both “hot” and “cold” contacts generate voltage. So, to accurately measure the temperature
at the “hot” end, the cold junction voltage must be considered.

The cold junction voltage is generated in two points of contact: metal 1 with copper and metal 2 with copper.
Instead of copper any metal or alloy can be used, the thermocouple’s cold ends can be connected to the voltmeter
via gold-plated contacts or soldered directly to the PCB. It doesn’t matter, as long as both contacts are made of
the same metal, they can be eliminated thus cold junction voltage is the same as between metal 1 and metal 2.

By its nature, a thermocouple junction does not generate any thermovoltage when it is at 0°C temperature, see
Figure 2. So, if the cold junction is placed in an electrically isolated ice bath or an accurate temperature block of
0°C, no compensation is needed. But this is not practical. The best way to compensate for the thermocouple cold
junction is by measuring the temperature of the cold contacts (using a separate temp sensor), and converting that
temperature into the thermocouple voltage, then using this value and the voltmeter readings to calculate the
temperature at the hot end. See the example below

@AN-CM-389

Напряжение, которое создается на термпаре (горячий спай):

$ E_h = E_"ADC" + E_c $

где:
- $E_h$ - напряжение, создаваемое горячим спаем ("hot")
- $E_c$ - напряжение, создаваемое холодным спаем ("cold")
- $E_"ADC"$ - напряжение, которое измеряет АЦП

Например:
+ АЦП измерил напряжение $E_"ADC" = 19792 "мкВ"$
+ Без компенсации по таблицам @GOST-8.585-2001 температура горячего спая: $480 "℃"$
+ Показания датчика температуры холодного спая: $E_c = 20 ℃$
+ По таблицам @GOST-8.585-2001 определям $E_c = 798 "мкВ"$
+ Определяем напряжение горячего спая $E_h = 19792 + 798 = 20590 "мкВ"$
+ По таблицам @GOST-8.585-2001 определяем температуру горячего спая с компенсацией: $498 "℃"$.

=== Напряжение смещения

#figure(
  caption: "Напряжение смещения",
  image("images/PMCNV-AI4T_bias_generator.png", width: 100%),
) <bias_generator>

=== Цепи защиты

#figure(
  caption: "Цепи защиты",
  image("images/PMCNV-AI4T_protection_circuit.png", width: 100%),
) <bias_generator>

#all_pcb_data(name: name)
