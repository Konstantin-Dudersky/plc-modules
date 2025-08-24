```sh
cargo build --target="aarch64-unknown-linux-gnu" --example "pmcnv-dq16src-v000002" --release; scp target/aarch64-unknown-linux-gnu/release/examples/pmcnv-dq16src-v000002 user@target:/home/user/

cargo build --target="armv7-unknown-linux-gnueabihf" --example "dq16src-v000003-i2c" --release; scp target/armv7-unknown-linux-gnueabihf/release/examples/dq16src-v000003-i2c root@target:/root
```
