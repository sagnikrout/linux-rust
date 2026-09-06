//! Automatically rewritten from C to Rust
//! Source: sound/hda/codecs/side-codecs/cs35l41_hda_spi.c
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
// CS35l41 HDA SPI driver
//
// Copyright 2021 Cirrus Logic, Inc.
//
// Author: Lucas Tanure <tanureal@opensource.cirrus.com>

#[no_mangle]
unsafe extern "C" fn cs35l41_hda_spi_probe(spi: *mut spi_device) -> c_int {
    static int cs35l41_hda_spi_probe(struct spi_device *spi)
    {
    const char *device_name;
//
// Compare against the device name so it works for SPI, normal ACPI
// and for ACPI by serial-multi-instantiate matching cases.
//
    if (strstr(dev_name(&spi.dev), "CSC3551"))
    device_name = "CSC3551";
    else
    return -ENODEV;
    return cs35l41_hda_probe(&spi.dev, device_name, spi_get_chipselect(spi, 0), spi.irq,
    devm_regmap_init_spi(spi, &cs35l41_regmap_spi), SPI);
    }
#[no_mangle]
unsafe extern "C" fn cs35l41_hda_spi_remove(spi: *mut spi_device) {
    static void cs35l41_hda_spi_remove(struct spi_device *spi)
    {
    cs35l41_hda_remove(&spi.dev);
    }
    static const struct spi_device_id cs35l41_hda_spi_id[] = {
    { "cs35l41-hda", 0 },
    {}
    };
    MODULE_DEVICE_TABLE(spi, cs35l41_hda_spi_id);
    static const struct acpi_device_id cs35l41_acpi_hda_match[] = {
    { "CSC3551", 0 },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, cs35l41_acpi_hda_match);
    static struct spi_driver cs35l41_spi_driver = {
    .driver = {
    .name		= "cs35l41-hda",
    .acpi_match_table = cs35l41_acpi_hda_match,
    .pm		= &cs35l41_hda_pm_ops,
    },
    .id_table	= cs35l41_hda_spi_id,
    .probe		= cs35l41_hda_spi_probe,
    .remove		= cs35l41_hda_spi_remove,
    };
    module_spi_driver(cs35l41_spi_driver);
    MODULE_DESCRIPTION("HDA CS35L41 driver");
    MODULE_IMPORT_NS("SND_HDA_SCODEC_CS35L41");
    MODULE_AUTHOR("Lucas Tanure <tanureal@opensource.cirrus.com>");
    MODULE_LICENSE("GPL");
