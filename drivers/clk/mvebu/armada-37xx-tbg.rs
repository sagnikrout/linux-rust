//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mvebu/armada-37xx-tbg.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Marvell Armada 37xx SoC Time Base Generator clocks
//
// Copyright (C) 2016 Marvell
//
// Gregory CLEMENT <gregory.clement@free-electrons.com>
//

pub const NUM_TBG: c_int = 4;
pub const TBG_CTRL0: c_uint = 0x4;
pub const TBG_CTRL1: c_uint = 0x8;
pub const TBG_CTRL7: c_uint = 0x20;
pub const TBG_CTRL8: c_uint = 0x30;
pub const TBG_DIV_MASK: c_uint = 0x1FF;
pub const TBG_A_REFDIV: c_int = 0;
pub const TBG_B_REFDIV: c_int = 16;
pub const TBG_A_FBDIV: c_int = 2;
pub const TBG_B_FBDIV: c_int = 18;
pub const TBG_A_VCODIV_SE: c_int = 0;
pub const TBG_B_VCODIV_SE: c_int = 16;
pub const TBG_A_VCODIV_DIFF: c_int = 1;
pub const TBG_B_VCODIV_DIFF: c_int = 17;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tbg_def {
    pub name: *mut c_char,
    pub refdiv_offset: u32,
    pub fbdiv_offset: u32,
    pub vcodiv_reg: u32,
    pub vcodiv_offset: u32,
}

    static const struct tbg_def tbg[NUM_TBG] = {
    {"TBG-A-P", TBG_A_REFDIV, TBG_A_FBDIV, TBG_CTRL8, TBG_A_VCODIV_DIFF},
    {"TBG-B-P", TBG_B_REFDIV, TBG_B_FBDIV, TBG_CTRL8, TBG_B_VCODIV_DIFF},
    {"TBG-A-S", TBG_A_REFDIV, TBG_A_FBDIV, TBG_CTRL1, TBG_A_VCODIV_SE},
    {"TBG-B-S", TBG_B_REFDIV, TBG_B_FBDIV, TBG_CTRL1, TBG_B_VCODIV_SE},
    };
#[no_mangle]
unsafe extern "C" fn tbg_get_mult(reg: *mut void __iomem, ptbg: *const tbg_def) -> c_uint {
    static unsigned int tbg_get_mult(void __iomem *reg, const struct tbg_def *ptbg)
    {
    u32 val;
    val = readl(reg + TBG_CTRL0);
    return ((val >> ptbg.fbdiv_offset) & TBG_DIV_MASK) << 2;
    }
#[no_mangle]
unsafe extern "C" fn tbg_get_div(reg: *mut void __iomem, ptbg: *const tbg_def) -> c_uint {
    static unsigned int tbg_get_div(void __iomem *reg, const struct tbg_def *ptbg)
    {
    u32 val;
    unsigned int div;
    val = readl(reg + TBG_CTRL7);
    div = (val >> ptbg.refdiv_offset) & TBG_DIV_MASK;
    if (div == 0)
    div = 1;
    val = readl(reg + ptbg.vcodiv_reg);
    div *= 1 << ((val >>  ptbg.vcodiv_offset) & TBG_DIV_MASK);
    return div;
    }
#[no_mangle]
unsafe extern "C" fn armada_3700_tbg_clock_probe(pdev: *mut platform_device) -> c_int {
    static int armada_3700_tbg_clock_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct clk_hw_onecell_data *hw_tbg_data;
    struct device *dev = &pdev.dev;
    const char *parent_name;
    struct clk *parent;
    void __iomem *reg;
    int i;
    hw_tbg_data = devm_kzalloc(&pdev.dev,
    struct_size(hw_tbg_data, hws, NUM_TBG),
    GFP_KERNEL);
    if (!hw_tbg_data)
    return -ENOMEM;
    hw_tbg_data.num = NUM_TBG;
    platform_set_drvdata(pdev, hw_tbg_data);
    parent = clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(parent)) {
    dev_err(dev, "Could get the clock parent\n");
    return -EINVAL;
    }
    parent_name = __clk_get_name(parent);
    clk_put(parent);
    reg = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(reg))
    return PTR_ERR(reg);
    for (i = 0; i < NUM_TBG; i++) {
    const char *name;
    unsigned int mult, div;
    name = tbg[i].name;
    mult = tbg_get_mult(reg, &tbg[i]);
    div = tbg_get_div(reg, &tbg[i]);
    hw_tbg_data.hws[i] = clk_hw_register_fixed_factor(core::ptr::null_mut(), name,
    parent_name, 0, mult, div);
    if (IS_ERR(hw_tbg_data.hws[i]))
    dev_err(dev, "Can't register TBG clock %s\n", name);
    }
    return of_clk_add_hw_provider(np, of_clk_hw_onecell_get, hw_tbg_data);
    }
#[no_mangle]
unsafe extern "C" fn armada_3700_tbg_clock_remove(pdev: *mut platform_device) {
    static void armada_3700_tbg_clock_remove(struct platform_device *pdev)
    {
    int i;
    struct clk_hw_onecell_data *hw_tbg_data = platform_get_drvdata(pdev);
    of_clk_del_provider(pdev.dev.of_node);
    for (i = 0; i < hw_tbg_data.num; i++)
    clk_hw_unregister_fixed_factor(hw_tbg_data.hws[i]);
    }
    static const struct of_device_id armada_3700_tbg_clock_of_match[] = {
    { .compatible = "marvell,armada-3700-tbg-clock", },
    { }
    };
    static struct platform_driver armada_3700_tbg_clock_driver = {
    .probe = armada_3700_tbg_clock_probe,
    .remove = armada_3700_tbg_clock_remove,
    .driver		= {
    .name	= "marvell-armada-3700-tbg-clock",
    .of_match_table = armada_3700_tbg_clock_of_match,
    },
    };
    builtin_platform_driver(armada_3700_tbg_clock_driver);
