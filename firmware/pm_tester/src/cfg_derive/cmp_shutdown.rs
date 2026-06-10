use rsiot::components::cmp_derive::*;

use crate::msg::*;

pub fn cmp_shutdown() -> Cmp<Msg, Buffer> {
    let config = Config {
        output_send: ConfigOutputSend::OnEveryChange,
        fn_input: |msg, buffer: &Buffer| {
            let buffer = match msg {
                Msg::MsgFilesystem(MsgFilesystem::Settings(_)) => Buffer {
                    msg_received: buffer.msg_received + 1,
                },

                _ => return None,
            };
            Some(buffer)
        },
        fn_output: |buffer| Msg::MsgDerive(MsgDerive::NeedShutdown(buffer.msg_received >= 2)),
    };

    Cmp::new(config)
}

#[derive(Clone, Default)]
pub struct Buffer {
    msg_received: usize,
}

impl BufferBound for Buffer {}
