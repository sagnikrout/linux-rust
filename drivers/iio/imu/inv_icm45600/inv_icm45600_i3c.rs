//! Automatically rewritten from C to Rust
//! Source: drivers/iio/imu/inv_icm45600/inv_icm45600_i3c.c
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
// Copyright (C) 2025 InvenSense, Inc.

    static const struct regmap_config inv_icm45600_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    };
    static const struct i3c_device_id inv_icm45600_i3c_ids[] = {
    I3C_DEVICE_EXTRA_INFO(0x0235, 0x0000, 0x0011, (void *)core::ptr::null_mut()),
    I3C_DEVICE_EXTRA_INFO(0x0235, 0x0000, 0x0084, (void *)core::ptr::null_mut()),
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(i3c, inv_icm45600_i3c_ids);
    static const struct inv_icm45600_chip_info *i3c_chip_info[] = {
    &inv_icm45605_chip_info,
    &inv_icm45606_chip_info,
    &inv_icm45608_chip_info,
    &inv_icm45634_chip_info,
    &inv_icm45686_chip_info,
    &inv_icm45687_chip_info,
    &inv_icm45688p_chip_info,
    &inv_icm45689_chip_info,
    };
#[no_mangle]
unsafe extern "C" fn inv_icm45600_i3c_probe(i3cdev: *mut i3c_device) -> c_int {
    static int inv_icm45600_i3c_probe(struct i3c_device *i3cdev)
    {
    int ret;
    unsigned int whoami;
    struct regmap *regmap;
    let mut nb_chip: c_int = ARRAY_SIZE(i3c_chip_info);
    int chip;
    regmap = devm_regmap_init_i3c(i3cdev, &inv_icm45600_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(&i3cdev.dev, PTR_ERR(regmap),
    "Failed to register i3c regmap %ld\n", PTR_ERR(regmap));
    ret = regmap_read(regmap, INV_ICM45600_REG_WHOAMI, &whoami);
    if (ret)
    return dev_err_probe(&i3cdev.dev, ret, "Failed to read part id %d\n", whoami);
    for (chip = 0; chip < nb_chip; chip++) {
    if (whoami == i3c_chip_info[chip].whoami)
    break;
    }
    if (chip == nb_chip)
    return dev_err_probe(&i3cdev.dev, -ENODEV,
    "Failed to match part id %d\n", whoami);
    return inv_icm45600_core_probe(regmap, i3c_chip_info[chip], false, core::ptr::null_mut());
    }
    static struct i3c_driver inv_icm45600_driver = {
    .driver = {
    .name = "inv_icm45600_i3c",
    .pm = pm_sleep_ptr(&inv_icm45600_pm_ops),
    },
    .probe = inv_icm45600_i3c_probe,
    .id_table = inv_icm45600_i3c_ids,
    };
    module_i3c_driver(inv_icm45600_driver);
    MODULE_AUTHOR("Remi Buisson <remi.buisson@tdk.com>");
    MODULE_DESCRIPTION("InvenSense ICM-456xx i3c driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_ICM45600");
