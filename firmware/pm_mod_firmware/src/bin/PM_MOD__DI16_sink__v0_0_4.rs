#![allow(non_snake_case)]

#[tokio::main(flavor = "current_thread")]
async fn main() {
    esp_idf_svc::sys::link_patches();

    tracing_subscriber::fmt().init();

    pm_mod_firmware::plc_modules::pm_mod__di16_sink__v0_0_4::main()
        .await
        .unwrap()
}
