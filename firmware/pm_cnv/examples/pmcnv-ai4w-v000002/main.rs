//! cargo build --target="aarch64-unknown-linux-gnu" --example "pmcnv-ai4w-v000002" --release; scp target/aarch64-unknown-linux-gnu/release/examples/pmcnv-ai4w-v000002 user@target:/home/user/
//!
//! cargo build --target="armv7-unknown-linux-gnueabihf" --example "pmcnv-ai4w-v000002" --release; scp target/armv7-unknown-linux-gnueabihf/release/examples/pmcnv-ai4w-v000002 root@target:/root

mod cfg_inject_periodic;
// mod config_logger;
mod cfg_spi_master;
mod msg;

use std::time::Duration;

use rsiot::{
    executor::{ComponentExecutor, ComponentExecutorConfig},
    logging::{LogConfig, LogConfigFilter},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    LogConfig {
        filter: LogConfigFilter::String("info"),
    }
    .run()?;

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        delay_publish: Duration::from_millis(100),
        fn_auth: |msg, _| Some(msg),
        fn_tokio_metrics: |_| None,
    };

    ComponentExecutor::<msg::Msg>::new(executor_config)
        .add_cmp(cfg_spi_master::cmp())
        .add_cmp(cfg_inject_periodic::cmp())
        // .add_cmp(config_logger::cmp())
        .wait_result()
        .await?;

    Err(anyhow::Error::msg("Execution end"))
}
