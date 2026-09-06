//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/apple-efuses.c
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
// Apple SoC eFuse driver
//
// Copyright (C) The Asahi Linux Contributors
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_efuses_priv {
    pub fuses: *mut void __iomem,
}

    static int apple_efuses_read(void *context, unsigned int offset, void *val,
    size_t bytes)
    {
    struct apple_efuses_priv *priv = context;
    u32 *dst = val;
    while (bytes >= sizeof(u32)) {
// dst++ = readl_relaxed(priv->fuses + offset);
    bytes -= sizeof(u32);
    offset += sizeof(u32);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apple_efuses_probe(pdev: *mut platform_device) -> c_int {
    static int apple_efuses_probe(struct platform_device *pdev)
    {
    struct apple_efuses_priv *priv;
    struct resource *res;
    struct nvmem_config config = {
    .dev = &pdev.dev,
    .add_legacy_fixed_of_cells = true,
    .read_only = true,
    .reg_read = apple_efuses_read,
    .stride = sizeof(u32),
    .word_size = sizeof(u32),
    .name = "apple_efuses_nvmem",
    .id = NVMEM_DEVID_AUTO,
    .root_only = true,
    };
    priv = devm_kzalloc(config.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.fuses = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(priv.fuses))
    return PTR_ERR(priv.fuses);
    config.priv = priv;
    config.size = resource_size(res);
    return PTR_ERR_OR_ZERO(devm_nvmem_register(config.dev, &config));
    }
    static const struct of_device_id apple_efuses_of_match[] = {
    { .compatible = "apple,efuses", },
    {}
    };
    MODULE_DEVICE_TABLE(of, apple_efuses_of_match);
    static struct platform_driver apple_efuses_driver = {
    .driver = {
    .name = "apple_efuses",
    .of_match_table = apple_efuses_of_match,
    },
    .probe = apple_efuses_probe,
    };
    module_platform_driver(apple_efuses_driver);
    MODULE_AUTHOR("Sven Peter <sven@svenpeter.dev>");
    MODULE_DESCRIPTION("Apple SoC eFuse driver");
    MODULE_LICENSE("GPL");
