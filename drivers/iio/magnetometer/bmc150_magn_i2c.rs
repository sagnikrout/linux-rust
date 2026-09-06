//! Automatically rewritten from C to Rust
//! Source: drivers/iio/magnetometer/bmc150_magn_i2c.c
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
// 3-axis magnetometer driver supporting following I2C Bosch-Sensortec chips:
// - BMC150
// - BMC156
// - BMM150
//
// Copyright (c) 2016, Intel Corporation.
//

#[no_mangle]
unsafe extern "C" fn bmc150_magn_i2c_probe(client: *mut i2c_client) -> c_int {
    static int bmc150_magn_i2c_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct regmap *regmap;
    const char *name = core::ptr::null_mut();
    regmap = devm_regmap_init_i2c(client, &bmc150_magn_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(&client.dev, "Failed to initialize i2c regmap\n");
    return PTR_ERR(regmap);
    }
    if (id)
    name = id.name;
    return bmc150_magn_probe(&client.dev, regmap, client.irq, name);
    }
#[no_mangle]
unsafe extern "C" fn bmc150_magn_i2c_remove(client: *mut i2c_client) {
    static void bmc150_magn_i2c_remove(struct i2c_client *client)
    {
    bmc150_magn_remove(&client.dev);
    }
    static const struct i2c_device_id bmc150_magn_i2c_id[] = {
    { .name = "bmc150_magn" },
    { .name = "bmc156_magn" },
    { .name = "bmm150_magn" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, bmc150_magn_i2c_id);
    static const struct of_device_id bmc150_magn_of_match[] = {
    { .compatible = "bosch,bmc150_magn" },
    { .compatible = "bosch,bmc156_magn" },
    { .compatible = "bosch,bmm150_magn" }, /* deprecated compatible */
    { .compatible = "bosch,bmm150" },
    { }
    };
    MODULE_DEVICE_TABLE(of, bmc150_magn_of_match);
    static struct i2c_driver bmc150_magn_driver = {
    .driver = {
    .name	= "bmc150_magn_i2c",
    .of_match_table = bmc150_magn_of_match,
    .pm	= &bmc150_magn_pm_ops,
    },
    .probe		= bmc150_magn_i2c_probe,
    .remove		= bmc150_magn_i2c_remove,
    .id_table	= bmc150_magn_i2c_id,
    };
    module_i2c_driver(bmc150_magn_driver);
    MODULE_AUTHOR("Daniel Baluta <daniel.baluta@intel.com");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("BMC150 I2C magnetometer driver");
    MODULE_IMPORT_NS("IIO_BMC150_MAGN");
