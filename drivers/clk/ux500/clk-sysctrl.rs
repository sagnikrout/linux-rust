//! Automatically rewritten from C to Rust
//! Source: drivers/clk/ux500/clk-sysctrl.c
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
// Sysctrl clock implementation for ux500 platform.
//
// Copyright (C) 2013 ST-Ericsson SA
// Author: Ulf Hansson <ulf.hansson@linaro.org>
//

pub const SYSCTRL_MAX_NUM_PARENTS: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_sysctrl {
    pub hw: clk_hw,
    pub dev: *mut device,
    pub parent_index: u8,
    pub reg_sel: [u16; SYSCTRL_MAX_NUM_PARENTS],
    pub reg_mask: [u8; SYSCTRL_MAX_NUM_PARENTS],
    pub reg_bits: [u8; SYSCTRL_MAX_NUM_PARENTS],
    pub rate: c_ulong,
    pub enable_delay_us: c_ulong,
}

// Sysctrl clock operations.
#[no_mangle]
unsafe extern "C" fn clk_sysctrl_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_sysctrl_prepare(struct clk_hw *hw)
    {
    int ret;
    struct clk_sysctrl *clk = to_clk_sysctrl(hw);
    ret = ab8500_sysctrl_write(clk.reg_sel[0], clk.reg_mask[0],
    clk.reg_bits[0]);
    if (!ret && clk.enable_delay_us)
    usleep_range(clk.enable_delay_us, clk.enable_delay_us +
    (clk.enable_delay_us >> 2));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn clk_sysctrl_unprepare(hw: *mut clk_hw) {
    static void clk_sysctrl_unprepare(struct clk_hw *hw)
    {
    struct clk_sysctrl *clk = to_clk_sysctrl(hw);
    if (ab8500_sysctrl_clear(clk.reg_sel[0], clk.reg_mask[0]))
    dev_err(clk.dev, "clk_sysctrl: %s fail to clear %s.\n",
    __func__, clk_hw_get_name(hw));
    }
    static unsigned long clk_sysctrl_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_sysctrl *clk = to_clk_sysctrl(hw);
    return clk.rate;
    }
#[no_mangle]
unsafe extern "C" fn clk_sysctrl_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int clk_sysctrl_set_parent(struct clk_hw *hw, u8 index)
    {
    struct clk_sysctrl *clk = to_clk_sysctrl(hw);
    let mut old_index: u8 = clk.parent_index;
    let mut ret: c_int = 0;
    if (clk.reg_sel[old_index]) {
    ret = ab8500_sysctrl_clear(clk.reg_sel[old_index],
    clk.reg_mask[old_index]);
    if (ret)
    return ret;
    }
    if (clk.reg_sel[index]) {
    ret = ab8500_sysctrl_write(clk.reg_sel[index],
    clk.reg_mask[index],
    clk.reg_bits[index]);
    if (ret) {
    if (clk.reg_sel[old_index])
    ab8500_sysctrl_write(clk.reg_sel[old_index],
    clk.reg_mask[old_index],
    clk.reg_bits[old_index]);
    return ret;
    }
    }
    clk.parent_index = index;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn clk_sysctrl_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 clk_sysctrl_get_parent(struct clk_hw *hw)
    {
    struct clk_sysctrl *clk = to_clk_sysctrl(hw);
    return clk.parent_index;
    }
    static const struct clk_ops clk_sysctrl_gate_ops = {
    .prepare = clk_sysctrl_prepare,
    .unprepare = clk_sysctrl_unprepare,
    };
    static const struct clk_ops clk_sysctrl_gate_fixed_rate_ops = {
    .prepare = clk_sysctrl_prepare,
    .unprepare = clk_sysctrl_unprepare,
    .recalc_rate = clk_sysctrl_recalc_rate,
    };
    static const struct clk_ops clk_sysctrl_set_parent_ops = {
    .determine_rate = clk_hw_determine_rate_no_reparent,
    .set_parent = clk_sysctrl_set_parent,
    .get_parent = clk_sysctrl_get_parent,
    };
    static struct clk *clk_reg_sysctrl(struct device *dev,
    const char *name,
    const char **parent_names,
    u8 num_parents,
    u16 *reg_sel,
    u8 *reg_mask,
    u8 *reg_bits,
    unsigned long rate,
    unsigned long enable_delay_us,
    unsigned long flags,
    const struct clk_ops *clk_sysctrl_ops)
    {
    struct clk_sysctrl *clk;
    struct clk_init_data clk_sysctrl_init;
    struct clk *clk_reg;
    int i;
    if (!dev)
    return ERR_PTR(-EINVAL);
    if (!name || (num_parents > SYSCTRL_MAX_NUM_PARENTS)) {
    dev_err(dev, "clk_sysctrl: invalid arguments passed\n");
    return ERR_PTR(-EINVAL);
    }
    clk = devm_kzalloc(dev, sizeof(*clk), GFP_KERNEL);
    if (!clk)
    return ERR_PTR(-ENOMEM);
// set main clock registers
    clk.reg_sel[0] = reg_sel[0];
    clk.reg_bits[0] = reg_bits[0];
    clk.reg_mask[0] = reg_mask[0];
// handle clocks with more than one parent
    for (i = 1; i < num_parents; i++) {
    clk.reg_sel[i] = reg_sel[i];
    clk.reg_bits[i] = reg_bits[i];
    clk.reg_mask[i] = reg_mask[i];
    }
    clk.parent_index = 0;
    clk.rate = rate;
    clk.enable_delay_us = enable_delay_us;
    clk.dev = dev;
    clk_sysctrl_init.name = name;
    clk_sysctrl_init.ops = clk_sysctrl_ops;
    clk_sysctrl_init.flags = flags;
    clk_sysctrl_init.parent_names = parent_names;
    clk_sysctrl_init.num_parents = num_parents;
    clk.hw.init = &clk_sysctrl_init;
    clk_reg = devm_clk_register(clk.dev, &clk.hw);
    if (IS_ERR(clk_reg))
    dev_err(dev, "clk_sysctrl: clk_register failed\n");
    return clk_reg;
    }
    struct clk *clk_reg_sysctrl_gate(struct device *dev,
    const char *name,
    const char *parent_name,
    u16 reg_sel,
    u8 reg_mask,
    u8 reg_bits,
    unsigned long enable_delay_us,
    unsigned long flags)
    {
    const char **parent_names = (parent_name ? &parent_name : core::ptr::null_mut());
    let mut num_parents: u8 = (parent_name ? 1 : 0);
    return clk_reg_sysctrl(dev, name, parent_names, num_parents,
    &reg_sel, &reg_mask, &reg_bits, 0, enable_delay_us,
    flags, &clk_sysctrl_gate_ops);
    }
    struct clk *clk_reg_sysctrl_gate_fixed_rate(struct device *dev,
    const char *name,
    const char *parent_name,
    u16 reg_sel,
    u8 reg_mask,
    u8 reg_bits,
    unsigned long rate,
    unsigned long enable_delay_us,
    unsigned long flags)
    {
    const char **parent_names = (parent_name ? &parent_name : core::ptr::null_mut());
    let mut num_parents: u8 = (parent_name ? 1 : 0);
    return clk_reg_sysctrl(dev, name, parent_names, num_parents,
    &reg_sel, &reg_mask, &reg_bits,
    rate, enable_delay_us, flags,
    &clk_sysctrl_gate_fixed_rate_ops);
    }
    struct clk *clk_reg_sysctrl_set_parent(struct device *dev,
    const char *name,
    const char **parent_names,
    u8 num_parents,
    u16 *reg_sel,
    u8 *reg_mask,
    u8 *reg_bits,
    unsigned long flags)
    {
    return clk_reg_sysctrl(dev, name, parent_names, num_parents,
    reg_sel, reg_mask, reg_bits, 0, 0, flags,
    &clk_sysctrl_set_parent_ops);
    }
