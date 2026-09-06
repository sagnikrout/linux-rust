//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-zynq.c
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
// Copyright (c) 2015, National Instruments Corp.
//
// Xilinx Zynq Reset controller driver
//
// Author: Moritz Fischer <moritz.fischer@ettus.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynq_reset_data {
    pub slcr: *mut regmap,
    pub rcdev: reset_controller_dev,
    pub offset: u32,
}

    container_of((p), struct zynq_reset_data, rcdev)
    static int zynq_reset_assert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct zynq_reset_data *priv = to_zynq_reset_data(rcdev);
    let mut bank: c_int = id / BITS_PER_LONG;
    let mut offset: c_int = id % BITS_PER_LONG;
    pr_debug("%s: %s reset bank %u offset %u\n", KBUILD_MODNAME, __func__,
    bank, offset);
    return regmap_update_bits(priv.slcr,
    priv.offset + (bank * 4),
    BIT(offset),
    BIT(offset));
    }
    static int zynq_reset_deassert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct zynq_reset_data *priv = to_zynq_reset_data(rcdev);
    let mut bank: c_int = id / BITS_PER_LONG;
    let mut offset: c_int = id % BITS_PER_LONG;
    pr_debug("%s: %s reset bank %u offset %u\n", KBUILD_MODNAME, __func__,
    bank, offset);
    return regmap_update_bits(priv.slcr,
    priv.offset + (bank * 4),
    BIT(offset),
    ~BIT(offset));
    }
    static int zynq_reset_status(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct zynq_reset_data *priv = to_zynq_reset_data(rcdev);
    let mut bank: c_int = id / BITS_PER_LONG;
    let mut offset: c_int = id % BITS_PER_LONG;
    int ret;
    u32 reg;
    pr_debug("%s: %s reset bank %u offset %u\n", KBUILD_MODNAME, __func__,
    bank, offset);
    ret = regmap_read(priv.slcr, priv.offset + (bank * 4), &reg);
    if (ret)
    return ret;
    return !!(reg & BIT(offset));
    }
    static const struct reset_control_ops zynq_reset_ops = {
    .assert		= zynq_reset_assert,
    .deassert	= zynq_reset_deassert,
    .status		= zynq_reset_status,
    };
#[no_mangle]
unsafe extern "C" fn zynq_reset_probe(pdev: *mut platform_device) -> c_int {
    static int zynq_reset_probe(struct platform_device *pdev)
    {
    struct resource *res;
    struct zynq_reset_data *priv;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.slcr = syscon_regmap_lookup_by_phandle(pdev.dev.of_node,
    "syscon");
    if (IS_ERR(priv.slcr)) {
    dev_err(&pdev.dev, "unable to get zynq-slcr regmap");
    return PTR_ERR(priv.slcr);
    }
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res) {
    dev_err(&pdev.dev, "missing IO resource\n");
    return -ENODEV;
    }
    priv.offset = res.start;
    priv.rcdev.owner = THIS_MODULE;
    priv.rcdev.nr_resets = resource_size(res) / 4 * BITS_PER_LONG;
    priv.rcdev.ops = &zynq_reset_ops;
    priv.rcdev.of_node = pdev.dev.of_node;
    return devm_reset_controller_register(&pdev.dev, &priv.rcdev);
    }
    static const struct of_device_id zynq_reset_dt_ids[] = {
    { .compatible = "xlnx,zynq-reset", },
    { /* sentinel */ },
    };
    static struct platform_driver zynq_reset_driver = {
    .probe	= zynq_reset_probe,
    .driver = {
    .name		= KBUILD_MODNAME,
    .of_match_table	= zynq_reset_dt_ids,
    },
    };
    builtin_platform_driver(zynq_reset_driver);
