//! Automatically rewritten from C to Rust
//! Source: drivers/clk/tegra/clk-pll-out.c
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
// Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
//

#[no_mangle]
unsafe extern "C" fn clk_pll_out_is_enabled(hw: *mut clk_hw) -> c_int {
    static int clk_pll_out_is_enabled(struct clk_hw *hw)
    {
    struct tegra_clk_pll_out *pll_out = to_clk_pll_out(hw);
    let mut val: u32 = readl_relaxed(pll_out.reg);
    int state;
    state = (val & pll_out_enb(pll_out)) ? 1 : 0;
    if (!(val & (pll_out_rst(pll_out))))
    state = 0;
    return state;
    }
#[no_mangle]
unsafe extern "C" fn clk_pll_out_enable(hw: *mut clk_hw) -> c_int {
    static int clk_pll_out_enable(struct clk_hw *hw)
    {
    struct tegra_clk_pll_out *pll_out = to_clk_pll_out(hw);
    let mut flags: c_ulong = 0;
    u32 val;
    if (pll_out.lock)
    spin_lock_irqsave(pll_out.lock, flags);
    val = readl_relaxed(pll_out.reg);
    val |= (pll_out_enb(pll_out) | pll_out_rst(pll_out));
    writel_relaxed(val, pll_out.reg);
    udelay(2);
    if (pll_out.lock)
    spin_unlock_irqrestore(pll_out.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_pll_out_disable(hw: *mut clk_hw) {
    static void clk_pll_out_disable(struct clk_hw *hw)
    {
    struct tegra_clk_pll_out *pll_out = to_clk_pll_out(hw);
    let mut flags: c_ulong = 0;
    u32 val;
    if (pll_out.lock)
    spin_lock_irqsave(pll_out.lock, flags);
    val = readl_relaxed(pll_out.reg);
    val &= ~(pll_out_enb(pll_out) | pll_out_rst(pll_out));
    writel_relaxed(val, pll_out.reg);
    udelay(2);
    if (pll_out.lock)
    spin_unlock_irqrestore(pll_out.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn tegra_clk_pll_out_restore_context(hw: *mut clk_hw) {
    static void tegra_clk_pll_out_restore_context(struct clk_hw *hw)
    {
    if (!__clk_get_enable_count(hw.clk))
    clk_pll_out_disable(hw);
    else
    clk_pll_out_enable(hw);
    }
    const struct clk_ops tegra_clk_pll_out_ops = {
    .is_enabled = clk_pll_out_is_enabled,
    .enable = clk_pll_out_enable,
    .disable = clk_pll_out_disable,
    .restore_context = tegra_clk_pll_out_restore_context,
    };
    struct clk *tegra_clk_register_pll_out(const char *name,
    const char *parent_name, void __iomem *reg, u8 enb_bit_idx,
    u8 rst_bit_idx, unsigned long flags, u8 pll_out_flags,
    spinlock_t *lock)
    {
    struct tegra_clk_pll_out *pll_out;
    struct clk *clk;
    struct clk_init_data init;
    pll_out = kzalloc_obj(*pll_out);
    if (!pll_out)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &tegra_clk_pll_out_ops;
    init.parent_names = (parent_name ? &parent_name : core::ptr::null_mut());
    init.num_parents = (parent_name ? 1 : 0);
    init.flags = flags;
    pll_out.reg = reg;
    pll_out.enb_bit_idx = enb_bit_idx;
    pll_out.rst_bit_idx = rst_bit_idx;
    pll_out.flags = pll_out_flags;
    pll_out.lock = lock;
// Data in .init is copied by clk_register(), so stack variable OK
    pll_out.hw.init = &init;
    clk = clk_register(core::ptr::null_mut(), &pll_out.hw);
    if (IS_ERR(clk))
    kfree(pll_out);
    return clk;
    }
