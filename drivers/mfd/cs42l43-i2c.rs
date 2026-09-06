//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/cs42l43-i2c.c
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
// CS42L43 I2C driver
//
// Copyright (C) 2022-2023 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//

    static const struct regmap_config cs42l43_i2c_regmap = {
    .reg_bits		= 32,
    .reg_stride		= 4,
    .val_bits		= 32,
    .reg_format_endian	= REGMAP_ENDIAN_BIG,
    .val_format_endian	= REGMAP_ENDIAN_BIG,
    .max_register		= CS42L43_MCU_RAM_MAX,
    .readable_reg		= cs42l43_readable_register,
    .volatile_reg		= cs42l43_volatile_register,
    .precious_reg		= cs42l43_precious_register,
    .cache_type		= REGCACHE_MAPLE,
    .reg_defaults		= cs42l43_reg_default,
    .num_reg_defaults	= ARRAY_SIZE(cs42l43_reg_default),
    };
#[no_mangle]
unsafe extern "C" fn cs42l43_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int cs42l43_i2c_probe(struct i2c_client *i2c)
    {
    struct cs42l43 *cs42l43;
    cs42l43 = devm_kzalloc(&i2c.dev, sizeof(*cs42l43), GFP_KERNEL);
    if (!cs42l43)
    return -ENOMEM;
    cs42l43.dev = &i2c.dev;
    cs42l43.irq = i2c.irq;
    cs42l43.variant_id = (long)device_get_match_data(cs42l43.dev);
    cs42l43.regmap = devm_regmap_init_i2c(i2c, &cs42l43_i2c_regmap);
    if (IS_ERR(cs42l43.regmap))
    return dev_err_probe(cs42l43.dev, PTR_ERR(cs42l43.regmap),
    "Failed to allocate regmap\n");
    return cs42l43_dev_probe(cs42l43);
    }

    static const struct of_device_id cs42l43_of_match[] = {
    { .compatible = "cirrus,cs42l43", .data = (void *)CS42L43_DEVID_VAL },
    { .compatible = "cirrus,cs42l43b", .data = (void *)CS42L43B_DEVID_VAL },
    {}
    };
    MODULE_DEVICE_TABLE(of, cs42l43_of_match);

    static const struct acpi_device_id cs42l43_acpi_match[] = {
    { "CSC4243", CS42L43_DEVID_VAL },
    { "CSC2A3B", CS42L43B_DEVID_VAL },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, cs42l43_acpi_match);

    static struct i2c_driver cs42l43_i2c_driver = {
    .driver = {
    .name			= "cs42l43",
    .pm			= pm_ptr(&cs42l43_pm_ops),
    .of_match_table		= of_match_ptr(cs42l43_of_match),
    .acpi_match_table	= ACPI_PTR(cs42l43_acpi_match),
    },
    .probe		= cs42l43_i2c_probe,
    };
    module_i2c_driver(cs42l43_i2c_driver);
    MODULE_IMPORT_NS("MFD_CS42L43");
    MODULE_DESCRIPTION("CS42L43 I2C Driver");
    MODULE_AUTHOR("Charles Keepax <ckeepax@opensource.cirrus.com>");
    MODULE_LICENSE("GPL");
