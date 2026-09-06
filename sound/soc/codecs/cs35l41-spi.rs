//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/cs35l41-spi.c
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
// cs35l41-spi.c -- CS35l41 SPI driver
//
// Copyright 2017-2021 Cirrus Logic, Inc.
//
// Author: David Rhodes	<david.rhodes@cirrus.com>

    static const struct spi_device_id cs35l41_id_spi[] = {
    { "cs35l40", 0 },
    { "cs35l41", 0 },
    { "cs35l51", 0 },
    { "cs35l53", 0 },
    {}
    };
    MODULE_DEVICE_TABLE(spi, cs35l41_id_spi);
#[no_mangle]
unsafe extern "C" fn cs35l41_spi_probe(spi: *mut spi_device) -> c_int {
    static int cs35l41_spi_probe(struct spi_device *spi)
    {
    const struct regmap_config *regmap_config = &cs35l41_regmap_spi;
    struct cs35l41_hw_cfg *hw_cfg = dev_get_platdata(&spi.dev);
    struct cs35l41_private *cs35l41;
    int ret;
    cs35l41 = devm_kzalloc(&spi.dev, sizeof(struct cs35l41_private), GFP_KERNEL);
    if (!cs35l41)
    return -ENOMEM;
    spi.max_speed_hz = CS35L41_SPI_MAX_FREQ;
    ret = spi_setup(spi);
    if (ret < 0)
    return ret;
    spi_set_drvdata(spi, cs35l41);
    cs35l41.regmap = devm_regmap_init_spi(spi, regmap_config);
    if (IS_ERR(cs35l41.regmap))
    return dev_err_probe(cs35l41.dev, PTR_ERR(cs35l41.regmap),
    "Failed to allocate register map\n");
    cs35l41.dev = &spi.dev;
    cs35l41.irq = spi.irq;
    return cs35l41_probe(cs35l41, hw_cfg);
    }
#[no_mangle]
unsafe extern "C" fn cs35l41_spi_remove(spi: *mut spi_device) {
    static void cs35l41_spi_remove(struct spi_device *spi)
    {
    struct cs35l41_private *cs35l41 = spi_get_drvdata(spi);
    cs35l41_remove(cs35l41);
    }

    static const struct of_device_id cs35l41_of_match[] = {
    { .compatible = "cirrus,cs35l40" },
    { .compatible = "cirrus,cs35l41" },
    {},
    };
    MODULE_DEVICE_TABLE(of, cs35l41_of_match);

    static const struct acpi_device_id cs35l41_acpi_match[] = {
    { "CSC3541", 0 }, /* Cirrus Logic PnP ID + part ID */
    { "CLSA3541", 0 }, /* Cirrus Logic PnP ID + part ID */
    {},
    };
    MODULE_DEVICE_TABLE(acpi, cs35l41_acpi_match);

    static struct spi_driver cs35l41_spi_driver = {
    .driver = {
    .name		= "cs35l41",
    .pm		= pm_ptr(&cs35l41_pm_ops),
    .of_match_table = of_match_ptr(cs35l41_of_match),
    .acpi_match_table = ACPI_PTR(cs35l41_acpi_match),
    },
    .id_table	= cs35l41_id_spi,
    .probe		= cs35l41_spi_probe,
    .remove		= cs35l41_spi_remove,
    };
    module_spi_driver(cs35l41_spi_driver);
    MODULE_DESCRIPTION("SPI CS35L41 driver");
    MODULE_AUTHOR("David Rhodes, Cirrus Logic Inc, <david.rhodes@cirrus.com>");
    MODULE_LICENSE("GPL");
