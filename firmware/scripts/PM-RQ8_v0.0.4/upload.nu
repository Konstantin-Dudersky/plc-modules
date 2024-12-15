use ../shared.nu print_header

print_header "upload: PM-RQ8_v0.0.4"
export def main [] {
    espflash flash ./pm_firmware/release/PM_RQ8__v0_0_4 --monitor
}
