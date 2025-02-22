#![allow(non_snake_case)]

#[tokio::main(flavor = "current_thread")]
async fn main() {
    esp_idf_svc::sys::link_patches();

    tracing_subscriber::fmt().init();

    pm_mod_firmware::plc_modules::pm_mod__rq16__v0_0_5::main()
        .await
        .unwrap()
}
