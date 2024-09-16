#![allow(non_snake_case)]

#[tokio::main(flavor = "current_thread")]
async fn main() {
    esp_idf_svc::sys::link_patches();

    tracing_subscriber::fmt().init();

    pm_firmware::plc_modules::pm_rq8_v0_0_3::main()
        .await
        .unwrap()
}
