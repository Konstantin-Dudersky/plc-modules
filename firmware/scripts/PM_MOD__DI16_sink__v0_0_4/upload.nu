use ../shared.nu print_header

print_header "upload: PM_MOD-DI16_sink-v0.0.4"
export def main [] {
    espflash flash ./pm_mod_firmware/release/PM_MOD__DI16_sink__v0_0_4 --monitor
}
