//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-gate2.c
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
// Copyright (C) 2010-2011 Canonical Ltd <jeremy.kerr@canonical.com>
// Copyright (C) 2011-2012 Mike Turquette, Linaro Ltd <mturquette@linaro.org>
//
// Gated clock implementation
//

//
// DOC: basic gateable clock which can gate and ungate its output
//
// Traits of this clock:
// prepare - clk_(un)prepare only ensures parent is (un)prepared
// enable - clk_enable and clk_disable are functional & control gating
// rate - inherits rate from parent.  No clk_set_rate support
// parent - fixed parent.  No clk_set_parent support
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_gate2 {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub bit_idx: u8,
    pub cgr_val: u8,
    pub cgr_mask: u8,
    pub flags: u8,
    pub lock: *mut spinlock_t,
    pub share_count: *mut c_uint,
}

#[no_mangle]
unsafe extern "C" fn clk_gate2_do_shared_clks(hw: *mut clk_hw, enable: bool) {
    static void clk_gate2_do_shared_clks(struct clk_hw *hw, bool enable)
    {
    struct clk_gate2 *gate = to_clk_gate2(hw);
    u32 reg;
    reg = readl(gate.reg);
    reg &= ~(gate.cgr_mask << gate.bit_idx);
    if (enable)
    reg |= (gate.cgr_val & gate.cgr_mask) << gate.bit_idx;
    writel(reg, gate.reg);
    }
#[no_mangle]
unsafe extern "C" fn clk_gate2_enable(hw: *mut clk_hw) -> c_int {
    static int clk_gate2_enable(struct clk_hw *hw)
    {
    struct clk_gate2 *gate = to_clk_gate2(hw);
    unsigned long flags;
    spin_lock_irqsave(gate.lock, flags);
    if (gate.share_count && (*gate.share_count)++ > 0)
    goto out;
    clk_gate2_do_shared_clks(hw, true);
    out:
    spin_unlock_irqrestore(gate.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_gate2_disable(hw: *mut clk_hw) {
    static void clk_gate2_disable(struct clk_hw *hw)
    {
    struct clk_gate2 *gate = to_clk_gate2(hw);
    unsigned long flags;
    spin_lock_irqsave(gate.lock, flags);
    if (gate.share_count) {
    if (WARN_ON(*gate.share_count == 0))
    goto out;
#[no_mangle]
pub unsafe extern "C" fn if(0: *mut *mut --(gate->share_count) >) -> else {
    else if (--(*gate.share_count) > 0)
    goto out;
    }
    clk_gate2_do_shared_clks(hw, false);
    out:
    spin_unlock_irqrestore(gate.lock, flags);
    }
    static int clk_gate2_reg_is_enabled(void __iomem *reg, u8 bit_idx,
    u8 cgr_val, u8 cgr_mask)
    {
    let mut val: u32 = readl(reg);
    if (((val >> bit_idx) & cgr_mask) == cgr_val)
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_gate2_is_enabled(hw: *mut clk_hw) -> c_int {
    static int clk_gate2_is_enabled(struct clk_hw *hw)
    {
    struct clk_gate2 *gate = to_clk_gate2(hw);
    unsigned long flags;
    let mut ret: c_int = 0;
    spin_lock_irqsave(gate.lock, flags);
    ret = clk_gate2_reg_is_enabled(gate.reg, gate.bit_idx,
    gate.cgr_val, gate.cgr_mask);
    spin_unlock_irqrestore(gate.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn clk_gate2_disable_unused(hw: *mut clk_hw) {
    static void clk_gate2_disable_unused(struct clk_hw *hw)
    {
    struct clk_gate2 *gate = to_clk_gate2(hw);
    unsigned long flags;
    spin_lock_irqsave(gate.lock, flags);
    if (!gate.share_count || *gate.share_count == 0)
    clk_gate2_do_shared_clks(hw, false);
    spin_unlock_irqrestore(gate.lock, flags);
    }
    static const struct clk_ops clk_gate2_ops = {
    .enable = clk_gate2_enable,
    .disable = clk_gate2_disable,
    .disable_unused = clk_gate2_disable_unused,
    .is_enabled = clk_gate2_is_enabled,
    };
    struct clk_hw *clk_hw_register_gate2(struct device *dev, const char *name,
    const char *parent_name, unsigned long flags,
    void __iomem *reg, u8 bit_idx, u8 cgr_val, u8 cgr_mask,
    u8 clk_gate2_flags, spinlock_t *lock,
    unsigned int *share_count)
    {
    struct clk_gate2 *gate;
    struct clk_hw *hw;
    struct clk_init_data init;
    int ret;
    gate = kzalloc_obj(struct clk_gate2);
    if (!gate)
    return ERR_PTR(-ENOMEM);
// struct clk_gate2 assignments
    gate.reg = reg;
    gate.bit_idx = bit_idx;
    gate.cgr_val = cgr_val;
    gate.cgr_mask = cgr_mask;
    gate.flags = clk_gate2_flags;
    gate.lock = lock;
    gate.share_count = share_count;
    init.name = name;
    init.ops = &clk_gate2_ops;
    init.flags = flags;
    init.parent_names = parent_name ? &parent_name : core::ptr::null_mut();
    init.num_parents = parent_name ? 1 : 0;
    gate.hw.init = &init;
    hw = &gate.hw;
    ret = clk_hw_register(dev, hw);
    if (ret) {
    kfree(gate);
    return ERR_PTR(ret);
    }
    return hw;
    }
    EXPORT_SYMBOL_GPL(clk_hw_register_gate2);
