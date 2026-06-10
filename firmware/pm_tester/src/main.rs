use std::{
    fs::File,
    io::{Read, Write},
    time::Duration,
};

use rsiot::{
    executor::{ComponentExecutor, ComponentExecutorConfig},
    logging::{LogConfig, LogConfigFilter},
};
use slint::{ComponentHandle, Weak};
use tokio::{main, time::sleep};

mod cfg_derive;
mod cfg_filesystem;
mod cfg_inject_periodic;
mod cfg_inject_single;
mod cfg_linux_gpio;
mod cfg_linux_i2c_master;
mod cfg_os_process;
mod cfg_shutdown;
mod cfg_slint;
mod modules;
mod msg;

use msg::*;
use tracing::info;

pub const CONFIG_FILE: &str = "config.toml";

fn main() -> anyhow::Result<()> {
    let main_window = cfg_slint::MainWindow::new()?;
    let main_window_link = main_window.as_weak();

    std::thread::spawn(move || main_executor(main_window_link));

    main_window.run()?;

    Err(anyhow::Error::msg("Program stop"))
}

#[main]
async fn main_executor(slint_window: Weak<cfg_slint::MainWindow>) -> anyhow::Result<()> {
    LogConfig {
        filter: LogConfigFilter::String("info"),
    }
    .run()?;

    loop {
        let settings = load_settings();

        info!("Config loaded: {settings:?}");

        let executor_config = ComponentExecutorConfig::<Msg> {
            buffer_size: 1000,
            fn_auth: |msg, _| Some(msg),
            delay_publish: Duration::from_millis(100),
            fn_tokio_metrics: |_| None,
        };

        let ce = ComponentExecutor::new(executor_config)
            .add_cmp(cfg_derive::cmp_shutdown())
            .add_cmp(cfg_derive::cmp_watchdog())
            .add_cmp(cfg_filesystem::cmp())
            .add_cmp(cfg_inject_periodic::cmp_i2cdetect())
            .add_cmp(cfg_inject_periodic::cmp_watchdog())
            .add_cmp(cfg_inject_single::cmp())
            .add_cmp(cfg_linux_gpio::cmp())
            .add_cmp(cfg_os_process::cmp())
            .add_cmp(cfg_shutdown::cmp())
            .add_cmp(cfg_slint::cmp(slint_window.clone()));

        let ce = if !matches!(settings.module, crate::modules::Module::None) {
            ce.add_cmp(cfg_linux_i2c_master::cmp(settings))
        } else {
            ce
        };

        ce.wait_result().await?;

        sleep(Duration::from_millis(200)).await
    }
}

fn load_settings() -> cfg_filesystem::Buffer {
    let file = File::open(CONFIG_FILE);

    let mut file = match file {
        Ok(file) => file,
        Err(_) => {
            return save_default_settings();
        }
    };

    let mut config_str = String::new();

    file.read_to_string(&mut config_str).unwrap();

    let settings = toml::from_str::<cfg_filesystem::Buffer>(&config_str);

    match settings {
        Ok(v) => v,
        Err(_) => save_default_settings(),
    }
}

fn save_default_settings() -> cfg_filesystem::Buffer {
    let settings = cfg_filesystem::Buffer::default();
    let bytes = toml::to_string(&settings).unwrap();

    let _ = File::create(CONFIG_FILE)
        .unwrap()
        .write(bytes.as_bytes())
        .unwrap();

    settings
}
