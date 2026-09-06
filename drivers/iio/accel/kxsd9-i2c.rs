//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/kxsd9-i2c.c
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

#[no_mangle]
unsafe extern "C" fn kxsd9_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int kxsd9_i2c_probe(struct i2c_client *i2c)
    {
    static const struct regmap_config config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x0e,
    };
    struct regmap *regmap;
    regmap = devm_regmap_init_i2c(i2c, &config);
    if (IS_ERR(regmap)) {
    dev_err(&i2c.dev, "Failed to register i2c regmap: %pe\n",
    regmap);
    return PTR_ERR(regmap);
    }
    return kxsd9_common_probe(&i2c.dev,
    regmap,
    i2c.name);
    }
#[no_mangle]
unsafe extern "C" fn kxsd9_i2c_remove(client: *mut i2c_client) {
    static void kxsd9_i2c_remove(struct i2c_client *client)
    {
    kxsd9_common_remove(&client.dev);
    }
    static const struct of_device_id kxsd9_of_match[] = {
    { .compatible = "kionix,kxsd9", },
    { }
    };
    MODULE_DEVICE_TABLE(of, kxsd9_of_match);
    static const struct i2c_device_id kxsd9_i2c_id[] = {
    { .name = "kxsd9" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, kxsd9_i2c_id);
    static struct i2c_driver kxsd9_i2c_driver = {
    .driver = {
    .name	= "kxsd9",
    .of_match_table = kxsd9_of_match,
    .pm = pm_ptr(&kxsd9_dev_pm_ops),
    },
    .probe		= kxsd9_i2c_probe,
    .remove		= kxsd9_i2c_remove,
    .id_table	= kxsd9_i2c_id,
    };
    module_i2c_driver(kxsd9_i2c_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("KXSD9 accelerometer I2C interface");
    MODULE_IMPORT_NS("IIO_KXSD9");
