//! Automatically rewritten from C to Rust
//! Source: drivers/iio/imu/fxos8700_i2c.c
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
// FXOS8700 - NXP IMU, I2C bits
//
// 7-bit I2C slave address determined by SA1 and SA0 logic level
// inputs represented in the following table:
// SA1  |  SA0  |  Slave Address
// 0    |  0    |  0x1E
// 0    |  1    |  0x1D
// 1    |  0    |  0x1C
// 1    |  1    |  0x1F
//

#[no_mangle]
unsafe extern "C" fn fxos8700_i2c_probe(client: *mut i2c_client) -> c_int {
    static int fxos8700_i2c_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct regmap *regmap;
    const char *name = core::ptr::null_mut();
    regmap = devm_regmap_init_i2c(client, &fxos8700_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(&client.dev, "Failed to register i2c regmap %ld\n", PTR_ERR(regmap));
    return PTR_ERR(regmap);
    }
    if (id)
    name = id.name;
    return fxos8700_core_probe(&client.dev, regmap, name, false);
    }
    static const struct i2c_device_id fxos8700_i2c_id[] = {
    { .name = "fxos8700" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, fxos8700_i2c_id);
    static const struct acpi_device_id fxos8700_acpi_match[] = {
    {"FXOS8700", 0},
    { }
    };
    MODULE_DEVICE_TABLE(acpi, fxos8700_acpi_match);
    static const struct of_device_id fxos8700_of_match[] = {
    { .compatible = "nxp,fxos8700" },
    { }
    };
    MODULE_DEVICE_TABLE(of, fxos8700_of_match);
    static struct i2c_driver fxos8700_i2c_driver = {
    .driver = {
    .name                   = "fxos8700_i2c",
    .acpi_match_table       = fxos8700_acpi_match,
    .of_match_table         = fxos8700_of_match,
    },
    .probe          = fxos8700_i2c_probe,
    .id_table       = fxos8700_i2c_id,
    };
    module_i2c_driver(fxos8700_i2c_driver);
    MODULE_AUTHOR("Robert Jones <rjones@gateworks.com>");
    MODULE_DESCRIPTION("FXOS8700 I2C driver");
    MODULE_LICENSE("GPL v2");
