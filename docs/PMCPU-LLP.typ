#import "functions.typ": all_pcb_data, i2c

#let name = "PMCPU-LLP"

#let i2c0 = [I#super[2]C#sub[0]]
#let spi0 = [SPI#sub[0]]

#pagebreak()

== #name - контроллер на базе Luckfox Lyra / Luckfox Lyra Plus <PMCPU-LLP>

=== Настройка дерева устройств

Для работы периферии необходимо настроить дерево устройств.

#figure(
  caption: "Назначение сигналов выводам процессора",
  table(
    columns: (auto, auto, auto),
    table.header(
      repeat: true,
      [*Сигнал*],
      [*Rockchip matrix*],
      [*GPIO устройство; линия*],
    ),
    table.cell(colspan: 3)[#spi0 - `/dev/spidev0.0`],
    [MOSI], [RM_IO12], [`/dev/gpiochip0`; 12],
    [MISO], [RM_IO13], [`/dev/gpiochip0`; 13],
    [SCK],  [RM_IO11], [`/dev/gpiochip0`; 11],
    [CS0],  [RM_IO10], [`/dev/gpiochip0`; 10],
    [CS1],  [RM_IO9], [`/dev/gpiochip0`; 9],
    [CS2],  [RM_IO8], [`/dev/gpiochip0`; 8],
    [CS3],  [RM_IO7], [`/dev/gpiochip0`; 7],
    table.cell(colspan: 3)[#i2c0 - `/dev/i2c-0`],
    [SDA],  [RM_IO6], [`/dev/gpiochip1`; 6],
    [SCL],  [RM_IO5], [`/dev/gpiochip1`; 5],
  ),
)

Путь к файлу с настройками:
- Для Luckfox Lyra - `kernel/arch/arm/boot/dts/rk3506g-luckfox-lyra.dts`
- Для Luckfox Lyra Plus - `kernel/arch/arm/boot/dts/rk3506g-luckfox-lyra-plus.dts`

```
&spi0 {
    status = "okay";
    pinctrl-names = "default";
    pinctrl-0 = <&rm_io11_spi0_clk &rm_io12_spi0_mosi &rm_io13_spi0_miso>;
    spidev@0 {
        compatible = "rockchip,spidev";
        spi-max-frequency = <10000000>;
        reg = <0>;
    };
};

&i2c0 {
    status = "okay";
    pinctrl-names = "default";
    pinctrl-0 = <&rm_io5_i2c0_scl &rm_io6_i2c0_sda>;
};
```

#figure(
  caption: "Распиновка Luckfox Lyra Plus",
  image("images/Luckfox-Lyra-Plus-details-inter-en-d673a0a10e6ce4ce348bc759f27d844e.jpg"),
) <luckfox_lyra_plus_pinout>

#all_pcb_data(name: name)
