#import "functions.typ": all_pcb_data

#let name = "PM_CNV-DI16_sink"


== #name <PM_CNV-DI16_sink>

Модуль для подключения 16 дискретных входов DC24В. Гальваническая изоляция обеспечивается ISO1212. Микроконтроллер считывает состояние входов через расширитель GPIO MCP23S17.

#all_pcb_data(name: name)
