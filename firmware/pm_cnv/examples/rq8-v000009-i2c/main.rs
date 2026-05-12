mod config_i2c_master;
mod config_inject_periodic;
mod messages;

use std::time::Duration;

use rsiot::{
    executor::{ComponentExecutor, ComponentExecutorConfig},
    logging::{LogConfig, LogConfigFilter},
};

use crate::messages::Msg;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    LogConfig {
        // filter: LogConfigFilter::String("info,rsiot::components::cmp_linux_i2c_master=trace"),
        filter: LogConfigFilter::String("info"),
    }
    .run()?;

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        delay_publish: Duration::from_millis(100),
        fn_auth: |msg, _| Some(msg),
        fn_tokio_metrics: |_| None,
    };

    ComponentExecutor::<Msg>::new(executor_config)
        .add_cmp(config_i2c_master::cmp())
        .add_cmp(config_inject_periodic::cmp())
        .wait_result()
        .await?;

    Err(anyhow::Error::msg("Execution end"))
}
