//! Automatically rewritten from C to Rust
//! Source: drivers/iio/pressure/bmp280-i2c.c
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
unsafe extern "C" fn bmp280_i2c_probe(client: *mut i2c_client) -> c_int {
    static int bmp280_i2c_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    const struct bmp280_chip_info *chip_info;
    struct regmap *regmap;
    chip_info = i2c_get_match_data(client);
    regmap = devm_regmap_init_i2c(client, chip_info.regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(&client.dev, "failed to allocate register map\n");
    return PTR_ERR(regmap);
    }
    return bmp280_common_probe(&client.dev,
    regmap,
    chip_info,
    id.name,
    client.irq);
    }
    static const struct of_device_id bmp280_of_i2c_match[] = {
    { .compatible = "bosch,bmp085", .data = &bmp085_chip_info },
    { .compatible = "bosch,bmp180", .data = &bmp180_chip_info },
    { .compatible = "bosch,bmp280", .data = &bmp280_chip_info },
    { .compatible = "bosch,bme280", .data = &bme280_chip_info },
    { .compatible = "bosch,bmp380", .data = &bmp380_chip_info },
    { .compatible = "bosch,bmp580", .data = &bmp580_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(of, bmp280_of_i2c_match);
    static const struct i2c_device_id bmp280_i2c_id[] = {
    { .name = "bmp085", .driver_data = (kernel_ulong_t)&bmp085_chip_info },
    { .name = "bmp180", .driver_data = (kernel_ulong_t)&bmp180_chip_info },
    { .name = "bmp280", .driver_data = (kernel_ulong_t)&bmp280_chip_info },
    { .name = "bme280", .driver_data = (kernel_ulong_t)&bme280_chip_info },
    { .name = "bmp380", .driver_data = (kernel_ulong_t)&bmp380_chip_info },
    { .name = "bmp580", .driver_data = (kernel_ulong_t)&bmp580_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, bmp280_i2c_id);
    static struct i2c_driver bmp280_i2c_driver = {
    .driver = {
    .name	= "bmp280",
    .of_match_table = bmp280_of_i2c_match,
    .pm = pm_ptr(&bmp280_dev_pm_ops),
    },
    .probe		= bmp280_i2c_probe,
    .id_table	= bmp280_i2c_id,
    };
    module_i2c_driver(bmp280_i2c_driver);
    MODULE_AUTHOR("Vlad Dogaru <vlad.dogaru@intel.com>");
    MODULE_DESCRIPTION("Driver for Bosch Sensortec BMP180/BMP280 pressure and temperature sensor");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_BMP280");
