//! Automatically rewritten from C to Rust
//! Source: drivers/clk/uniphier/clk-uniphier-mux.c
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
pub struct uniphier_clk_mux {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub reg: c_uint,
    pub masks: *const c_uint,
    pub vals: *const c_uint,
}

#[no_mangle]
unsafe extern "C" fn uniphier_clk_mux_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int uniphier_clk_mux_set_parent(struct clk_hw *hw, u8 index)
    {
    struct uniphier_clk_mux *mux = to_uniphier_clk_mux(hw);
    return regmap_write_bits(mux.regmap, mux.reg, mux.masks[index],
    mux.vals[index]);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_clk_mux_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 uniphier_clk_mux_get_parent(struct clk_hw *hw)
    {
    struct uniphier_clk_mux *mux = to_uniphier_clk_mux(hw);
    let mut num_parents: c_uint = clk_hw_get_num_parents(hw);
    int ret;
    unsigned int val;
    unsigned int i;
    ret = regmap_read(mux.regmap, mux.reg, &val);
    if (ret)
    return ret;
    for (i = 0; i < num_parents; i++)
    if ((mux.masks[i] & val) == mux.vals[i])
    return i;
    return -EINVAL;
    }
    static const struct clk_ops uniphier_clk_mux_ops = {
    .determine_rate = __clk_mux_determine_rate,
    .set_parent = uniphier_clk_mux_set_parent,
    .get_parent = uniphier_clk_mux_get_parent,
    };
    struct clk_hw *uniphier_clk_register_mux(struct device *dev,
    struct regmap *regmap,
    const char *name,
    const struct uniphier_clk_mux_data *data)
    {
    struct uniphier_clk_mux *mux;
    struct clk_init_data init;
    int ret;
    mux = devm_kzalloc(dev, sizeof(*mux), GFP_KERNEL);
    if (!mux)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &uniphier_clk_mux_ops;
    init.flags = CLK_SET_RATE_PARENT;
    init.parent_names = data.parent_names;
    init.num_parents = data.num_parents;
    mux.regmap = regmap;
    mux.reg = data.reg;
    mux.masks = data.masks;
    mux.vals = data.vals;
    mux.hw.init = &init;
    ret = devm_clk_hw_register(dev, &mux.hw);
    if (ret)
    return ERR_PTR(ret);
    return &mux.hw;
    }
