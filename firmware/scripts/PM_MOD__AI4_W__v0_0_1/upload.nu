use ../shared.nu print_header

print_header "upload: PM_MOD-AI4_W-v0.0.1"
export def main [] {
    espflash flash ./pm_mod_firmware/release/PM_MOD__AI4_W__v0_0_1 --monitor
}
