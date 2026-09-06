//! Automatically rewritten from C to Rust
//! Source: drivers/clk/uniphier/clk-uniphier-cpugear.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2016 Socionext Inc.
// Author: Masahiro Yamada <yamada.masahiro@socionext.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_clk_cpugear {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub regbase: c_uint,
    pub mask: c_uint,
}

    container_of(_hw, struct uniphier_clk_cpugear, hw)
#[no_mangle]
unsafe extern "C" fn uniphier_clk_cpugear_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int uniphier_clk_cpugear_set_parent(struct clk_hw *hw, u8 index)
    {
    struct uniphier_clk_cpugear *gear = to_uniphier_clk_cpugear(hw);
    int ret;
    unsigned int val;
    ret = regmap_write_bits(gear.regmap,
    gear.regbase + UNIPHIER_CLK_CPUGEAR_SET,
    gear.mask, index);
    if (ret)
    return ret;
    ret = regmap_write_bits(gear.regmap,
    gear.regbase + UNIPHIER_CLK_CPUGEAR_UPD,
    UNIPHIER_CLK_CPUGEAR_UPD_BIT,
    UNIPHIER_CLK_CPUGEAR_UPD_BIT);
    if (ret)
    return ret;
    return regmap_read_poll_timeout(gear.regmap,
    gear.regbase + UNIPHIER_CLK_CPUGEAR_UPD,
    val, !(val & UNIPHIER_CLK_CPUGEAR_UPD_BIT),
    0, 1);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_clk_cpugear_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 uniphier_clk_cpugear_get_parent(struct clk_hw *hw)
    {
    struct uniphier_clk_cpugear *gear = to_uniphier_clk_cpugear(hw);
    let mut num_parents: c_int = clk_hw_get_num_parents(hw);
    int ret;
    unsigned int val;
    ret = regmap_read(gear.regmap,
    gear.regbase + UNIPHIER_CLK_CPUGEAR_STAT, &val);
    if (ret)
    return ret;
    val &= gear.mask;
    return val < num_parents ? val : -EINVAL;
    }
    static const struct clk_ops uniphier_clk_cpugear_ops = {
    .determine_rate = __clk_mux_determine_rate,
    .set_parent = uniphier_clk_cpugear_set_parent,
    .get_parent = uniphier_clk_cpugear_get_parent,
    };
    struct clk_hw *uniphier_clk_register_cpugear(struct device *dev,
    struct regmap *regmap,
    const char *name,
    const struct uniphier_clk_cpugear_data *data)
    {
    struct uniphier_clk_cpugear *gear;
    struct clk_init_data init;
    int ret;
    gear = devm_kzalloc(dev, sizeof(*gear), GFP_KERNEL);
    if (!gear)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &uniphier_clk_cpugear_ops;
    init.flags = CLK_SET_RATE_PARENT;
    init.parent_names = data.parent_names;
    init.num_parents = data.num_parents;
    gear.regmap = regmap;
    gear.regbase = data.regbase;
    gear.mask = data.mask;
    gear.hw.init = &init;
    ret = devm_clk_hw_register(dev, &gear.hw);
    if (ret)
    return ERR_PTR(ret);
    return &gear.hw;
    }
