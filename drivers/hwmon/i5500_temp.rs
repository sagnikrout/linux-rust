//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/i5500_temp.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0-or-later
//
// i5500_temp - Driver for Intel 5500/5520/X58 chipset thermal sensor
//
// Copyright (C) 2012, 2014 Jean Delvare <jdelvare@suse.de>
//

// Register definitions from datasheet
pub const REG_TSTHRCATA: c_uint = 0xE2;
pub const REG_TSCTRL: c_uint = 0xE8;
pub const REG_TSTHRRPEX: c_uint = 0xEB;
pub const REG_TSTHRLO: c_uint = 0xEC;
pub const REG_TSTHRHI: c_uint = 0xEE;
pub const REG_CTHINT: c_uint = 0xF0;
pub const REG_TSFSC: c_uint = 0xF3;
pub const REG_CTSTS: c_uint = 0xF4;
pub const REG_TSTHRRQPI: c_uint = 0xF5;
pub const REG_CTCTRL: c_uint = 0xF7;
pub const REG_TSTIMER: c_uint = 0xF8;
    static int i5500_read(struct device *dev, enum hwmon_sensor_types type, u32 attr, int channel,
    long *val)
    {
    struct pci_dev *pdev = to_pci_dev(dev.parent);
    u16 tsthr;
    s8 tsfsc;
    u8 ctsts;
    switch (type) {
    case hwmon_temp:
    switch (attr) {
// Sensor resolution : 0.5 degree C
    case hwmon_temp_input:
    pci_read_config_word(pdev, REG_TSTHRHI, &tsthr);
    pci_read_config_byte(pdev, REG_TSFSC, &tsfsc);
// val = (tsthr - tsfsc) * 500;
    return 0;
    case hwmon_temp_max:
    pci_read_config_word(pdev, REG_TSTHRHI, &tsthr);
// val = tsthr * 500;
    return 0;
    case hwmon_temp_max_hyst:
    pci_read_config_word(pdev, REG_TSTHRLO, &tsthr);
// val = tsthr * 500;
    return 0;
    case hwmon_temp_crit:
    pci_read_config_word(pdev, REG_TSTHRCATA, &tsthr);
// val = tsthr * 500;
    return 0;
    case hwmon_temp_max_alarm:
    pci_read_config_byte(pdev, REG_CTSTS, &ctsts);
// val = !!(ctsts & BIT(1));
    return 0;
    case hwmon_temp_crit_alarm:
    pci_read_config_byte(pdev, REG_CTSTS, &ctsts);
// val = !!(ctsts & BIT(0));
    return 0;
    default:
    break;
    }
    break;
    default:
    break;
    }
    return -EOPNOTSUPP;
    }
    static const struct hwmon_ops i5500_ops = {
    .visible = 0444,
    .read = i5500_read,
    };
    static const struct hwmon_channel_info * const i5500_info[] = {
    HWMON_CHANNEL_INFO(chip, HWMON_C_REGISTER_TZ),
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT | HWMON_T_MAX | HWMON_T_MAX_HYST | HWMON_T_CRIT |
    HWMON_T_MAX_ALARM | HWMON_T_CRIT_ALARM
    ),
    core::ptr::null_mut()
    };
    static const struct hwmon_chip_info i5500_chip_info = {
    .ops = &i5500_ops,
    .info = i5500_info,
    };
    static const struct pci_device_id i5500_temp_ids[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, 0x3438) },
    { 0 },
    };
    MODULE_DEVICE_TABLE(pci, i5500_temp_ids);
    static int i5500_temp_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    int err;
    struct device *hwmon_dev;
    u32 tstimer;
    s8 tsfsc;
    err = pcim_enable_device(pdev);
    if (err) {
    dev_err(&pdev.dev, "Failed to enable device\n");
    return err;
    }
    pci_read_config_byte(pdev, REG_TSFSC, &tsfsc);
    pci_read_config_dword(pdev, REG_TSTIMER, &tstimer);
    if (tsfsc == 0x7F && tstimer == 0x07D30D40) {
    dev_notice(&pdev.dev, "Sensor seems to be disabled\n");
    return -ENODEV;
    }
    hwmon_dev = devm_hwmon_device_register_with_info(&pdev.dev, "intel5500", core::ptr::null_mut(),
    &i5500_chip_info, core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static struct pci_driver i5500_temp_driver = {
    .name = "i5500_temp",
    .id_table = i5500_temp_ids,
    .probe = i5500_temp_probe,
    };
    module_pci_driver(i5500_temp_driver);
    MODULE_AUTHOR("Jean Delvare <jdelvare@suse.de>");
    MODULE_DESCRIPTION("Intel 5500/5520/X58 chipset thermal sensor driver");
    MODULE_LICENSE("GPL");
