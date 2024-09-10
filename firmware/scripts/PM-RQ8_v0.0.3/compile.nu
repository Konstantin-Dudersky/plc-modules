use ../shared.nu print_header

print_header "compile: PM-RQ8_v0.0.3"
do {
    cargo build --release --bin PM_RQ8__v0_0_3

    rsync -vhra ./target/riscv32imc-esp-espidf/release/PM_RQ8__v0_0_3 ./release
}
