//! Automatically rewritten from C to Rust
//! Source: drivers/iio/humidity/hts221_i2c.c
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
// STMicroelectronics hts221 i2c driver
//
// Copyright 2016 STMicroelectronics Inc.
//
// Lorenzo Bianconi <lorenzo.bianconi@st.com>
//

    static const struct regmap_config hts221_i2c_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .write_flag_mask = HTS221_I2C_AUTO_INCREMENT,
    .read_flag_mask = HTS221_I2C_AUTO_INCREMENT,
    };
#[no_mangle]
unsafe extern "C" fn hts221_i2c_probe(client: *mut i2c_client) -> c_int {
    static int hts221_i2c_probe(struct i2c_client *client)
    {
    struct regmap *regmap;
    regmap = devm_regmap_init_i2c(client, &hts221_i2c_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(&client.dev, "Failed to register i2c regmap %ld\n",
    PTR_ERR(regmap));
    return PTR_ERR(regmap);
    }
    return hts221_probe(&client.dev, client.irq,
    client.name, regmap);
    }
    static const struct acpi_device_id hts221_acpi_match[] = {
    {"SMO9100", 0},
    { }
    };
    MODULE_DEVICE_TABLE(acpi, hts221_acpi_match);
    static const struct of_device_id hts221_i2c_of_match[] = {
    { .compatible = "st,hts221", },
    { }
    };
    MODULE_DEVICE_TABLE(of, hts221_i2c_of_match);
    static const struct i2c_device_id hts221_i2c_id_table[] = {
    { .name = HTS221_DEV_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, hts221_i2c_id_table);
    static struct i2c_driver hts221_driver = {
    .driver = {
    .name = "hts221_i2c",
    .pm = pm_sleep_ptr(&hts221_pm_ops),
    .of_match_table = hts221_i2c_of_match,
    .acpi_match_table = hts221_acpi_match,
    },
    .probe = hts221_i2c_probe,
    .id_table = hts221_i2c_id_table,
    };
    module_i2c_driver(hts221_driver);
    MODULE_AUTHOR("Lorenzo Bianconi <lorenzo.bianconi@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics hts221 i2c driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_HTS221");
