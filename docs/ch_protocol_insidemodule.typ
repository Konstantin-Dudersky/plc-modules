#import "functions.typ": i2c

= Протоколы обмена данными между платами внутри модуля <ch_between_plates>

Наиболее распространенными протоколами обмена между интегральными схемами являются SPI и #i2c.

#figure(
  caption: "Распиновка коннектора между платами",

  table(
    columns: 3,
    align: (center, center, left + horizon),

    [1], [GND], table.cell(rowspan: 2, [GND (Ground) - общий потенциал]),
    [2], [GND],
    [3], [+5V], table.cell(rowspan: 2, [Питание ведомых устройств напряжением +5 В]),
    [4], [+5V],
    [5], [+3.3V], table.cell(rowspan: 2, [Питание ведомых устройств напряжением +3,3 В]),
    [6], [+3.3V],
    [7], [GND], [GND (Ground) - общий потенциал],
    [8], [SPI - MOSI], [MOSI (Master Out Slave In) - передача данных от ведущего к ведомому],
    [9], [SPI - MISO], [MISO (Master In Slave Out) - передача данных от ведомого к ведущему],
    [10], [SPI - SCK], [SCK (Serial Clock) - передача тактового сигнала],
    [11], [SPI - CS0], [CS0 (Chip Select) - выбор микросхемы 0],
    [12], [SPI - CS1], [CS1 (Chip Select) - выбор микросхемы 1],
    [13], [SPI - CS2], [CS2 (Chip Select) - выбор микросхемы 2],
    [14], [SPI - CS3], [CS3 (Chip Select) - выбор микросхемы 3],
    [15], [RST], [RST (Reset) - сброс микросхемы. 0 - сброс, 1 - работа],
    [16], [GND], [GND (Ground) - общий потенциал],
    [17], [#i2c - SDA], [SDA (Serial Data) - передача данных],
    [18], [#i2c - SCL], [SCL (Serial Clock) - передача тактового сигнала],
    [19], [GND], table.cell(rowspan: 2, [GND (Ground) - общий потенциал]),
    [20], [GND],
  )
) <pinout_between_plates>
