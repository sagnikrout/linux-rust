//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mvebu/armada-37xx-xtal.c
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
// Marvell Armada 37xx SoC xtal clocks
//
// Copyright (C) 2016 Marvell
//
// Gregory CLEMENT <gregory.clement@free-electrons.com>
//

pub const NB_GPIO1_LATCH: c_uint = 0x8;

#[no_mangle]
unsafe extern "C" fn armada_3700_xtal_clock_probe(pdev: *mut platform_device) -> c_int {
    static int armada_3700_xtal_clock_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    const char *xtal_name = "xtal";
    struct device_node *parent;
    struct regmap *regmap;
    struct clk_hw *xtal_hw;
    unsigned int rate;
    u32 reg;
    int ret;
    xtal_hw = devm_kzalloc(&pdev.dev, sizeof(*xtal_hw), GFP_KERNEL);
    if (!xtal_hw)
    return -ENOMEM;
    platform_set_drvdata(pdev, xtal_hw);
    parent = np.parent;
    if (!parent) {
    dev_err(&pdev.dev, "no parent\n");
    return -ENODEV;
    }
    regmap = syscon_node_to_regmap(parent);
    if (IS_ERR(regmap)) {
    dev_err(&pdev.dev, "cannot get regmap\n");
    return PTR_ERR(regmap);
    }
    ret = regmap_read(regmap, NB_GPIO1_LATCH, &reg);
    if (ret) {
    dev_err(&pdev.dev, "cannot read from regmap\n");
    return ret;
    }
    if (reg & XTAL_MODE)
    rate = 40000000;
    else
    rate = 25000000;
    of_property_read_string_index(np, "clock-output-names", 0, &xtal_name);
    xtal_hw = clk_hw_register_fixed_rate(core::ptr::null_mut(), xtal_name, core::ptr::null_mut(), 0, rate);
    if (IS_ERR(xtal_hw))
    return PTR_ERR(xtal_hw);
    ret = of_clk_add_hw_provider(np, of_clk_hw_simple_get, xtal_hw);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn armada_3700_xtal_clock_remove(pdev: *mut platform_device) {
    static void armada_3700_xtal_clock_remove(struct platform_device *pdev)
    {
    of_clk_del_provider(pdev.dev.of_node);
    }
    static const struct of_device_id armada_3700_xtal_clock_of_match[] = {
    { .compatible = "marvell,armada-3700-xtal-clock", },
    { }
    };
    static struct platform_driver armada_3700_xtal_clock_driver = {
    .probe = armada_3700_xtal_clock_probe,
    .remove = armada_3700_xtal_clock_remove,
    .driver		= {
    .name	= "marvell-armada-3700-xtal-clock",
    .of_match_table = armada_3700_xtal_clock_of_match,
    },
    };
    builtin_platform_driver(armada_3700_xtal_clock_driver);
