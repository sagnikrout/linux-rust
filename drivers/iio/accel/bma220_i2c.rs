//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/bma220_i2c.c
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
// Bosch triaxial acceleration sensor
//
// Copyright (c) 2025 Petre Rodan <petre.rodan@subdimension.ro>
//
// Datasheet: https://media.digikey.com/pdf/Data%20Sheets/Bosch/BMA220.pdf
// I2C address is either 0x0b or 0x0a depending on CSB (pin 10)
//

#[no_mangle]
unsafe extern "C" fn bma220_set_wdt(regmap: *mut regmap, val: u8) -> c_int {
    static int bma220_set_wdt(struct regmap *regmap, const u8 val)
    {
    return regmap_update_bits(regmap, BMA220_REG_WDT, BMA220_WDT_MASK,
    FIELD_PREP(BMA220_WDT_MASK, val));
    }
#[no_mangle]
unsafe extern "C" fn bma220_i2c_probe(client: *mut i2c_client) -> c_int {
    static int bma220_i2c_probe(struct i2c_client *client)
    {
    struct regmap *regmap;
    int ret;
    regmap = devm_regmap_init_i2c(client, &bma220_i2c_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(&client.dev, PTR_ERR(regmap),
    "failed to create regmap\n");
    ret = bma220_common_probe(&client.dev, regmap, client.irq);
    if (ret)
    return ret;
    return bma220_set_wdt(regmap, BMA220_WDT_1MS);
    }
    static const struct of_device_id bma220_i2c_match[] = {
    { .compatible = "bosch,bma220" },
    { }
    };
    MODULE_DEVICE_TABLE(of, bma220_i2c_match);
    static const struct i2c_device_id bma220_i2c_id[] = {
    { .name = "bma220" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, bma220_i2c_id);
    static struct i2c_driver bma220_i2c_driver = {
    .driver = {
    .name = "bma220_i2c",
    .pm = pm_sleep_ptr(&bma220_pm_ops),
    .of_match_table = bma220_i2c_match,
    },
    .probe = bma220_i2c_probe,
    .id_table = bma220_i2c_id,
    };
    module_i2c_driver(bma220_i2c_driver);
    MODULE_AUTHOR("Petre Rodan <petre.rodan@subdimension.ro>");
    MODULE_DESCRIPTION("Bosch triaxial acceleration sensor i2c driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_BOSCH_BMA220");
