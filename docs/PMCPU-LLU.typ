#import "functions.typ": all_pcb_data, i2c

#let name = "PMCPU-LLU"

#let i2c0 = [I#super[2]C#sub[0]]
#let spi0 = [SPI#sub[0]]

#pagebreak()

== #name - контроллер на базе Luckfox Lyra Ultra <PMCPU-LLU>

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
    [MOSI], [RM_IO8], [`/dev/gpiochip0`; 8],
    [MISO], [RM_IO7], [`/dev/gpiochip0`; 7],
    [SCK],  [RM_IO6], [`/dev/gpiochip0`; 6],
    [CS0],  [RM_IO5], [`/dev/gpiochip0`; 5],
    [CS1],  [RM_IO4], [`/dev/gpiochip0`; 4],
    [CS2],  [RM_IO3], [`/dev/gpiochip0`; 3],
    [CS3],  [RM_IO2], [`/dev/gpiochip0`; 2],
    table.cell(colspan: 3)[#i2c0 - `/dev/i2c-0`],
    [SDA],  [RM_IO25], [`/dev/gpiochip1`; 10],
    [SCL],  [RM_IO24], [`/dev/gpiochip1`; 9],
  ),
)

Путь к файлу с настройками: `kernel/arch/arm/boot/dts/rk3506b-luckfox-lyra-ultra-w.dts`

```
&spi0 {
    status = "okay";
    pinctrl-names = "default";
    pinctrl-0 = <&rm_io6_spi0_clk &rm_io8_spi0_mosi &rm_io7_spi0_miso>;
    spidev@0 {
        compatible = "rockchip,spidev";
        spi-max-frequency = <10000000>;
        reg = <0>;
    };
};

&i2c0 {
    status = "okay";
    pinctrl-names = "default";
    pinctrl-0 = <&rm_io24_i2c0_scl &rm_io25_i2c0_sda>;
};
```

В системе появятся адаптеры `/dev/spidev0.0` и `/dev/i2c-0`.

TODO - uart для связи с модулями

Настройка экрана: #link("https://wiki.luckfox.com/Luckfox-Lyra/SDK#6-device-tree-configuration")

#all_pcb_data(name: name)
