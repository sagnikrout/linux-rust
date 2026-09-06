//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/apple-spmi-nvmem.c
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Apple SPMI NVMEM driver
//
// Copyright The Asahi Linux Contributors
//

    static const struct regmap_config apple_spmi_regmap_config = {
    .reg_bits	= 16,
    .val_bits	= 8,
    .max_register	= 0xffff,
    };
    static int apple_spmi_nvmem_read(void *priv, unsigned int offset, void *val,
    size_t bytes)
    {
    struct regmap *map = priv;
    return regmap_bulk_read(map, offset, val, bytes);
    }
    static int apple_spmi_nvmem_write(void *priv, unsigned int offset, void *val,
    size_t bytes)
    {
    struct regmap *map = priv;
    return regmap_bulk_write(map, offset, val, bytes);
    }
#[no_mangle]
unsafe extern "C" fn apple_spmi_nvmem_probe(sdev: *mut spmi_device) -> c_int {
    static int apple_spmi_nvmem_probe(struct spmi_device *sdev)
    {
    struct regmap *regmap;
    struct nvmem_config nvmem_cfg = {
    .dev = &sdev.dev,
    .name = "spmi_nvmem",
    .id = NVMEM_DEVID_AUTO,
    .word_size = 1,
    .stride = 1,
    .size = 0xffff,
    .reg_read = apple_spmi_nvmem_read,
    .reg_write = apple_spmi_nvmem_write,
    };
    regmap = devm_regmap_init_spmi_ext(sdev, &apple_spmi_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    nvmem_cfg.priv = regmap;
    return PTR_ERR_OR_ZERO(devm_nvmem_register(&sdev.dev, &nvmem_cfg));
    }
    static const struct of_device_id apple_spmi_nvmem_id_table[] = {
    { .compatible = "apple,spmi-nvmem" },
    { },
    };
    MODULE_DEVICE_TABLE(of, apple_spmi_nvmem_id_table);
    static struct spmi_driver apple_spmi_nvmem_driver = {
    .probe = apple_spmi_nvmem_probe,
    .driver = {
    .name = "apple-spmi-nvmem",
    .of_match_table	= apple_spmi_nvmem_id_table,
    },
    };
    module_spmi_driver(apple_spmi_nvmem_driver);
    MODULE_LICENSE("Dual MIT/GPL");
    MODULE_AUTHOR("Hector Martin <marcan@marcan.st>");
    MODULE_DESCRIPTION("Apple SPMI NVMEM driver");
