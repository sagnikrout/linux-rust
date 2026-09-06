//! Automatically rewritten from C to Rust
//! Source: sound/hda/codecs/side-codecs/cs35l41_hda_i2c.c
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
// CS35l41 HDA I2C driver
//
// Copyright 2021 Cirrus Logic, Inc.
//
// Author: Lucas Tanure <tanureal@opensource.cirrus.com>

#[no_mangle]
unsafe extern "C" fn cs35l41_hda_i2c_probe(clt: *mut i2c_client) -> c_int {
    static int cs35l41_hda_i2c_probe(struct i2c_client *clt)
    {
    const char *device_name;
//
// Compare against the device name so it works for SPI, normal ACPI
// and for ACPI by serial-multi-instantiate matching cases.
//
    if (strstr(dev_name(&clt.dev), "CLSA0100"))
    device_name = "CLSA0100";
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstr(dev_name(&clt->dev), _arg: "CLSA0101")) -> else {
    else if (strstr(dev_name(&clt.dev), "CLSA0101"))
    device_name = "CLSA0101";
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstr(dev_name(&clt->dev), _arg: "CSC3551")) -> else {
    else if (strstr(dev_name(&clt.dev), "CSC3551"))
    device_name = "CSC3551";
    else
    return -ENODEV;
    return cs35l41_hda_probe(&clt.dev, device_name, clt.addr, clt.irq,
    devm_regmap_init_i2c(clt, &cs35l41_regmap_i2c), I2C);
    }
#[no_mangle]
unsafe extern "C" fn cs35l41_hda_i2c_remove(clt: *mut i2c_client) {
    static void cs35l41_hda_i2c_remove(struct i2c_client *clt)
    {
    cs35l41_hda_remove(&clt.dev);
    }
    static const struct i2c_device_id cs35l41_hda_i2c_id[] = {
    { .name = "cs35l41-hda" },
    { }
    };
    static const struct acpi_device_id cs35l41_acpi_hda_match[] = {
    {"CLSA0100", 0 },
    {"CLSA0101", 0 },
    {"CSC3551", 0 },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, cs35l41_acpi_hda_match);
    static struct i2c_driver cs35l41_i2c_driver = {
    .driver = {
    .name		= "cs35l41-hda",
    .acpi_match_table = cs35l41_acpi_hda_match,
    .pm		= &cs35l41_hda_pm_ops,
    },
    .id_table	= cs35l41_hda_i2c_id,
    .probe		= cs35l41_hda_i2c_probe,
    .remove		= cs35l41_hda_i2c_remove,
    };
    module_i2c_driver(cs35l41_i2c_driver);
    MODULE_DESCRIPTION("HDA CS35L41 driver");
    MODULE_IMPORT_NS("SND_HDA_SCODEC_CS35L41");
    MODULE_AUTHOR("Lucas Tanure <tanureal@opensource.cirrus.com>");
    MODULE_LICENSE("GPL");
