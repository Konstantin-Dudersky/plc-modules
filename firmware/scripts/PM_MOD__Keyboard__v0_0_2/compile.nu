use ../shared.nu print_header

print_header "compile: PM_MOD__Keyboard__v0_0_2"
do {
    cd pm_mod_firmware/
    cargo build --release --bin PM_MOD__Keyboard__v0_0_2

    rsync -vhra ./target/riscv32imc-esp-espidf/release/PM_MOD__Keyboard__v0_0_2 ./release
}
