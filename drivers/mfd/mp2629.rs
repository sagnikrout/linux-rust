//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/mp2629.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// MP2629 parent driver for ADC and battery charger
//
// Copyright 2020 Monolithic Power Systems, Inc
//
// Author: Saravanan Sekar <sravanhome@gmail.com>
//

    static const struct mfd_cell mp2629_cell[] = {
    {
    .name = "mp2629_adc",
    .of_compatible = "mps,mp2629_adc",
    },
    {
    .name = "mp2629_charger",
    .of_compatible = "mps,mp2629_charger",
    }
    };
    static const struct regmap_config mp2629_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x17,
    };
#[no_mangle]
unsafe extern "C" fn mp2629_probe(client: *mut i2c_client) -> c_int {
    static int mp2629_probe(struct i2c_client *client)
    {
    struct mp2629_data *ddata;
    int ret;
    ddata = devm_kzalloc(&client.dev, sizeof(*ddata), GFP_KERNEL);
    if (!ddata)
    return -ENOMEM;
    ddata.dev = &client.dev;
    i2c_set_clientdata(client, ddata);
    ddata.regmap = devm_regmap_init_i2c(client, &mp2629_regmap_config);
    if (IS_ERR(ddata.regmap)) {
    dev_err(ddata.dev, "Failed to allocate regmap\n");
    return PTR_ERR(ddata.regmap);
    }
    ret = devm_mfd_add_devices(ddata.dev, PLATFORM_DEVID_AUTO, mp2629_cell,
    ARRAY_SIZE(mp2629_cell), core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret)
    dev_err(ddata.dev, "Failed to register sub-devices %d\n", ret);
    return ret;
    }
    static const struct of_device_id mp2629_of_match[] = {
    { .compatible = "mps,mp2629"},
    { }
    };
    MODULE_DEVICE_TABLE(of, mp2629_of_match);
    static struct i2c_driver mp2629_driver = {
    .driver = {
    .name = "mp2629",
    .of_match_table = mp2629_of_match,
    },
    .probe		= mp2629_probe,
    };
    module_i2c_driver(mp2629_driver);
    MODULE_AUTHOR("Saravanan Sekar <sravanhome@gmail.com>");
    MODULE_DESCRIPTION("MP2629 Battery charger parent driver");
    MODULE_LICENSE("GPL");
