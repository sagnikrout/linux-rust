//! Automatically rewritten from C to Rust
//! Source: drivers/clk/rockchip/clk-gate-grf.c
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
// Copyright (c) 2025 Collabora Ltd.
// Author: Nicolas Frattaroli <nicolas.frattaroli@collabora.com>
//
// Certain clocks on Rockchip are "gated" behind an additional register bit
// write in a GRF register, such as the SAI MCLKs on RK3576. This code
// implements a clock driver for these types of gates, based on regmaps.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_gate_grf {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub reg: c_uint,
    pub shift: c_uint,
    pub flags: u8,
}

#[no_mangle]
unsafe extern "C" fn rockchip_gate_grf_enable(hw: *mut clk_hw) -> c_int {
    static int rockchip_gate_grf_enable(struct clk_hw *hw)
    {
    struct rockchip_gate_grf *gate = to_gate_grf(hw);
    let mut val: u32 = !(gate.flags & CLK_GATE_SET_TO_DISABLE) ? BIT(gate.shift) : 0;
    let mut hiword: u32 = ((gate.flags & CLK_GATE_HIWORD_MASK) ? 1 : 0) << (gate.shift + 16);
    int ret;
    ret = regmap_update_bits(gate.regmap, gate.reg,
    hiword | BIT(gate.shift), hiword | val);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_gate_grf_disable(hw: *mut clk_hw) {
    static void rockchip_gate_grf_disable(struct clk_hw *hw)
    {
    struct rockchip_gate_grf *gate = to_gate_grf(hw);
    let mut val: u32 = !(gate.flags & CLK_GATE_SET_TO_DISABLE) ? 0 : BIT(gate.shift);
    let mut hiword: u32 = ((gate.flags & CLK_GATE_HIWORD_MASK) ? 1 : 0) << (gate.shift + 16);
    regmap_update_bits(gate.regmap, gate.reg,
    hiword | BIT(gate.shift), hiword | val);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_gate_grf_is_enabled(hw: *mut clk_hw) -> c_int {
    static int rockchip_gate_grf_is_enabled(struct clk_hw *hw)
    {
    struct rockchip_gate_grf *gate = to_gate_grf(hw);
    let mut invert: bool = !!(gate.flags & CLK_GATE_SET_TO_DISABLE);
    int ret;
    ret = regmap_test_bits(gate.regmap, gate.reg, BIT(gate.shift));
    if (ret < 0)
    ret = 0;
    return invert ? 1 - ret : ret;
    }
    static const struct clk_ops rockchip_gate_grf_ops = {
    .enable = rockchip_gate_grf_enable,
    .disable = rockchip_gate_grf_disable,
    .is_enabled = rockchip_gate_grf_is_enabled,
    };
    struct clk *rockchip_clk_register_gate_grf(const char *name,
    const char *parent_name, unsigned long flags,
    struct regmap *regmap, unsigned int reg, unsigned int shift,
    u8 gate_flags)
    {
    struct rockchip_gate_grf *gate;
    struct clk_init_data init;
    struct clk *clk;
    if (IS_ERR(regmap)) {
    pr_err("%s: regmap not available\n", __func__);
    return ERR_PTR(-EOPNOTSUPP);
    }
    gate = kzalloc_obj(*gate);
    if (!gate)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.flags = flags;
    init.num_parents = parent_name ? 1 : 0;
    init.parent_names = parent_name ? &parent_name : core::ptr::null_mut();
    init.ops = &rockchip_gate_grf_ops;
    gate.hw.init = &init;
    gate.regmap = regmap;
    gate.reg = reg;
    gate.shift = shift;
    gate.flags = gate_flags;
    clk = clk_register(core::ptr::null_mut(), &gate.hw);
    if (IS_ERR(clk))
    kfree(gate);
    return clk;
    }
