//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/goodix_berlin_i2c.c
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
// Goodix Berlin Touchscreen Driver
//
// Copyright (C) 2020 - 2021 Goodix, Inc.
// Copyright (C) 2023 Linaro Ltd.
//
// Based on goodix_ts_berlin driver.
//

pub const I2C_MAX_TRANSFER_SIZE: c_int = 256;
    static const struct regmap_config goodix_berlin_i2c_regmap_conf = {
    .reg_bits = 32,
    .val_bits = 8,
    .max_raw_read = I2C_MAX_TRANSFER_SIZE,
    .max_raw_write = I2C_MAX_TRANSFER_SIZE,
    };
// vendor & product left unassigned here, should probably be updated from fw info
    static const struct input_id goodix_berlin_i2c_input_id = {
    .bustype = BUS_I2C,
    };
#[no_mangle]
unsafe extern "C" fn goodix_berlin_i2c_probe(client: *mut i2c_client) -> c_int {
    static int goodix_berlin_i2c_probe(struct i2c_client *client)
    {
    const struct goodix_berlin_ic_data *ic_data =
    i2c_get_match_data(client);
    struct regmap *regmap;
    int error;
    regmap = devm_regmap_init_i2c(client, &goodix_berlin_i2c_regmap_conf);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    error = goodix_berlin_probe(&client.dev, client.irq,
    &goodix_berlin_i2c_input_id, regmap,
    ic_data);
    if (error)
    return error;
    return 0;
    }
    static const struct goodix_berlin_ic_data gt9916_data = {
    .fw_version_info_addr = GOODIX_BERLIN_FW_VERSION_INFO_ADDR_D,
    .ic_info_addr = GOODIX_BERLIN_IC_INFO_ADDR_D,
    };
    static const struct i2c_device_id goodix_berlin_i2c_id[] = {
    { .name = "gt9916", .driver_data = (long)&gt9916_data },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, goodix_berlin_i2c_id);
    static const struct of_device_id goodix_berlin_i2c_of_match[] = {
    { .compatible = "goodix,gt9916", .data = &gt9916_data },
    { }
    };
    MODULE_DEVICE_TABLE(of, goodix_berlin_i2c_of_match);
    static struct i2c_driver goodix_berlin_i2c_driver = {
    .driver = {
    .name = "goodix-berlin-i2c",
    .of_match_table = goodix_berlin_i2c_of_match,
    .pm = pm_sleep_ptr(&goodix_berlin_pm_ops),
    .dev_groups = goodix_berlin_groups,
    },
    .probe = goodix_berlin_i2c_probe,
    .id_table = goodix_berlin_i2c_id,
    };
    module_i2c_driver(goodix_berlin_i2c_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Goodix Berlin I2C Touchscreen driver");
    MODULE_AUTHOR("Neil Armstrong <neil.armstrong@linaro.org>");
