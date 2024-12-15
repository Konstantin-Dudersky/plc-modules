use ../shared.nu print_header

print_header "compile: PM-RQ8_v0.0.4"
do {
    cd pm_firmware/
    cargo build --release --bin PM_RQ8__v0_0_4

    rsync -vhra ./target/riscv32imc-esp-espidf/release/PM_RQ8__v0_0_4 ./release
}
