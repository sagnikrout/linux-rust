//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-rk808.c
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
// Clkout driver for Rockchip RK808
//
// Copyright (c) 2014, Fuzhou Rockchip Electronics Co., Ltd
//
// Author:Chris Zhong <zyw@rock-chips.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rk808_clkout {
    pub regmap: *mut regmap,
    pub clkout1_hw: clk_hw,
    pub clkout2_hw: clk_hw,
}

    static unsigned long rk808_clkout_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    return 32768;
    }
#[no_mangle]
unsafe extern "C" fn rk808_clkout2_enable(hw: *mut clk_hw, enable: bool) -> c_int {
    static int rk808_clkout2_enable(struct clk_hw *hw, bool enable)
    {
    struct rk808_clkout *rk808_clkout = container_of(hw,
    struct rk808_clkout,
    clkout2_hw);
    return regmap_update_bits(rk808_clkout.regmap, RK808_CLK32OUT_REG,
    CLK32KOUT2_EN, enable ? CLK32KOUT2_EN : 0);
    }
#[no_mangle]
unsafe extern "C" fn rk808_clkout2_prepare(hw: *mut clk_hw) -> c_int {
    static int rk808_clkout2_prepare(struct clk_hw *hw)
    {
    return rk808_clkout2_enable(hw, true);
    }
#[no_mangle]
unsafe extern "C" fn rk808_clkout2_unprepare(hw: *mut clk_hw) {
    static void rk808_clkout2_unprepare(struct clk_hw *hw)
    {
    rk808_clkout2_enable(hw, false);
    }
#[no_mangle]
unsafe extern "C" fn rk808_clkout2_is_prepared(hw: *mut clk_hw) -> c_int {
    static int rk808_clkout2_is_prepared(struct clk_hw *hw)
    {
    struct rk808_clkout *rk808_clkout = container_of(hw,
    struct rk808_clkout,
    clkout2_hw);
    uint32_t val;
    let mut ret: c_int = regmap_read(rk808_clkout.regmap, RK808_CLK32OUT_REG, &val);
    if (ret < 0)
    return ret;
    return (val & CLK32KOUT2_EN) ? 1 : 0;
    }
    static const struct clk_ops rk808_clkout1_ops = {
    .recalc_rate = rk808_clkout_recalc_rate,
    };
    static const struct clk_ops rk808_clkout2_ops = {
    .prepare = rk808_clkout2_prepare,
    .unprepare = rk808_clkout2_unprepare,
    .is_prepared = rk808_clkout2_is_prepared,
    .recalc_rate = rk808_clkout_recalc_rate,
    };
    static struct clk_hw *
    of_clk_rk808_get(struct of_phandle_args *clkspec, void *data)
    {
    struct rk808_clkout *rk808_clkout = data;
    let mut idx: c_uint = clkspec.args[0];
    if (idx >= 2) {
    pr_err("%s: invalid index %u\n", __func__, idx);
    return ERR_PTR(-EINVAL);
    }
    return idx ? &rk808_clkout.clkout2_hw : &rk808_clkout.clkout1_hw;
    }
#[no_mangle]
unsafe extern "C" fn rk817_clkout2_enable(hw: *mut clk_hw, enable: bool) -> c_int {
    static int rk817_clkout2_enable(struct clk_hw *hw, bool enable)
    {
    struct rk808_clkout *rk808_clkout = container_of(hw,
    struct rk808_clkout,
    clkout2_hw);
    return regmap_update_bits(rk808_clkout.regmap, RK817_SYS_CFG(1),
    RK817_CLK32KOUT2_EN,
    enable ? RK817_CLK32KOUT2_EN : 0);
    }
#[no_mangle]
unsafe extern "C" fn rk817_clkout2_prepare(hw: *mut clk_hw) -> c_int {
    static int rk817_clkout2_prepare(struct clk_hw *hw)
    {
    return rk817_clkout2_enable(hw, true);
    }
#[no_mangle]
unsafe extern "C" fn rk817_clkout2_unprepare(hw: *mut clk_hw) {
    static void rk817_clkout2_unprepare(struct clk_hw *hw)
    {
    rk817_clkout2_enable(hw, false);
    }
#[no_mangle]
unsafe extern "C" fn rk817_clkout2_is_prepared(hw: *mut clk_hw) -> c_int {
    static int rk817_clkout2_is_prepared(struct clk_hw *hw)
    {
    struct rk808_clkout *rk808_clkout = container_of(hw,
    struct rk808_clkout,
    clkout2_hw);
    unsigned int val;
    let mut ret: c_int = regmap_read(rk808_clkout.regmap, RK817_SYS_CFG(1), &val);
    if (ret < 0)
    return 0;
    return (val & RK817_CLK32KOUT2_EN) ? 1 : 0;
    }
    static const struct clk_ops rk817_clkout2_ops = {
    .prepare = rk817_clkout2_prepare,
    .unprepare = rk817_clkout2_unprepare,
    .is_prepared = rk817_clkout2_is_prepared,
    .recalc_rate = rk808_clkout_recalc_rate,
    };
    static const struct clk_ops *rkpmic_get_ops(long variant)
    {
    switch (variant) {
    case RK809_ID:
    case RK817_ID:
    return &rk817_clkout2_ops;
//
// For the default case, it match the following PMIC type.
// RK805_ID
// RK808_ID
// RK818_ID
//
    default:
    return &rk808_clkout2_ops;
    }
    }
#[no_mangle]
unsafe extern "C" fn rk808_clkout_probe(pdev: *mut platform_device) -> c_int {
    static int rk808_clkout_probe(struct platform_device *pdev)
    {
    struct rk808 *rk808 = dev_get_drvdata(pdev.dev.parent);
    struct device *dev = &pdev.dev;
    let mut init: clk_init_data = {};
    struct rk808_clkout *rk808_clkout;
    int ret;
    device_set_of_node_from_dev(dev, dev.parent);
    rk808_clkout = devm_kzalloc(dev,
    sizeof(*rk808_clkout), GFP_KERNEL);
    if (!rk808_clkout)
    return -ENOMEM;
    rk808_clkout.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!rk808_clkout.regmap)
    return -ENODEV;
    init.parent_names = core::ptr::null_mut();
    init.num_parents = 0;
    init.name = "rk808-clkout1";
    init.ops = &rk808_clkout1_ops;
    rk808_clkout.clkout1_hw.init = &init;
// optional override of the clockname
    of_property_read_string_index(dev.of_node, "clock-output-names",
    0, &init.name);
    ret = devm_clk_hw_register(dev, &rk808_clkout.clkout1_hw);
    if (ret)
    return ret;
    init.name = "rk808-clkout2";
    init.ops = rkpmic_get_ops(rk808.variant);
    rk808_clkout.clkout2_hw.init = &init;
// optional override of the clockname
    of_property_read_string_index(dev.of_node, "clock-output-names",
    1, &init.name);
    ret = devm_clk_hw_register(dev, &rk808_clkout.clkout2_hw);
    if (ret)
    return ret;
    return devm_of_clk_add_hw_provider(&pdev.dev, of_clk_rk808_get,
    rk808_clkout);
    }
    static struct platform_driver rk808_clkout_driver = {
    .probe = rk808_clkout_probe,
    .driver		= {
    .name	= "rk808-clkout",
    },
    };
    module_platform_driver(rk808_clkout_driver);
    MODULE_DESCRIPTION("Clkout driver for the rk808 series PMICs");
    MODULE_AUTHOR("Chris Zhong <zyw@rock-chips.com>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:rk808-clkout");
