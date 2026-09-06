//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-uniphier-glue.c
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
// reset-uniphier-glue.c - Glue layer reset driver for UniPhier
// Copyright 2018 Socionext Inc.
// Author: Kunihiko Hayashi <hayashi.kunihiko@socionext.com>

pub const MAX_CLKS: c_int = 2;
pub const MAX_RSTS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_glue_reset_soc_data {
    pub nclks: c_int,
    pub clock_names: *const *const c_char,
    pub nrsts: c_int,
    pub reset_names: *const *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_glue_reset_priv {
    pub clk: [clk_bulk_data; MAX_CLKS],
    pub rst: [reset_control_bulk_data; MAX_RSTS],
    pub rdata: reset_simple_data,
    pub data: *const uniphier_glue_reset_soc_data,
}

#[no_mangle]
unsafe extern "C" fn uniphier_clk_disable(_priv: *mut c_void) {
    static void uniphier_clk_disable(void *_priv)
    {
    struct uniphier_glue_reset_priv *priv = _priv;
    clk_bulk_disable_unprepare(priv.data.nclks, priv.clk);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_glue_reset_probe(pdev: *mut platform_device) -> c_int {
    static int uniphier_glue_reset_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct uniphier_glue_reset_priv *priv;
    struct resource *res;
    int i, ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.data = of_device_get_match_data(dev);
    if (WARN_ON(!priv.data || priv.data.nclks > MAX_CLKS ||
    priv.data.nrsts > MAX_RSTS))
    return -EINVAL;
    priv.rdata.membase = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(priv.rdata.membase))
    return PTR_ERR(priv.rdata.membase);
    for (i = 0; i < priv.data.nclks; i++)
    priv.clk[i].id = priv.data.clock_names[i];
    ret = devm_clk_bulk_get(dev, priv.data.nclks, priv.clk);
    if (ret)
    return ret;
    ret = clk_bulk_prepare_enable(priv.data.nclks, priv.clk);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev, uniphier_clk_disable, priv);
    if (ret)
    return ret;
    for (i = 0; i < priv.data.nrsts; i++)
    priv.rst[i].id = priv.data.reset_names[i];
    ret = devm_reset_control_bulk_get_shared_deasserted(dev,
    priv.data.nrsts,
    priv.rst);
    if (ret)
    return ret;
    spin_lock_init(&priv.rdata.lock);
    priv.rdata.rcdev.owner = THIS_MODULE;
    priv.rdata.rcdev.nr_resets = resource_size(res) * BITS_PER_BYTE;
    priv.rdata.rcdev.ops = &reset_simple_ops;
    priv.rdata.rcdev.of_node = dev.of_node;
    priv.rdata.active_low = true;
    return devm_reset_controller_register(dev, &priv.rdata.rcdev);
    }
    static const char * const uniphier_pro4_clock_reset_names[] = {
    "gio", "link",
    };
    static const struct uniphier_glue_reset_soc_data uniphier_pro4_data = {
    .nclks = ARRAY_SIZE(uniphier_pro4_clock_reset_names),
    .clock_names = uniphier_pro4_clock_reset_names,
    .nrsts = ARRAY_SIZE(uniphier_pro4_clock_reset_names),
    .reset_names = uniphier_pro4_clock_reset_names,
    };
    static const char * const uniphier_pxs2_clock_reset_names[] = {
    "link",
    };
    static const struct uniphier_glue_reset_soc_data uniphier_pxs2_data = {
    .nclks = ARRAY_SIZE(uniphier_pxs2_clock_reset_names),
    .clock_names = uniphier_pxs2_clock_reset_names,
    .nrsts = ARRAY_SIZE(uniphier_pxs2_clock_reset_names),
    .reset_names = uniphier_pxs2_clock_reset_names,
    };
    static const struct of_device_id uniphier_glue_reset_match[] = {
    {
    .compatible = "socionext,uniphier-pro4-usb3-reset",
    .data = &uniphier_pro4_data,
    },
    {
    .compatible = "socionext,uniphier-pro5-usb3-reset",
    .data = &uniphier_pro4_data,
    },
    {
    .compatible = "socionext,uniphier-pxs2-usb3-reset",
    .data = &uniphier_pxs2_data,
    },
    {
    .compatible = "socionext,uniphier-ld20-usb3-reset",
    .data = &uniphier_pxs2_data,
    },
    {
    .compatible = "socionext,uniphier-pxs3-usb3-reset",
    .data = &uniphier_pxs2_data,
    },
    {
    .compatible = "socionext,uniphier-nx1-usb3-reset",
    .data = &uniphier_pxs2_data,
    },
    {
    .compatible = "socionext,uniphier-pro4-ahci-reset",
    .data = &uniphier_pro4_data,
    },
    {
    .compatible = "socionext,uniphier-pxs2-ahci-reset",
    .data = &uniphier_pxs2_data,
    },
    {
    .compatible = "socionext,uniphier-pxs3-ahci-reset",
    .data = &uniphier_pxs2_data,
    },
    { /* Sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, uniphier_glue_reset_match);
    static struct platform_driver uniphier_glue_reset_driver = {
    .probe = uniphier_glue_reset_probe,
    .driver = {
    .name = "uniphier-glue-reset",
    .of_match_table = uniphier_glue_reset_match,
    },
    };
    module_platform_driver(uniphier_glue_reset_driver);
    MODULE_AUTHOR("Kunihiko Hayashi <hayashi.kunihiko@socionext.com>");
    MODULE_DESCRIPTION("UniPhier Glue layer reset driver");
    MODULE_LICENSE("GPL");
