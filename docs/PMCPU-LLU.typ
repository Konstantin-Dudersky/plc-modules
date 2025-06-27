#import "functions.typ": all_pcb_data, i2c

#let name = "PMCPU-LLU"

#pagebreak()

== #name - контроллер на базе Luckfox Lyra Ultra <PMCPU-LLU>

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
```

В системе появится адаптер `/dev/spidev0.0`.


#figure(
  caption: "Сигналы Chip Select",
  table(
    columns: (auto, auto, auto, auto),
    table.header(
      repeat: true,
      [*Сигнал*],
      [*Rockchip matrix*],
      [*GPIO устройство*],
      [*GPIO линия*],
    ),
    [MOSI], [RM_IO8], [`/dev/gpiochip0`], [8],
    [MISO], [RM_IO7], [`/dev/gpiochip0`], [7],
    [SCK],  [RM_IO6], [`/dev/gpiochip0`], [6],
    [CS0],  [RM_IO5], [`/dev/gpiochip0`], [5],
    [CS1],  [RM_IO4], [`/dev/gpiochip0`], [4],
    [CS2],  [RM_IO3], [`/dev/gpiochip0`], [3],
    [CS3],  [RM_IO2], [`/dev/gpiochip0`], [2],
  ),
)


```
&i2c0 {
    status = "okay";
    pinctrl-names = "default";
    pinctrl-0 = <&rm_io24_i2c0_scl &rm_io25_i2c0_sda>;
};
```

#all_pcb_data(name: name)
