//! Automatically rewritten from C to Rust
//! Source: drivers/clk/renesas/rcar-usb2-clock-sel.c
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
// Renesas R-Car USB2.0 clock selector
//
// Copyright (C) 2017 Renesas Electronics Corp.
//
// Based on renesas-cpg-mssr.c
//
// Copyright (C) 2015 Glider bvba
//

pub const USB20_CLKSET0: c_uint = 0x00;

    static const struct clk_bulk_data rcar_usb2_clocks[] = {
    { .id = "ehci_ohci", },
    { .id = "hs-usb-if", },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb2_clock_sel_priv {
    pub base: *mut void __iomem,
    pub hw: clk_hw,
    pub clks: [clk_bulk_data; ARRAY_SIZE(rcar_usb2_clocks)],
    pub rsts: *mut reset_control,
    pub extal: bool,
    pub xtal: bool,
}

#[no_mangle]
unsafe extern "C" fn usb2_clock_sel_enable_extal_only(priv: *mut usb2_clock_sel_priv) {
    static void usb2_clock_sel_enable_extal_only(struct usb2_clock_sel_priv *priv)
    {
    let mut val: u16 = readw(priv.base + USB20_CLKSET0);
    pr_debug("%s: enter %d %d %x\n", __func__,
    priv.extal, priv.xtal, val);
    if (priv.extal && !priv.xtal && val != CLKSET0_EXTAL_ONLY)
    writew(CLKSET0_EXTAL_ONLY, priv.base + USB20_CLKSET0);
    }
#[no_mangle]
unsafe extern "C" fn usb2_clock_sel_disable_extal_only(priv: *mut usb2_clock_sel_priv) {
    static void usb2_clock_sel_disable_extal_only(struct usb2_clock_sel_priv *priv)
    {
    if (priv.extal && !priv.xtal)
    writew(CLKSET0_PRIVATE, priv.base + USB20_CLKSET0);
    }
#[no_mangle]
unsafe extern "C" fn usb2_clock_sel_enable(hw: *mut clk_hw) -> c_int {
    static int usb2_clock_sel_enable(struct clk_hw *hw)
    {
    struct usb2_clock_sel_priv *priv = to_priv(hw);
    int ret;
    ret = reset_control_deassert(priv.rsts);
    if (ret)
    return ret;
    ret = clk_bulk_prepare_enable(ARRAY_SIZE(priv.clks), priv.clks);
    if (ret) {
    reset_control_assert(priv.rsts);
    return ret;
    }
    usb2_clock_sel_enable_extal_only(priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn usb2_clock_sel_disable(hw: *mut clk_hw) {
    static void usb2_clock_sel_disable(struct clk_hw *hw)
    {
    struct usb2_clock_sel_priv *priv = to_priv(hw);
    usb2_clock_sel_disable_extal_only(priv);
    clk_bulk_disable_unprepare(ARRAY_SIZE(priv.clks), priv.clks);
    reset_control_assert(priv.rsts);
    }
//
// This module seems a mux, but this driver assumes a gate because
// ehci/ohci platform drivers don't support clk_set_parent() for now.
// If this driver acts as a gate, ehci/ohci-platform drivers don't need
// any modification.
//
    static const struct clk_ops usb2_clock_sel_clock_ops = {
    .enable = usb2_clock_sel_enable,
    .disable = usb2_clock_sel_disable,
    };
    static const struct of_device_id rcar_usb2_clock_sel_match[] = {
    { .compatible = "renesas,rcar-gen3-usb2-clock-sel" },
    { }
    };
#[no_mangle]
unsafe extern "C" fn rcar_usb2_clock_sel_suspend(dev: *mut device) -> c_int {
    static int rcar_usb2_clock_sel_suspend(struct device *dev)
    {
    struct usb2_clock_sel_priv *priv = dev_get_drvdata(dev);
    usb2_clock_sel_disable_extal_only(priv);
    pm_runtime_put(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rcar_usb2_clock_sel_resume(dev: *mut device) -> c_int {
    static int rcar_usb2_clock_sel_resume(struct device *dev)
    {
    struct usb2_clock_sel_priv *priv = dev_get_drvdata(dev);
    pm_runtime_get_sync(dev);
    usb2_clock_sel_enable_extal_only(priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rcar_usb2_clock_sel_remove(pdev: *mut platform_device) {
    static void rcar_usb2_clock_sel_remove(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    of_clk_del_provider(dev.of_node);
    pm_runtime_put(dev);
    pm_runtime_disable(dev);
    }
#[no_mangle]
unsafe extern "C" fn rcar_usb2_clock_sel_probe(pdev: *mut platform_device) -> c_int {
    static int rcar_usb2_clock_sel_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct usb2_clock_sel_priv *priv;
    struct clk *clk;
    let mut init: clk_init_data = {};
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.base))
    return PTR_ERR(priv.base);
    memcpy(priv.clks, rcar_usb2_clocks, sizeof(priv.clks));
    ret = devm_clk_bulk_get(dev, ARRAY_SIZE(priv.clks), priv.clks);
    if (ret < 0)
    return ret;
    priv.rsts = devm_reset_control_array_get_shared(dev);
    if (IS_ERR(priv.rsts))
    return PTR_ERR(priv.rsts);
    clk = devm_clk_get(dev, "usb_extal");
    if (!IS_ERR(clk) && !clk_prepare_enable(clk)) {
    priv.extal = !!clk_get_rate(clk);
    clk_disable_unprepare(clk);
    }
    clk = devm_clk_get(dev, "usb_xtal");
    if (!IS_ERR(clk) && !clk_prepare_enable(clk)) {
    priv.xtal = !!clk_get_rate(clk);
    clk_disable_unprepare(clk);
    }
    if (!priv.extal && !priv.xtal) {
    dev_err(dev, "This driver needs usb_extal or usb_xtal\n");
    return -ENOENT;
    }
    pm_runtime_enable(dev);
    pm_runtime_get_sync(dev);
    platform_set_drvdata(pdev, priv);
    dev_set_drvdata(dev, priv);
    init.name = "rcar_usb2_clock_sel";
    init.ops = &usb2_clock_sel_clock_ops;
    priv.hw.init = &init;
    ret = devm_clk_hw_register(dev, &priv.hw);
    if (ret)
    goto pm_put;
    ret = of_clk_add_hw_provider(np, of_clk_hw_simple_get, &priv.hw);
    if (ret)
    goto pm_put;
    return 0;
    pm_put:
    pm_runtime_put(dev);
    pm_runtime_disable(dev);
    return ret;
    }
    static const struct dev_pm_ops rcar_usb2_clock_sel_pm_ops = {
    .suspend	= rcar_usb2_clock_sel_suspend,
    .resume		= rcar_usb2_clock_sel_resume,
    };
    static struct platform_driver rcar_usb2_clock_sel_driver = {
    .driver		= {
    .name	= "rcar-usb2-clock-sel",
    .of_match_table = rcar_usb2_clock_sel_match,
    .pm	= &rcar_usb2_clock_sel_pm_ops,
    },
    .probe		= rcar_usb2_clock_sel_probe,
    .remove		= rcar_usb2_clock_sel_remove,
    };
    builtin_platform_driver(rcar_usb2_clock_sel_driver);
    MODULE_DESCRIPTION("Renesas R-Car USB2 clock selector Driver");
    MODULE_LICENSE("GPL v2");
