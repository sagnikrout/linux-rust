//! Automatically rewritten from C to Rust
//! Source: drivers/iio/gyro/bmg160_i2c.c
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

    static const struct regmap_config bmg160_regmap_i2c_conf = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x3f
    };
#[no_mangle]
unsafe extern "C" fn bmg160_i2c_probe(client: *mut i2c_client) -> c_int {
    static int bmg160_i2c_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct regmap *regmap;
    const char *name;
    regmap = devm_regmap_init_i2c(client, &bmg160_regmap_i2c_conf);
    if (IS_ERR(regmap)) {
    dev_err(&client.dev, "Failed to register i2c regmap: %pe\n",
    regmap);
    return PTR_ERR(regmap);
    }
    if (id)
    name = id.name;
    else
    name = iio_get_acpi_device_name(&client.dev);
    return bmg160_core_probe(&client.dev, regmap, client.irq, name);
    }
#[no_mangle]
unsafe extern "C" fn bmg160_i2c_remove(client: *mut i2c_client) {
    static void bmg160_i2c_remove(struct i2c_client *client)
    {
    bmg160_core_remove(&client.dev);
    }
    static const struct acpi_device_id bmg160_acpi_match[] = {
    {"BMG0160", 0},
    { }
    };
    MODULE_DEVICE_TABLE(acpi, bmg160_acpi_match);
    static const struct i2c_device_id bmg160_i2c_id[] = {
    { .name = "bmg160" },
    { .name = "bmi055_gyro" },
    { .name = "bmi088_gyro" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, bmg160_i2c_id);
    static const struct of_device_id bmg160_of_match[] = {
    { .compatible = "bosch,bmg160" },
    { .compatible = "bosch,bmi055_gyro" },
    { .compatible = "bosch,bmi088_gyro" },
    { }
    };
    MODULE_DEVICE_TABLE(of, bmg160_of_match);
    static struct i2c_driver bmg160_i2c_driver = {
    .driver = {
    .name	= "bmg160_i2c",
    .acpi_match_table = bmg160_acpi_match,
    .of_match_table = bmg160_of_match,
    .pm	= &bmg160_pm_ops,
    },
    .probe		= bmg160_i2c_probe,
    .remove		= bmg160_i2c_remove,
    .id_table	= bmg160_i2c_id,
    };
    module_i2c_driver(bmg160_i2c_driver);
    MODULE_AUTHOR("Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("BMG160 I2C Gyro driver");
