use ../shared.nu print_header

print_header "upload: PM-MOD_RQ16_v0.0.5"
export def main [] {
    espflash flash ./pm_mod_firmware/release/PM_MOD__RQ16__v0_0_5 --monitor
}
