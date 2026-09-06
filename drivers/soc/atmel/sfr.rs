//! Automatically rewritten from C to Rust
//! Source: drivers/soc/atmel/sfr.c
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
// sfr.c - driver for special function registers
//
// Copyright (C) 2019 Bootlin.
//

pub const SFR_SN0: c_uint = 0x4c;
pub const SFR_SN_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_sfr_priv {
    pub regmap: *mut regmap,
}

    static int atmel_sfr_read(void *context, unsigned int offset,
    void *buf, size_t bytes)
    {
    struct atmel_sfr_priv *priv = context;
    return regmap_bulk_read(priv.regmap, SFR_SN0 + offset,
    buf, bytes / 4);
    }
    static struct nvmem_config atmel_sfr_nvmem_config = {
    .name = "atmel-sfr",
    .read_only = true,
    .word_size = 4,
    .stride = 4,
    .size = SFR_SN_SIZE,
    .reg_read = atmel_sfr_read,
    };
#[no_mangle]
unsafe extern "C" fn atmel_sfr_probe(pdev: *mut platform_device) -> c_int {
    static int atmel_sfr_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct nvmem_device *nvmem;
    struct atmel_sfr_priv *priv;
    u8 sn[SFR_SN_SIZE];
    int ret;
    priv = devm_kmalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.regmap = syscon_node_to_regmap(np);
    if (IS_ERR(priv.regmap)) {
    dev_err(dev, "cannot get parent's regmap\n");
    return PTR_ERR(priv.regmap);
    }
    atmel_sfr_nvmem_config.dev = dev;
    atmel_sfr_nvmem_config.priv = priv;
    nvmem = devm_nvmem_register(dev, &atmel_sfr_nvmem_config);
    if (IS_ERR(nvmem)) {
    dev_err(dev, "error registering nvmem config\n");
    return PTR_ERR(nvmem);
    }
    ret = atmel_sfr_read(priv, 0, sn, SFR_SN_SIZE);
    if (ret == 0)
    add_device_randomness(sn, SFR_SN_SIZE);
    return ret;
    }
    static const struct of_device_id atmel_sfr_dt_ids[] = {
    {
    .compatible = "atmel,sama5d2-sfr",
    }, {
    .compatible = "atmel,sama5d4-sfr",
    }, {
// sentinel
    },
    };
    MODULE_DEVICE_TABLE(of, atmel_sfr_dt_ids);
    static struct platform_driver atmel_sfr_driver = {
    .probe = atmel_sfr_probe,
    .driver = {
    .name = "atmel-sfr",
    .of_match_table = atmel_sfr_dt_ids,
    },
    };
    module_platform_driver(atmel_sfr_driver);
    MODULE_AUTHOR("Kamel Bouhara <kamel.bouhara@bootlin.com>");
    MODULE_DESCRIPTION("Atmel SFR SN driver for SAMA5D2/4 SoC family");
    MODULE_LICENSE("GPL v2");
