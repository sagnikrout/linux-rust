//! Automatically rewritten from C to Rust
//! Source: drivers/iio/imu/bmi270/bmi270_i2c.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)

    static const struct regmap_config bmi270_i2c_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    };
#[no_mangle]
unsafe extern "C" fn bmi270_i2c_probe(client: *mut i2c_client) -> c_int {
    static int bmi270_i2c_probe(struct i2c_client *client)
    {
    struct regmap *regmap;
    struct device *dev = &client.dev;
    const struct bmi270_chip_info *chip_info;
    chip_info = i2c_get_match_data(client);
    if (!chip_info)
    return -ENODEV;
    regmap = devm_regmap_init_i2c(client, &bmi270_i2c_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap),
    "Failed to init i2c regmap");
    return bmi270_core_probe(dev, regmap, chip_info);
    }
    static const struct i2c_device_id bmi270_i2c_id[] = {
    { .name = "bmi260", .driver_data = (kernel_ulong_t)&bmi260_chip_info },
    { .name = "bmi270", .driver_data = (kernel_ulong_t)&bmi270_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, bmi270_i2c_id);
    static const struct acpi_device_id bmi270_acpi_match[] = {
// GPD Win Mini, Aya Neo AIR Pro, OXP Mini Pro, etc.
    { "BMI0160",  (kernel_ulong_t)&bmi260_chip_info },
// GPD Win Max 2 2023(sincice BIOS v0.40), etc.
    { "BMI0260",  (kernel_ulong_t)&bmi260_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, bmi270_acpi_match);
    static const struct of_device_id bmi270_of_match[] = {
    { .compatible = "bosch,bmi260", .data = &bmi260_chip_info },
    { .compatible = "bosch,bmi270", .data = &bmi270_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(of, bmi270_of_match);
    static struct i2c_driver bmi270_i2c_driver = {
    .driver = {
    .name = "bmi270_i2c",
    .pm = pm_ptr(&bmi270_core_pm_ops),
    .acpi_match_table = bmi270_acpi_match,
    .of_match_table = bmi270_of_match,
    },
    .probe = bmi270_i2c_probe,
    .id_table = bmi270_i2c_id,
    };
    module_i2c_driver(bmi270_i2c_driver);
    MODULE_AUTHOR("Alex Lanzano");
    MODULE_DESCRIPTION("BMI270 driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_BMI270");
