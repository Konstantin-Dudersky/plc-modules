#import "functions.typ": all_pcb_data, i2c

#let name = "PMPS-20W"

#let i2c0 = [I#super[2]C#sub[0]]
#let i2c2 = [I#super[2]C#sub[2]]
#let spi0 = [SPI#sub[0]]

#pagebreak()

== #name - блок питания 20 Вт 230 В <PMPS-20W>

#all_pcb_data(name: name)
