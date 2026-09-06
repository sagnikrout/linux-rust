//! Automatically rewritten from C to Rust
//! Source: drivers/iio/imu/st_lsm6dsx/st_lsm6dsx_i3c.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2018 Synopsys, Inc. and/or its affiliates.
//
// Author: Vitor Soares <vitor.soares@synopsys.com>
//

    static const struct i3c_device_id st_lsm6dsx_i3c_ids[] = {
    I3C_DEVICE(0x0104, 0x006C, (void *)ST_LSM6DSO_ID),
    I3C_DEVICE(0x0104, 0x006B, (void *)ST_LSM6DSR_ID),
    { }
    };
    MODULE_DEVICE_TABLE(i3c, st_lsm6dsx_i3c_ids);
#[no_mangle]
unsafe extern "C" fn st_lsm6dsx_i3c_probe(i3cdev: *mut i3c_device) -> c_int {
    static int st_lsm6dsx_i3c_probe(struct i3c_device *i3cdev)
    {
    struct regmap_config st_lsm6dsx_i3c_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    };
    const struct i3c_device_id *id = i3c_device_match_id(i3cdev,
    st_lsm6dsx_i3c_ids);
    struct device *dev = i3cdev_to_dev(i3cdev);
    struct regmap *regmap;
    regmap = devm_regmap_init_i3c(i3cdev, &st_lsm6dsx_i3c_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(dev, "Failed to register i3c regmap %ld\n", PTR_ERR(regmap));
    return PTR_ERR(regmap);
    }
    return st_lsm6dsx_probe(dev, 0, (uintptr_t)id.data, regmap);
    }
    static struct i3c_driver st_lsm6dsx_driver = {
    .driver = {
    .name = "st_lsm6dsx_i3c",
    .pm = pm_sleep_ptr(&st_lsm6dsx_pm_ops),
    },
    .probe = st_lsm6dsx_i3c_probe,
    .id_table = st_lsm6dsx_i3c_ids,
    };
    module_i3c_driver(st_lsm6dsx_driver);
    MODULE_AUTHOR("Vitor Soares <vitor.soares@synopsys.com>");
    MODULE_DESCRIPTION("STMicroelectronics st_lsm6dsx i3c driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_LSM6DSX");
