//! cargo build --target="aarch64-unknown-linux-gnu" --example "pmcnv-ai4w-v000002" --release; scp target/aarch64-unknown-linux-gnu/release/examples/pmcnv-ai4w-v000002 user@target:/home/user/
//!
//! cargo build --target="armv7-unknown-linux-gnueabihf" --example "pmcnv-ai4w-v000002" --release; scp target/armv7-unknown-linux-gnueabihf/release/examples/pmcnv-ai4w-v000002 root@target:/root

mod config_inject_periodic;
// mod config_logger;
mod config_spi_master;
mod messages;

use std::time::Duration;

use rsiot::{
    executor::{ComponentExecutor, ComponentExecutorConfig},
    logging::configure_logging,
};

#[tokio::main]
async fn main() {
    configure_logging("info", None).await.unwrap();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        delay_publish: Duration::from_millis(100),
        fn_auth: |msg, _| Some(msg),
    };

    ComponentExecutor::<messages::Msg>::new(executor_config)
        .add_cmp(config_spi_master::cmp())
        .add_cmp(config_inject_periodic::cmp())
        // .add_cmp(config_logger::cmp())
        .wait_result()
        .await
        .unwrap();
}
