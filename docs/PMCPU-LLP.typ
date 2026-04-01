#import "functions.typ": all_pcb_data, i2c, listing

#let name = "PMCPU-LLP"

#let i2c0 = [I#super[2]C#sub[0]]
#let i2c1 = [I#super[2]C#sub[1]]
#let i2c2 = [I#super[2]C#sub[2]]
#let spi0 = [SPI#sub[0]]
#let can0 = [CAN#sub[0]]
#let can1 = [CAN#sub[1]]
#let uart1 = [UART#sub[1]]
#let uart2 = [UART#sub[2]]

#pagebreak()

== #name - контроллер на базе Luckfox Lyra / Luckfox Lyra Plus <PMCPU-LLP>

=== Настройка дерева устройств

Для работы периферии необходимо настроить дерево устройств.

#figure(
  caption: "Назначение сигналов выводам процессора",
  table(
    columns: (30%, 30%, 40%),
    table.header(
      repeat: true,
      [*Сигнал*],
      [*Rockchip matrix*],
      [*GPIO устройство; линия*],
    ),

    table.cell(colspan: 3)[#spi0],
    [MOSI], [RM_IO11], [`/dev/gpiochip0`; 11],
    [MISO], [RM_IO10], [`/dev/gpiochip0`; 10],
    [SCK],  [RM_IO9], [`/dev/gpiochip0`; 9],
    [CS0],  [RM_IO8], [`/dev/gpiochip0`; 8],
    [CS1],  [RM_IO7], [`/dev/gpiochip0`; 7],
    [CS2],  [RM_IO6], [`/dev/gpiochip0`; 6],
    [CS3],  [RM_IO5], [`/dev/gpiochip0`; 5],

    table.cell(colspan: 3)[#i2c0],
    [SDA],  [RM_IO12], [`/dev/gpiochip0`; 12],
    [SCL],  [RM_IO13], [`/dev/gpiochip0`; 13],

    table.cell(colspan: 3)[#i2c1],
    [SDA],  [RM_IO22], [`/dev/gpiochip0`; 22],
    [SCL],  [RM_IO23], [`/dev/gpiochip0`; 23],

    table.cell(colspan: 3)[#i2c2],
    [SDA],  [RM_IO0], [`/dev/gpiochip0`; 0],
    [SCL],  [RM_IO1], [`/dev/gpiochip0`; 1],

    table.cell(colspan: 3)[#can0],
    [TX],  [RM_IO3], [`/dev/gpiochip0`; 3],
    [RX],  [RM_IO2], [`/dev/gpiochip0`; 2],

    table.cell(colspan: 3)[#can1],
    [TX],  [RM_IO24], [`/dev/gpiochip1`; 9],
    [RX],  [RM_IO25], [`/dev/gpiochip1`; 10],

    table.cell(colspan: 3)[#uart1],
    [TX],  [RM_IO31], [`/dev/gpiochip1`; 27],
    [RX],  [RM_IO29], [`/dev/gpiochip1`; 25],
    [RTS],  [RM_IO30], [`/dev/gpiochip1`; 26],

    table.cell(colspan: 3)[#uart2],
    [TX],  [RM_IO28], [`/dev/gpiochip1`; 19],
    [RX],  [RM_IO26], [`/dev/gpiochip1`; 11],
    [RTS],  [RM_IO27], [`/dev/gpiochip1`; 18],

    table.cell(colspan: 3)[Пины GPIO],
    [WDT (watchdog)], [RM_IO4], [`/dev/gpiochip0`; 4],
    [LED0], [ADC3], [`/dev/gpiochip4`; 11],
    [LED1], [ADC2], [`/dev/gpiochip4`; 10],
  ),
)

#figure(
  caption: "Описание интерфейсов",
  table(
    columns: 3,
    table.header(
      repeat: true,
      [*Интерфейс*],
      [*Путь в системе*],
      [*Описание*],
    ),

    [#spi0],
    [`/dev/spidev0.0`],
    [Шина IBus],

    [#i2c0],
    [`/dev/i2c-0`],
    [Шина IBus],

    [#i2c1],
    [`/dev/i2c-1`],
    [Подключение кнопок, RTC],

    [#i2c2],
    [`/dev/i2c-2`],
    [Настроен в прошивке по умолчанию, нужен для работы тач-контроллера],

    [#can0],
    [`/dev/can0`],
    [Шина EBus],

    [#can1],
    [`/dev/can1`],
    [Внешний интерфейс CAN],

    [#uart1],
    [`/dev/ttyS1`],
    [Внешний интерфейс UART],

    [#uart2],
    [`/dev/ttyS2`],
    [Внешний интерфейс UART],
  )
)


Путь к файлу с настройками:
`kernel/arch/arm/boot/dts/rk3506-luckfox-lyra.dtsi`

#let code = ```
&spi0 {
    status = "okay";
    pinctrl-names = "default";
    pinctrl-0 = <&rm_io9_spi0_clk &rm_io11_spi0_mosi &rm_io10_spi0_miso>;
    spidev@0 {
        compatible = "rockchip,spidev";
        spi-max-frequency = <10000000>;
        reg = <0>;
    };
};

&i2c0 {
    status = "okay";
    pinctrl-names = "default";
    pinctrl-0 = <&rm_io13_i2c0_scl &rm_io12_i2c0_sda>;
};

&i2c1 {
    status = "okay";
    pinctrl-names = "default";
    pinctrl-0 = <&rm_io23_i2c1_scl &rm_io22_i2c1_sda>;
};

&can0 {
    assigned-clocks = <&cru CLK_CAN0>;
    assigned-clock-rates = <200000000>;
    pinctrl-names = "default";
    pinctrl-0 = <&rm_io3_can0_tx &rm_io2_can0_rx>;
    status = "okay";
};

&can1 {
    assigned-clocks = <&cru CLK_CAN1>;
    assigned-clock-rates = <200000000>;
    pinctrl-names = "default";
    pinctrl-0 = <&rm_io24_can1_tx &rm_io25_can1_rx>;
    status = "okay";
};

&uart1 {
    pinctrl-0 = <&rm_io31_uart1_tx &rm_io29_uart1_rx>;
    status = "okay";
    pinctrl-names = "default";
};

&uart2 {
    pinctrl-0 = <&rm_io28_uart2_tx &rm_io26_uart2_rx>;
    status = "okay";
    pinctrl-names = "default";
};
```

#listing(
  content: code,
  caption: "Настройка дерева устройств Linux",
  label: <device_tree_llp>, breakable: false)

#figure(
  caption: "Распиновка Luckfox Lyra Plus",
  image("images/Luckfox-Lyra-Plus-details-inter-en-d673a0a10e6ce4ce348bc759f27d844e.jpg"),
) <luckfox_lyra_plus_pinout>

#all_pcb_data(name: name)
