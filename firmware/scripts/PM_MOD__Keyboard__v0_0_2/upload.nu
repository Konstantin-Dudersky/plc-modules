use ../shared.nu print_header

print_header "upload: PM_MOD__Keyboard__v0_0_2"
export def main [] {
    espflash flash ./pm_mod_firmware/release/PM_MOD__Keyboard__v0_0_2 --monitor
}
