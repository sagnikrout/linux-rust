//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/an8855-efuse.c
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
// Airoha AN8855 Switch EFUSE Driver
//

pub const AN8855_EFUSE_CELL: c_int = 50;
pub const AN8855_EFUSE_DATA0: c_uint = 0x1000a500;

    static int an8855_efuse_read(void *context, unsigned int offset,
    void *val, size_t bytes)
    {
    struct regmap *regmap = context;
    return regmap_bulk_read(regmap, AN8855_EFUSE_DATA0 + offset,
    val, bytes / sizeof(u32));
    }
#[no_mangle]
unsafe extern "C" fn an8855_efuse_probe(pdev: *mut platform_device) -> c_int {
    static int an8855_efuse_probe(struct platform_device *pdev)
    {
    struct nvmem_config an8855_nvmem_config = {
    .name = "an8855-efuse",
    .size = AN8855_EFUSE_CELL * sizeof(u32),
    .stride = sizeof(u32),
    .word_size = sizeof(u32),
    .reg_read = an8855_efuse_read,
    };
    struct device *dev = &pdev.dev;
    struct nvmem_device *nvmem;
    struct regmap *regmap;
// Assign NVMEM priv to MFD regmap
    regmap = dev_get_regmap(dev.parent, core::ptr::null_mut());
    if (!regmap)
    return -ENOENT;
    an8855_nvmem_config.priv = regmap;
    an8855_nvmem_config.dev = dev;
    nvmem = devm_nvmem_register(dev, &an8855_nvmem_config);
    return PTR_ERR_OR_ZERO(nvmem);
    }
    static const struct of_device_id an8855_efuse_of_match[] = {
    { .compatible = "airoha,an8855-efuse", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, an8855_efuse_of_match);
    static struct platform_driver an8855_efuse_driver = {
    .probe = an8855_efuse_probe,
    .driver = {
    .name = "an8855-efuse",
    .of_match_table = an8855_efuse_of_match,
    },
    };
    module_platform_driver(an8855_efuse_driver);
    MODULE_AUTHOR("Christian Marangi <ansuelsmth@gmail.com>");
    MODULE_DESCRIPTION("Driver for AN8855 Switch EFUSE");
    MODULE_LICENSE("GPL");
