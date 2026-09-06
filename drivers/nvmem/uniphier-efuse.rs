//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/uniphier-efuse.c
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
// UniPhier eFuse driver
//
// Copyright (C) 2017 Socionext Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_efuse_priv {
    pub base: *mut void __iomem,
}

    static int uniphier_reg_read(void *context,
    unsigned int reg, void *_val, size_t bytes)
    {
    struct uniphier_efuse_priv *priv = context;
    u8 *val = _val;
    int offs;
    for (offs = 0; offs < bytes; offs += sizeof(u8))
// val++ = readb(priv->base + reg + offs);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_efuse_probe(pdev: *mut platform_device) -> c_int {
    static int uniphier_efuse_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct resource *res;
    struct nvmem_device *nvmem;
    let mut econfig: nvmem_config = {};
    struct uniphier_efuse_priv *priv;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    econfig.stride = 1;
    econfig.word_size = 1;
    econfig.read_only = true;
    econfig.reg_read = uniphier_reg_read;
    econfig.size = resource_size(res);
    econfig.priv = priv;
    econfig.dev = dev;
    econfig.add_legacy_fixed_of_cells = true;
    nvmem = devm_nvmem_register(dev, &econfig);
    return PTR_ERR_OR_ZERO(nvmem);
    }
    static const struct of_device_id uniphier_efuse_of_match[] = {
    { .compatible = "socionext,uniphier-efuse",},
    {/* sentinel */},
    };
    MODULE_DEVICE_TABLE(of, uniphier_efuse_of_match);
    static struct platform_driver uniphier_efuse_driver = {
    .probe = uniphier_efuse_probe,
    .driver = {
    .name = "uniphier-efuse",
    .of_match_table = uniphier_efuse_of_match,
    },
    };
    module_platform_driver(uniphier_efuse_driver);
    MODULE_AUTHOR("Keiji Hayashibara <hayashibara.keiji@socionext.com>");
    MODULE_DESCRIPTION("UniPhier eFuse driver");
    MODULE_LICENSE("GPL v2");
