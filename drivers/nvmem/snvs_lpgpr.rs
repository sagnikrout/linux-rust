//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/snvs_lpgpr.c
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
// Copyright (c) 2015 Pengutronix, Steffen Trumtrar <kernel@pengutronix.de>
// Copyright (c) 2017 Pengutronix, Oleksij Rempel <kernel@pengutronix.de>
//

pub const IMX6Q_SNVS_HPLR: c_uint = 0x00;
pub const IMX6Q_SNVS_LPLR: c_uint = 0x34;
pub const IMX6Q_SNVS_LPGPR: c_uint = 0x68;
pub const IMX7D_SNVS_HPLR: c_uint = 0x00;
pub const IMX7D_SNVS_LPLR: c_uint = 0x34;
pub const IMX7D_SNVS_LPGPR: c_uint = 0x90;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snvs_lpgpr_cfg {
    pub offset: c_int,
    pub offset_hplr: c_int,
    pub offset_lplr: c_int,
    pub size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snvs_lpgpr_priv {
    pub dev: *mut device_d,
    pub regmap: *mut regmap,
    pub cfg: nvmem_config,
    pub dcfg: *const snvs_lpgpr_cfg,
}

    static const struct snvs_lpgpr_cfg snvs_lpgpr_cfg_imx6q = {
    .offset		= IMX6Q_SNVS_LPGPR,
    .offset_hplr	= IMX6Q_SNVS_HPLR,
    .offset_lplr	= IMX6Q_SNVS_LPLR,
    .size		= 4,
    };
    static const struct snvs_lpgpr_cfg snvs_lpgpr_cfg_imx7d = {
    .offset		= IMX7D_SNVS_LPGPR,
    .offset_hplr	= IMX7D_SNVS_HPLR,
    .offset_lplr	= IMX7D_SNVS_LPLR,
    .size		= 16,
    };
    static int snvs_lpgpr_write(void *context, unsigned int offset, void *val,
    size_t bytes)
    {
    struct snvs_lpgpr_priv *priv = context;
    const struct snvs_lpgpr_cfg *dcfg = priv.dcfg;
    unsigned int lock_reg;
    int ret;
    ret = regmap_read(priv.regmap, dcfg.offset_hplr, &lock_reg);
    if (ret < 0)
    return ret;
    if (lock_reg & IMX_GPR_SL)
    return -EPERM;
    ret = regmap_read(priv.regmap, dcfg.offset_lplr, &lock_reg);
    if (ret < 0)
    return ret;
    if (lock_reg & IMX_GPR_HL)
    return -EPERM;
    return regmap_bulk_write(priv.regmap, dcfg.offset + offset, val,
    bytes / 4);
    }
    static int snvs_lpgpr_read(void *context, unsigned int offset, void *val,
    size_t bytes)
    {
    struct snvs_lpgpr_priv *priv = context;
    const struct snvs_lpgpr_cfg *dcfg = priv.dcfg;
    return regmap_bulk_read(priv.regmap, dcfg.offset + offset,
    val, bytes / 4);
    }
#[no_mangle]
unsafe extern "C" fn snvs_lpgpr_probe(pdev: *mut platform_device) -> c_int {
    static int snvs_lpgpr_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *node = dev.of_node;
    struct device_node *syscon_node;
    struct snvs_lpgpr_priv *priv;
    struct nvmem_config *cfg;
    struct nvmem_device *nvmem;
    const struct snvs_lpgpr_cfg *dcfg;
    if (!node)
    return -ENOENT;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    dcfg = of_device_get_match_data(dev);
    if (!dcfg)
    return -EINVAL;
    syscon_node = of_get_parent(node);
    if (!syscon_node)
    return -ENODEV;
    priv.regmap = syscon_node_to_regmap(syscon_node);
    of_node_put(syscon_node);
    if (IS_ERR(priv.regmap))
    return PTR_ERR(priv.regmap);
    priv.dcfg = dcfg;
    cfg = &priv.cfg;
    cfg.priv = priv;
    cfg.name = dev_name(dev);
    cfg.dev = dev;
    cfg.stride = 4;
    cfg.word_size = 4;
    cfg.size = dcfg.size;
    cfg.owner = THIS_MODULE;
    cfg.reg_read  = snvs_lpgpr_read;
    cfg.reg_write = snvs_lpgpr_write;
    nvmem = devm_nvmem_register(dev, cfg);
    return PTR_ERR_OR_ZERO(nvmem);
    }
    static const struct of_device_id snvs_lpgpr_dt_ids[] = {
    { .compatible = "fsl,imx6q-snvs-lpgpr", .data = &snvs_lpgpr_cfg_imx6q },
    { .compatible = "fsl,imx6ul-snvs-lpgpr",
    .data = &snvs_lpgpr_cfg_imx6q },
    { .compatible = "fsl,imx7d-snvs-lpgpr",	.data = &snvs_lpgpr_cfg_imx7d },
    { },
    };
    MODULE_DEVICE_TABLE(of, snvs_lpgpr_dt_ids);
    static struct platform_driver snvs_lpgpr_driver = {
    .probe	= snvs_lpgpr_probe,
    .driver = {
    .name	= "snvs_lpgpr",
    .of_match_table = snvs_lpgpr_dt_ids,
    },
    };
    module_platform_driver(snvs_lpgpr_driver);
    MODULE_AUTHOR("Oleksij Rempel <o.rempel@pengutronix.de>");
    MODULE_DESCRIPTION("Low Power General Purpose Register in i.MX6 and i.MX7 Secure Non-Volatile Storage");
    MODULE_LICENSE("GPL v2");
