use ../shared.nu print_header

print_header "upload: PM-DI16-DC24sink_v0.0.2"
export def main [] {
    espflash flash ./pm_firmware/release/PM_DI16_DC24sink__v0_0_2 --monitor
}
