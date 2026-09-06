//! Automatically rewritten from C to Rust
//! Source: drivers/dpll/zl3073x/i2c.c
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

#[no_mangle]
unsafe extern "C" fn zl3073x_i2c_probe(client: *mut i2c_client) -> c_int {
    static int zl3073x_i2c_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct zl3073x_dev *zldev;
    zldev = zl3073x_devm_alloc(dev);
    if (IS_ERR(zldev))
    return PTR_ERR(zldev);
    zldev.regmap = devm_regmap_init_i2c(client, &zl3073x_regmap_config);
    if (IS_ERR(zldev.regmap))
    return dev_err_probe(dev, PTR_ERR(zldev.regmap),
    "Failed to initialize regmap\n");
    return zl3073x_dev_probe(zldev);
    }
    static const struct i2c_device_id zl3073x_i2c_id[] = {
    { .name = "zl30731" },
    { .name = "zl30732" },
    { .name = "zl30733" },
    { .name = "zl30734" },
    { .name = "zl30735" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(i2c, zl3073x_i2c_id);
    static const struct of_device_id zl3073x_i2c_of_match[] = {
    { .compatible = "microchip,zl30731" },
    { .compatible = "microchip,zl30732" },
    { .compatible = "microchip,zl30733" },
    { .compatible = "microchip,zl30734" },
    { .compatible = "microchip,zl30735" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, zl3073x_i2c_of_match);
    static struct i2c_driver zl3073x_i2c_driver = {
    .driver = {
    .name = "zl3073x-i2c",
    .of_match_table = zl3073x_i2c_of_match,
    },
    .probe = zl3073x_i2c_probe,
    .id_table = zl3073x_i2c_id,
    };
    module_i2c_driver(zl3073x_i2c_driver);
    MODULE_AUTHOR("Ivan Vecera <ivecera@redhat.com>");
    MODULE_DESCRIPTION("Microchip ZL3073x I2C driver");
    MODULE_IMPORT_NS("ZL3073X");
    MODULE_LICENSE("GPL");
