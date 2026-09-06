//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/qoriq-efuse.c
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
// Copyright (C) 2023  Westermo Network Technologies AB
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qoriq_efuse_priv {
    pub base: *mut void __iomem,
}

    static int qoriq_efuse_read(void *context, unsigned int offset, void *val,
    size_t bytes)
    {
    struct qoriq_efuse_priv *priv = context;
// .stride = 4 so offset is guaranteed to be aligned
    __ioread32_copy(val, priv.base + offset, bytes / 4);
// Ignore trailing bytes (there shouldn't be any)
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qoriq_efuse_probe(pdev: *mut platform_device) -> c_int {
    static int qoriq_efuse_probe(struct platform_device *pdev)
    {
    struct nvmem_config config = {
    .dev = &pdev.dev,
    .read_only = true,
    .reg_read = qoriq_efuse_read,
    .stride = sizeof(u32),
    .word_size = sizeof(u32),
    .name = "qoriq_efuse_read",
    .id = NVMEM_DEVID_AUTO,
    .root_only = true,
    };
    struct qoriq_efuse_priv *priv;
    struct nvmem_device *nvmem;
    struct resource *res;
    priv = devm_kzalloc(config.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    config.size = resource_size(res);
    config.priv = priv;
    nvmem = devm_nvmem_register(config.dev, &config);
    return PTR_ERR_OR_ZERO(nvmem);
    }
    static const struct of_device_id qoriq_efuse_of_match[] = {
    { .compatible = "fsl,t1023-sfp", },
    {/* sentinel */},
    };
    MODULE_DEVICE_TABLE(of, qoriq_efuse_of_match);
    static struct platform_driver qoriq_efuse_driver = {
    .probe = qoriq_efuse_probe,
    .driver = {
    .name = "qoriq-efuse",
    .of_match_table = qoriq_efuse_of_match,
    },
    };
    module_platform_driver(qoriq_efuse_driver);
    MODULE_AUTHOR("Richard Alpe <richard.alpe@bit42.se>");
    MODULE_DESCRIPTION("NXP QorIQ Security Fuse Processor (SFP) Reader");
    MODULE_LICENSE("GPL");
