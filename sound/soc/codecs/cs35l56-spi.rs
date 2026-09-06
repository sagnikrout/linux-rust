//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/cs35l56-spi.c
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
// CS35L56 ALSA SoC audio driver SPI binding
//
// Copyright (C) 2023 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.

#[no_mangle]
unsafe extern "C" fn cs35l56_spi_probe(spi: *mut spi_device) -> c_int {
    static int cs35l56_spi_probe(struct spi_device *spi)
    {
    const struct regmap_config *regmap_config = &cs35l56_regmap_spi;
    struct cs35l56_private *cs35l56;
    int ret;
    cs35l56 = devm_kzalloc(&spi.dev, sizeof(struct cs35l56_private), GFP_KERNEL);
    if (!cs35l56)
    return -ENOMEM;
    spi_set_drvdata(spi, cs35l56);
    cs35l56.base.type = 0x56;
    cs35l56.base.regmap = devm_regmap_init_spi(spi, regmap_config);
    if (IS_ERR(cs35l56.base.regmap)) {
    ret = PTR_ERR(cs35l56.base.regmap);
    return dev_err_probe(&spi.dev, ret, "Failed to allocate register map\n");
    }
    cs35l56.base.dev = &spi.dev;
    cs35l56.base.can_hibernate = true;
    ret = cs35l56_init_config_for_spi(&cs35l56.base, spi);
    if (ret)
    return ret;
    return cs35l56_common_probe(cs35l56, spi.irq);
    }
#[no_mangle]
unsafe extern "C" fn cs35l56_spi_remove(spi: *mut spi_device) {
    static void cs35l56_spi_remove(struct spi_device *spi)
    {
    struct cs35l56_private *cs35l56 = spi_get_drvdata(spi);
    cs35l56_remove(cs35l56);
    }
    static const struct spi_device_id cs35l56_id_spi[] = {
    { "cs35l56", 0 },
    {}
    };
    MODULE_DEVICE_TABLE(spi, cs35l56_id_spi);

    static const struct acpi_device_id cs35l56_asoc_acpi_match[] = {
    { "CSC355C", 0 },
    {},
    };
    MODULE_DEVICE_TABLE(acpi, cs35l56_asoc_acpi_match);

    static struct spi_driver cs35l56_spi_driver = {
    .driver = {
    .name		= "cs35l56",
    .pm = pm_ptr(&cs35l56_pm_ops_i2c_spi),
    .acpi_match_table = ACPI_PTR(cs35l56_asoc_acpi_match),
    },
    .id_table	= cs35l56_id_spi,
    .probe		= cs35l56_spi_probe,
    .remove		= cs35l56_spi_remove,
    };
    module_spi_driver(cs35l56_spi_driver);
    MODULE_DESCRIPTION("ASoC CS35L56 SPI driver");
    MODULE_IMPORT_NS("SND_SOC_CS35L56_CORE");
    MODULE_IMPORT_NS("SND_SOC_CS35L56_SHARED");
    MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>");
    MODULE_AUTHOR("Simon Trimmer <simont@opensource.cirrus.com>");
    MODULE_LICENSE("GPL");
