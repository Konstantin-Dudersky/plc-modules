```sh
cargo build --target="armv7-unknown-linux-gnueabihf" --example "PMCMV_INA226x4_v000001" --release; scp target/armv7-unknown-linux-gnueabihf/release/examples/PMCMV_INA226x4_v000001 root@target:/root
```
