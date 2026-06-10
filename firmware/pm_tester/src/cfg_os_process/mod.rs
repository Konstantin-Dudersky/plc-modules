use rsiot::components::cmp_os_process::*;

use crate::msg::*;

pub fn cmp() -> Cmp<Msg> {
    let config = Config {
        commands: vec![ConfigCommand {
            fn_input: |msg| {
                if let Msg::MsgInjPeriodic(MsgInjPeriodic::I2cDetect(_)) = msg {
                    let cmds = vec!["i2cdetect -a -y 0".to_string()];
                    return Some(cmds);
                }

                None
            },
            fn_output: |results| {
                if let Some(result) = results.iter().next() {
                    let res = parse_i2cdetect(&result.stdout);
                    return Some(vec![Msg::MsgOsProcess(MsgOsProcess::I2cDetect(res))]);
                }
                None
            },
        }],
    };

    Cmp::new(config)
}

fn parse_i2cdetect(v: &str) -> Vec<usize> {
    v
        // разбиваем на строки
        .split("\n")
        // пропускаем заголовок
        .skip(1)
        .flat_map(|l| l.split(" ").skip(1).collect::<Vec<_>>())
        // фильтруем пустые значения - получаются справа от таблицы
        .filter(|l| !l.is_empty())
        // добавляем индексы
        .enumerate()
        // находим индексы, где значение не "--"
        .filter_map(|(index, v)| if v == "--" { None } else { Some(index) })
        .collect::<Vec<_>>()
}
