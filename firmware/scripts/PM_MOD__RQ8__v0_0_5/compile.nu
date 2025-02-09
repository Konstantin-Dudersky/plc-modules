use ../shared.nu print_header

print_header "compile: PM-MOD_RQ8_v0.0.5"
do {
    cd pm_mod_firmware/
    cargo build --release --bin PM_MOD__RQ8__v0_0_5

    rsync -vhra ./target/riscv32imc-esp-espidf/release/PM_MOD__RQ8__v0_0_5 ./release
}
