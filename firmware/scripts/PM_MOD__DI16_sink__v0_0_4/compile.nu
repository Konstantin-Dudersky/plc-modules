use ../shared.nu print_header

print_header "compile: PM_MOD-DI16_sink-v0.0.4"
do {
    cd pm_mod_firmware/
    cargo build --release --bin PM_MOD__DI16_sink__v0_0_4

    rsync -vhra ./target/riscv32imc-esp-espidf/release/PM_MOD__DI16_sink__v0_0_4 ./release
}
