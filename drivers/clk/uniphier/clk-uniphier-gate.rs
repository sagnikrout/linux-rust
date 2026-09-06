//! Automatically rewritten from C to Rust
//! Source: drivers/clk/uniphier/clk-uniphier-gate.c
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
pub struct uniphier_clk_gate {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub reg: c_uint,
    pub bit: c_uint,
}

    container_of(_hw, struct uniphier_clk_gate, hw)
#[no_mangle]
unsafe extern "C" fn uniphier_clk_gate_endisable(hw: *mut clk_hw, enable: c_int) -> c_int {
    static int uniphier_clk_gate_endisable(struct clk_hw *hw, int enable)
    {
    struct uniphier_clk_gate *gate = to_uniphier_clk_gate(hw);
    return regmap_write_bits(gate.regmap, gate.reg, BIT(gate.bit),
    enable ? BIT(gate.bit) : 0);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_clk_gate_enable(hw: *mut clk_hw) -> c_int {
    static int uniphier_clk_gate_enable(struct clk_hw *hw)
    {
    return uniphier_clk_gate_endisable(hw, 1);
    }
#[no_mangle]
unsafe extern "C" fn uniphier_clk_gate_disable(hw: *mut clk_hw) {
    static void uniphier_clk_gate_disable(struct clk_hw *hw)
    {
    if (uniphier_clk_gate_endisable(hw, 0) < 0)
    pr_warn("failed to disable clk\n");
    }
#[no_mangle]
unsafe extern "C" fn uniphier_clk_gate_is_enabled(hw: *mut clk_hw) -> c_int {
    static int uniphier_clk_gate_is_enabled(struct clk_hw *hw)
    {
    struct uniphier_clk_gate *gate = to_uniphier_clk_gate(hw);
    unsigned int val;
    if (regmap_read(gate.regmap, gate.reg, &val) < 0)
    pr_warn("is_enabled() may return wrong result\n");
    return !!(val & BIT(gate.bit));
    }
    static const struct clk_ops uniphier_clk_gate_ops = {
    .enable = uniphier_clk_gate_enable,
    .disable = uniphier_clk_gate_disable,
    .is_enabled = uniphier_clk_gate_is_enabled,
    };
    struct clk_hw *uniphier_clk_register_gate(struct device *dev,
    struct regmap *regmap,
    const char *name,
    const struct uniphier_clk_gate_data *data)
    {
    struct uniphier_clk_gate *gate;
    struct clk_init_data init;
    int ret;
    gate = devm_kzalloc(dev, sizeof(*gate), GFP_KERNEL);
    if (!gate)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &uniphier_clk_gate_ops;
    init.flags = data.parent_name ? CLK_SET_RATE_PARENT : 0;
    init.parent_names = data.parent_name ? &data.parent_name : core::ptr::null_mut();
    init.num_parents = data.parent_name ? 1 : 0;
    gate.regmap = regmap;
    gate.reg = data.reg;
    gate.bit = data.bit;
    gate.hw.init = &init;
    ret = devm_clk_hw_register(dev, &gate.hw);
    if (ret)
    return ERR_PTR(ret);
    return &gate.hw;
    }
