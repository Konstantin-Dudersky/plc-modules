#![allow(non_snake_case)]

#[tokio::main(flavor = "current_thread")]
async fn main() {
    esp_idf_svc::sys::link_patches();

    tracing_subscriber::fmt().init();

    // pm_firmware::plc_modules::pm_di16_dc24sink_v0_0_2::main()
    //     .await
    //     .unwrap()
}
