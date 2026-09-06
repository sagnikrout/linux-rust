//! Automatically rewritten from C to Rust
//! Source: drivers/clk/tegra/clk-periph-fixed.c
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
// Copyright (c) 2015, NVIDIA CORPORATION.  All rights reserved.
//

    static inline struct tegra_clk_periph_fixed *
    to_tegra_clk_periph_fixed(struct clk_hw *hw)
    {
    return container_of(hw, struct tegra_clk_periph_fixed, hw);
    }
#[no_mangle]
unsafe extern "C" fn tegra_clk_periph_fixed_is_enabled(hw: *mut clk_hw) -> c_int {
    static int tegra_clk_periph_fixed_is_enabled(struct clk_hw *hw)
    {
    struct tegra_clk_periph_fixed *fixed = to_tegra_clk_periph_fixed(hw);
    let mut mask: u32 = 1 << (fixed.num % 32), value;
    value = readl(fixed.base + fixed.regs.enb_reg);
    if (value & mask) {
    value = readl(fixed.base + fixed.regs.rst_reg);
    if ((value & mask) == 0)
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_clk_periph_fixed_enable(hw: *mut clk_hw) -> c_int {
    static int tegra_clk_periph_fixed_enable(struct clk_hw *hw)
    {
    struct tegra_clk_periph_fixed *fixed = to_tegra_clk_periph_fixed(hw);
    let mut mask: u32 = 1 << (fixed.num % 32);
    writel(mask, fixed.base + fixed.regs.enb_set_reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tegra_clk_periph_fixed_disable(hw: *mut clk_hw) {
    static void tegra_clk_periph_fixed_disable(struct clk_hw *hw)
    {
    struct tegra_clk_periph_fixed *fixed = to_tegra_clk_periph_fixed(hw);
    let mut mask: u32 = 1 << (fixed.num % 32);
    writel(mask, fixed.base + fixed.regs.enb_clr_reg);
    }
    static unsigned long
    tegra_clk_periph_fixed_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct tegra_clk_periph_fixed *fixed = to_tegra_clk_periph_fixed(hw);
    unsigned long long rate;
    rate = (unsigned long long)parent_rate * fixed.mul;
    do_div(rate, fixed.div);
    return (unsigned long)rate;
    }
    static const struct clk_ops tegra_clk_periph_fixed_ops = {
    .is_enabled = tegra_clk_periph_fixed_is_enabled,
    .enable = tegra_clk_periph_fixed_enable,
    .disable = tegra_clk_periph_fixed_disable,
    .recalc_rate = tegra_clk_periph_fixed_recalc_rate,
    };
    struct clk *tegra_clk_register_periph_fixed(const char *name,
    const char *parent,
    unsigned long flags,
    void __iomem *base,
    unsigned int mul,
    unsigned int div,
    unsigned int num)
    {
    const struct tegra_clk_periph_regs *regs;
    struct tegra_clk_periph_fixed *fixed;
    struct clk_init_data init;
    struct clk *clk;
    regs = get_reg_bank(num);
    if (!regs)
    return ERR_PTR(-EINVAL);
    fixed = kzalloc_obj(*fixed);
    if (!fixed)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.flags = flags;
    init.parent_names = parent ? &parent : core::ptr::null_mut();
    init.num_parents = parent ? 1 : 0;
    init.ops = &tegra_clk_periph_fixed_ops;
    fixed.base = base;
    fixed.regs = regs;
    fixed.mul = mul;
    fixed.div = div;
    fixed.num = num;
    fixed.hw.init = &init;
    clk = clk_register(core::ptr::null_mut(), &fixed.hw);
    if (IS_ERR(clk))
    kfree(fixed);
    return clk;
    }
