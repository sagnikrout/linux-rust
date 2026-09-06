//! Automatically rewritten from C to Rust
//! Source: drivers/clk/nxp/clk-lpc18xx-creg.c
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
// Clk driver for NXP LPC18xx/43xx Configuration Registers (CREG)
//
// Copyright (C) 2015 Joachim Eastwood <manabian@gmail.com>
//

pub const LPC18XX_CREG_CREG0: c_uint = 0x004;

    enum {
    CREG_CLK_1KHZ,
    CREG_CLK_32KHZ,
    CREG_CLK_MAX,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_creg_data {
    pub hw: clk_hw,
    pub name: *const c_char,
    pub reg: *mut regmap,
    pub en_mask: c_uint,
    pub ops: *const clk_ops,
}

    {						\
    .name = _name,				\
    .en_mask = LPC18XX_CREG_CREG0_##_emask,	\
    .ops = &_ops,				\
    }
#[no_mangle]
unsafe extern "C" fn clk_creg_32k_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_creg_32k_prepare(struct clk_hw *hw)
    {
    struct clk_creg_data *creg = to_clk_creg(hw);
    int ret;
    ret = regmap_update_bits(creg.reg, LPC18XX_CREG_CREG0,
    LPC18XX_CREG_CREG0_PD32KHZ |
    LPC18XX_CREG_CREG0_RESET32KHZ, 0);
//
// Powering up the 32k oscillator takes a long while
// and sadly there aren't any status bit to poll.
//
    msleep(2500);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn clk_creg_32k_unprepare(hw: *mut clk_hw) {
    static void clk_creg_32k_unprepare(struct clk_hw *hw)
    {
    struct clk_creg_data *creg = to_clk_creg(hw);
    regmap_update_bits(creg.reg, LPC18XX_CREG_CREG0,
    LPC18XX_CREG_CREG0_PD32KHZ,
    LPC18XX_CREG_CREG0_PD32KHZ);
    }
#[no_mangle]
unsafe extern "C" fn clk_creg_32k_is_prepared(hw: *mut clk_hw) -> c_int {
    static int clk_creg_32k_is_prepared(struct clk_hw *hw)
    {
    struct clk_creg_data *creg = to_clk_creg(hw);
    u32 reg;
    regmap_read(creg.reg, LPC18XX_CREG_CREG0, &reg);
    return !(reg & LPC18XX_CREG_CREG0_PD32KHZ) &&
    !(reg & LPC18XX_CREG_CREG0_RESET32KHZ);
    }
    static unsigned long clk_creg_1k_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    return parent_rate / 32;
    }
#[no_mangle]
unsafe extern "C" fn clk_creg_enable(hw: *mut clk_hw) -> c_int {
    static int clk_creg_enable(struct clk_hw *hw)
    {
    struct clk_creg_data *creg = to_clk_creg(hw);
    return regmap_update_bits(creg.reg, LPC18XX_CREG_CREG0,
    creg.en_mask, creg.en_mask);
    }
#[no_mangle]
unsafe extern "C" fn clk_creg_disable(hw: *mut clk_hw) {
    static void clk_creg_disable(struct clk_hw *hw)
    {
    struct clk_creg_data *creg = to_clk_creg(hw);
    regmap_update_bits(creg.reg, LPC18XX_CREG_CREG0,
    creg.en_mask, 0);
    }
#[no_mangle]
unsafe extern "C" fn clk_creg_is_enabled(hw: *mut clk_hw) -> c_int {
    static int clk_creg_is_enabled(struct clk_hw *hw)
    {
    struct clk_creg_data *creg = to_clk_creg(hw);
    u32 reg;
    regmap_read(creg.reg, LPC18XX_CREG_CREG0, &reg);
    return !!(reg & creg.en_mask);
    }
    static const struct clk_ops clk_creg_32k = {
    .enable		= clk_creg_enable,
    .disable	= clk_creg_disable,
    .is_enabled	= clk_creg_is_enabled,
    .prepare	= clk_creg_32k_prepare,
    .unprepare	= clk_creg_32k_unprepare,
    .is_prepared	= clk_creg_32k_is_prepared,
    };
    static const struct clk_ops clk_creg_1k = {
    .enable		= clk_creg_enable,
    .disable	= clk_creg_disable,
    .is_enabled	= clk_creg_is_enabled,
    .recalc_rate	= clk_creg_1k_recalc_rate,
    };
    static struct clk_creg_data clk_creg_clocks[] = {
    [CREG_CLK_1KHZ]  = CREG_CLK("1khz_clk",  EN1KHZ,  clk_creg_1k),
    [CREG_CLK_32KHZ] = CREG_CLK("32khz_clk", EN32KHZ, clk_creg_32k),
    };
    static struct clk *clk_register_creg_clk(struct device *dev,
    struct clk_creg_data *creg_clk,
    const char **parent_name,
    struct regmap *syscon)
    {
    struct clk_init_data init;
    init.ops = creg_clk.ops;
    init.name = creg_clk.name;
    init.parent_names = parent_name;
    init.num_parents = 1;
    init.flags = 0;
    creg_clk.reg = syscon;
    creg_clk.hw.init = &init;
    if (dev)
    return devm_clk_register(dev, &creg_clk.hw);
    return clk_register(core::ptr::null_mut(), &creg_clk.hw);
    }
    static struct clk *clk_creg_early[CREG_CLK_MAX];
    static struct clk_onecell_data clk_creg_early_data = {
    .clks = clk_creg_early,
    .clk_num = CREG_CLK_MAX,
    };
#[no_mangle]
unsafe extern "C" fn lpc18xx_creg_clk_init(np: *mut device_node) -> void __init {
    static void __init lpc18xx_creg_clk_init(struct device_node *np)
    {
    const char *clk_32khz_parent;
    struct regmap *syscon;
    syscon = syscon_node_to_regmap(np.parent);
    if (IS_ERR(syscon)) {
    pr_err("%s: syscon lookup failed\n", __func__);
    return;
    }
    clk_32khz_parent = of_clk_get_parent_name(np, 0);
    clk_creg_early[CREG_CLK_32KHZ] =
    clk_register_creg_clk(core::ptr::null_mut(), &clk_creg_clocks[CREG_CLK_32KHZ],
    &clk_32khz_parent, syscon);
    clk_creg_early[CREG_CLK_1KHZ] = ERR_PTR(-EPROBE_DEFER);
    of_clk_add_provider(np, of_clk_src_onecell_get, &clk_creg_early_data);
    }
    CLK_OF_DECLARE_DRIVER(lpc18xx_creg_clk, "nxp,lpc1850-creg-clk",
    lpc18xx_creg_clk_init);
    static struct clk *clk_creg[CREG_CLK_MAX];
    static struct clk_onecell_data clk_creg_data = {
    .clks = clk_creg,
    .clk_num = CREG_CLK_MAX,
    };
#[no_mangle]
unsafe extern "C" fn lpc18xx_creg_clk_probe(pdev: *mut platform_device) -> c_int {
    static int lpc18xx_creg_clk_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct regmap *syscon;
    syscon = syscon_node_to_regmap(np.parent);
    if (IS_ERR(syscon)) {
    dev_err(&pdev.dev, "syscon lookup failed\n");
    return PTR_ERR(syscon);
    }
    clk_creg[CREG_CLK_32KHZ] = clk_creg_early[CREG_CLK_32KHZ];
    clk_creg[CREG_CLK_1KHZ] =
    clk_register_creg_clk(core::ptr::null_mut(), &clk_creg_clocks[CREG_CLK_1KHZ],
    &clk_creg_clocks[CREG_CLK_32KHZ].name,
    syscon);
    return of_clk_add_provider(np, of_clk_src_onecell_get, &clk_creg_data);
    }
    static const struct of_device_id lpc18xx_creg_clk_of_match[] = {
    { .compatible = "nxp,lpc1850-creg-clk" },
    {},
    };
    static struct platform_driver lpc18xx_creg_clk_driver = {
    .probe = lpc18xx_creg_clk_probe,
    .driver = {
    .name = "lpc18xx-creg-clk",
    .of_match_table = lpc18xx_creg_clk_of_match,
    },
    };
    builtin_platform_driver(lpc18xx_creg_clk_driver);
