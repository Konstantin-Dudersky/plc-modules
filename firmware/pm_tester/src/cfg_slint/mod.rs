use std::time::Duration;

use rsiot::{components::cmp_slint::*, executor::Component};
use slint::{ComponentHandle, SharedString, VecModel, Weak};

use crate::msg::*;

slint::include_modules!();

pub fn cmp(slint_window: Weak<MainWindow>) -> Component<Config<Msg, MainWindow>, Msg> {
    let config = Config {
        slint_window,
        fn_input: |msg, w| {
            let input = w.global::<Input>();

            match msg {
                Msg::MsgDerive(msg) => match msg {
                    MsgDerive::Watchdog {
                        watchdog_enabled,
                        watchdog_signal,
                    } => {
                        input.set_watchdog_enabled(watchdog_enabled);

                        let watchdog_signal = if watchdog_signal { "1" } else { "0" };
                        input.set_watchdog_signal(SharedString::from(watchdog_signal));
                    }

                    MsgDerive::NeedShutdown(_) => (),
                },

                Msg::MsgInjPeriodic(_) => (),

                Msg::MsgOsProcess(msg) => match msg {
                    MsgOsProcess::I2cDetect(v) => {
                        let v = v
                            .iter()
                            .map(|v| format!("0x{:x} ({})", v, v))
                            .collect::<Vec<_>>()
                            .join(",");
                        let v = SharedString::from(v);
                        input.set_i2cdetect(v);
                    }
                },

                Msg::MsgSlint(_) => (),

                Msg::MsgFilesystem(msg) => match msg {
                    MsgFilesystem::Settings(v) => {
                        let module = match v.module {
                            crate::modules::Module::None => Module::None,
                            crate::modules::Module::PMCNV_DIx16_v000010 => {
                                Module::PMCNVDIx16V000010
                            }
                        };

                        input.set_module(module);
                        input.set_pmcnv_dix16_address(v.pmcnv_dix16_v000010_address as i32);
                    }
                },

                Msg::MsgI2c(msg) => match msg {
                    MsgI2c::Diag(v) => {
                        input.set_i2c_diag_response_ok(v.response_ok_count as i32);
                        input.set_i2c_diag_response_error(v.response_err_count as i32);
                    }

                    MsgI2c::PMCNV_DIx16_data(v) => {
                        let data = vec![
                            v.ch00, v.ch01, v.ch02, v.ch03, v.ch04, v.ch05, v.ch06, v.ch07, v.ch08,
                            v.ch09, v.ch10, v.ch11, v.ch12, v.ch13, v.ch14, v.ch15,
                        ];
                        let data = VecModel::from_slice(&data);
                        input.set_pmcnv_dix16_state(data);
                    }
                },

                Msg::MsgInjectSingle(_) => (),
            }
        },

        fn_output: |w, sender| {
            let output = w.global::<Output>();

            let sender_clone = sender.clone();
            output.on_set_watchdog(move |v| {
                let msg = Msg::MsgSlint(MsgSlint::SetWatchdogEnabled(v));
                sender_clone.send(msg);
            });

            let sender_clone = sender.clone();
            output.on_change_active_module(move |m| {
                let m = match m {
                    Module::None => crate::modules::Module::None,
                    Module::PMCNVDIx16V000010 => crate::modules::Module::PMCNV_DIx16_v000010,
                    Module::PMCNVDIx32V000010 => crate::modules::Module::PMCNV_DIx16_v000010,
                };
                let msg = Msg::MsgSlint(MsgSlint::ChangeActiveModule(m));
                sender_clone.send(msg);
            });

            let sender_clone = sender.clone();
            output.on_pmcnv_dix16_set_address(move |address| {
                let address = address as u8;
                let msg = Msg::MsgSlint(MsgSlint::PmcnvDix16SetAddress(address));
                sender_clone.send(msg);
            });
        },
        output_period: Duration::from_millis(100),
    };

    Cmp::new(config)
}
