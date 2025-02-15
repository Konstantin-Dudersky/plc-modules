use ../shared.nu print_header

print_header "compile: PM_MOD-AI4_W-v0.0.1"
do {
    cd pm_mod_firmware/
    cargo build --release --bin PM_MOD__AI4_W__v0_0_1

    rsync -vhra ./target/riscv32imc-esp-espidf/release/PM_MOD__AI4_W__v0_0_1 ./release
}
