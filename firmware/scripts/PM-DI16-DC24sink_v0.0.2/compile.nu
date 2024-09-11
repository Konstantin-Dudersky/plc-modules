use ../shared.nu print_header

print_header "compile: PM-DI16-DC24sink_v0.0.2"
do {
    cd pm_firmware/
    cargo build --release --bin PM_DI16_DC24sink__v0_0_2

    rsync -vhra ./target/riscv32imc-esp-espidf/release/PM_DI16_DC24sink__v0_0_2 ./release
}
