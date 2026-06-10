use rsiot::components::cmp_derive::*;

use crate::msg::*;

pub fn cmp_watchdog() -> Cmp<Msg, Buffer> {
    let config = Config {
        output_send: ConfigOutputSend::OnEveryChange,
        fn_input: |msg, buffer| {
            let buffer = match msg {
                Msg::MsgSlint(MsgSlint::SetWatchdogEnabled(v)) => Buffer {
                    watchdog_enabled: *v,
                    ..*buffer
                },

                Msg::MsgInjPeriodic(MsgInjPeriodic::Watchdog(v)) => Buffer {
                    cycle_signal: *v,
                    ..*buffer
                },

                Msg::MsgInjectSingle(MsgInjectSingle::Start(_)) => Buffer {
                    watchdog_enabled: true,
                    ..*buffer
                },

                _ => return None,
            };
            Some(buffer)
        },
        fn_output: |buffer| {
            let watchdog_signal = buffer.cycle_signal && buffer.watchdog_enabled;
            Msg::MsgDerive(MsgDerive::Watchdog {
                watchdog_enabled: buffer.watchdog_enabled,
                watchdog_signal,
            })
        },
    };

    Cmp::new(config)
}

#[derive(Clone, Default)]
pub struct Buffer {
    cycle_signal: bool,
    watchdog_enabled: bool,
}

impl BufferBound for Buffer {}
