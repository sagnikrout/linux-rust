//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/layerscape-sfp.c
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
// Layerscape SFP driver
//
// Copyright (c) 2022 Michael Walle <michael@walle.cc>
//

pub const LAYERSCAPE_SFP_OTP_OFFSET: c_uint = 0x0200;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct layerscape_sfp_priv {
    pub regmap: *mut regmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct layerscape_sfp_data {
    pub size: c_int,
    pub endian: enum regmap_endian,
}

    static int layerscape_sfp_read(void *context, unsigned int offset, void *val,
    size_t bytes)
    {
    struct layerscape_sfp_priv *priv = context;
    return regmap_bulk_read(priv.regmap,
    LAYERSCAPE_SFP_OTP_OFFSET + offset, val,
    bytes / 4);
    }
    static struct nvmem_config layerscape_sfp_nvmem_config = {
    .name = "fsl-sfp",
    .reg_read = layerscape_sfp_read,
    .word_size = 4,
    .stride = 4,
    };
#[no_mangle]
unsafe extern "C" fn layerscape_sfp_probe(pdev: *mut platform_device) -> c_int {
    static int layerscape_sfp_probe(struct platform_device *pdev)
    {
    const struct layerscape_sfp_data *data;
    struct layerscape_sfp_priv *priv;
    struct nvmem_device *nvmem;
    let mut config: regmap_config = { 0 };
    void __iomem *base;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    data = device_get_match_data(&pdev.dev);
    config.reg_bits = 32;
    config.reg_stride = 4;
    config.val_bits = 32;
    config.val_format_endian = data.endian;
    config.max_register = LAYERSCAPE_SFP_OTP_OFFSET + data.size - 4;
    priv.regmap = devm_regmap_init_mmio(&pdev.dev, base, &config);
    if (IS_ERR(priv.regmap))
    return PTR_ERR(priv.regmap);
    layerscape_sfp_nvmem_config.size = data.size;
    layerscape_sfp_nvmem_config.dev = &pdev.dev;
    layerscape_sfp_nvmem_config.priv = priv;
    nvmem = devm_nvmem_register(&pdev.dev, &layerscape_sfp_nvmem_config);
    return PTR_ERR_OR_ZERO(nvmem);
    }
    static const struct layerscape_sfp_data ls1021a_data = {
    .size = 0x88,
    .endian = REGMAP_ENDIAN_BIG,
    };
    static const struct layerscape_sfp_data ls1028a_data = {
    .size = 0x88,
    .endian = REGMAP_ENDIAN_LITTLE,
    };
    static const struct of_device_id layerscape_sfp_dt_ids[] = {
    { .compatible = "fsl,ls1021a-sfp", .data = &ls1021a_data },
    { .compatible = "fsl,ls1028a-sfp", .data = &ls1028a_data },
    {},
    };
    MODULE_DEVICE_TABLE(of, layerscape_sfp_dt_ids);
    static struct platform_driver layerscape_sfp_driver = {
    .probe	= layerscape_sfp_probe,
    .driver = {
    .name	= "layerscape_sfp",
    .of_match_table = layerscape_sfp_dt_ids,
    },
    };
    module_platform_driver(layerscape_sfp_driver);
    MODULE_AUTHOR("Michael Walle <michael@walle.cc>");
    MODULE_DESCRIPTION("Layerscape Security Fuse Processor driver");
    MODULE_LICENSE("GPL");
