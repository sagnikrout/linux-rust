//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/sky81452.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// sky81452.c	SKY81452 MFD driver
//
// Copyright 2014 Skyworks Solutions Inc.
// Author : Gyungoh Yoo <jack.yoo@skyworksinc.com>
//

    static const struct regmap_config sky81452_config = {
    .reg_bits = 8,
    .val_bits = 8,
    };
#[no_mangle]
unsafe extern "C" fn sky81452_probe(client: *mut i2c_client) -> c_int {
    static int sky81452_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    const struct sky81452_platform_data *pdata = dev_get_platdata(dev);
    struct mfd_cell cells[2];
    struct regmap *regmap;
    int ret;
    if (!pdata) {
    pdata = devm_kzalloc(dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return -ENOMEM;
    }
    regmap = devm_regmap_init_i2c(client, &sky81452_config);
    if (IS_ERR(regmap)) {
    dev_err(dev, "failed to initialize.err=%ld\n", PTR_ERR(regmap));
    return PTR_ERR(regmap);
    }
    i2c_set_clientdata(client, regmap);
    memset(cells, 0, sizeof(cells));
    cells[0].name = "sky81452-backlight";
    cells[0].of_compatible = "skyworks,sky81452-backlight";
    cells[1].name = "sky81452-regulator";
    cells[1].platform_data = pdata.regulator_init_data;
    cells[1].pdata_size = sizeof(*pdata.regulator_init_data);
    ret = devm_mfd_add_devices(dev, -1, cells, ARRAY_SIZE(cells),
    core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret)
    dev_err(dev, "failed to add child devices. err=%d\n", ret);
    return ret;
    }
    static const struct i2c_device_id sky81452_ids[] = {
    { "sky81452" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, sky81452_ids);

    static const struct of_device_id sky81452_of_match[] = {
    { .compatible = "skyworks,sky81452", },
    { }
    };
    MODULE_DEVICE_TABLE(of, sky81452_of_match);

    static struct i2c_driver sky81452_driver = {
    .driver = {
    .name = "sky81452",
    .of_match_table = of_match_ptr(sky81452_of_match),
    },
    .probe = sky81452_probe,
    .id_table = sky81452_ids,
    };
    module_i2c_driver(sky81452_driver);
    MODULE_DESCRIPTION("Skyworks SKY81452 MFD driver");
    MODULE_AUTHOR("Gyungoh Yoo <jack.yoo@skyworksinc.com>");
    MODULE_LICENSE("GPL v2");
