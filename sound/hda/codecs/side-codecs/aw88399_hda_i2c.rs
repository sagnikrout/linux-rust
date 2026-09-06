//! Automatically rewritten from C to Rust
//! Source: sound/hda/codecs/side-codecs/aw88399_hda_i2c.c
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
// AW88399 HDA I2C driver
//
// Based on cs35l41_hda_i2c.c
//

#[no_mangle]
unsafe extern "C" fn aw88399_hda_i2c_probe(clt: *mut i2c_client) -> c_int {
    static int aw88399_hda_i2c_probe(struct i2c_client *clt)
    {
    if (!strstr(dev_name(&clt.dev), "AWDZ8399"))
    return -ENODEV;
    return aw88399_hda_probe(&clt.dev,
    devm_regmap_init_i2c(clt, &aw88399_remap_config));
    }
#[no_mangle]
unsafe extern "C" fn aw88399_hda_i2c_remove(clt: *mut i2c_client) {
    static void aw88399_hda_i2c_remove(struct i2c_client *clt)
    {
    aw88399_hda_remove(&clt.dev);
    }
    static const struct i2c_device_id aw88399_hda_i2c_id[] = {
    { .name = "aw88399-hda" },
    { }
    };
    static const struct acpi_device_id aw88399_acpi_hda_match[] = {
    { "AWDZ8399", 0 },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, aw88399_acpi_hda_match);
    static struct i2c_driver aw88399_hda_i2c_driver = {
    .driver = {
    .name		= "aw88399-hda",
    .acpi_match_table = aw88399_acpi_hda_match,
    .pm		= &aw88399_hda_pm_ops,
    },
    .probe		= aw88399_hda_i2c_probe,
    .remove		= aw88399_hda_i2c_remove,
    .id_table	= aw88399_hda_i2c_id,
    };
    module_i2c_driver(aw88399_hda_i2c_driver);
    MODULE_DESCRIPTION("HDA AW88399 I2C driver");
    MODULE_IMPORT_NS("SND_HDA_SCODEC_AW88399");
    MODULE_AUTHOR("Yakov Till <yakov.till@gmail.com>");
    MODULE_LICENSE("GPL");
