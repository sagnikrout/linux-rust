//! Automatically rewritten from C to Rust
//! Source: drivers/clk/tegra/clk-tegra-fixed.c
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
// Copyright (c) 2012, 2013, NVIDIA CORPORATION.  All rights reserved.
//

pub const OSC_CTRL: c_uint = 0x50;
pub const OSC_CTRL_OSC_FREQ_SHIFT: c_int = 28;
pub const OSC_CTRL_PLL_REF_DIV_SHIFT: c_int = 26;

    (0xf << OSC_CTRL_OSC_FREQ_SHIFT))
    static u32 osc_ctrl_ctx;
    int __init tegra_osc_clk_init(void __iomem *clk_base, struct tegra_clk *clks,
    unsigned long *input_freqs, unsigned int num,
    unsigned int clk_m_div, unsigned long *osc_freq,
    unsigned long *pll_ref_freq)
    {
    struct clk *clk, *osc;
    struct clk **dt_clk;
    u32 val, pll_ref_div;
    unsigned osc_idx;
    val = readl_relaxed(clk_base + OSC_CTRL);
    osc_ctrl_ctx = val & OSC_CTRL_MASK;
    osc_idx = val >> OSC_CTRL_OSC_FREQ_SHIFT;
    if (osc_idx < num)
// osc_freq = input_freqs[osc_idx];
    else
// osc_freq = 0;
    if (!*osc_freq) {
    WARN_ON(1);
    return -EINVAL;
    }
    dt_clk = tegra_lookup_dt_id(tegra_clk_osc, clks);
    if (!dt_clk)
    return 0;
    osc = clk_register_fixed_rate(core::ptr::null_mut(), "osc", core::ptr::null_mut(), 0, *osc_freq);
// dt_clk = osc;
// osc_div2
    dt_clk = tegra_lookup_dt_id(tegra_clk_osc_div2, clks);
    if (dt_clk) {
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "osc_div2", "osc",
    0, 1, 2);
// dt_clk = clk;
    }
// osc_div4
    dt_clk = tegra_lookup_dt_id(tegra_clk_osc_div4, clks);
    if (dt_clk) {
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "osc_div4", "osc",
    0, 1, 4);
// dt_clk = clk;
    }
    dt_clk = tegra_lookup_dt_id(tegra_clk_clk_m, clks);
    if (!dt_clk)
    return 0;
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "clk_m", "osc",
    0, 1, clk_m_div);
// dt_clk = clk;
// pll_ref
    val = (val >> OSC_CTRL_PLL_REF_DIV_SHIFT) & 3;
    pll_ref_div = 1 << val;
    dt_clk = tegra_lookup_dt_id(tegra_clk_pll_ref, clks);
    if (!dt_clk)
    return 0;
    clk = clk_register_fixed_factor(core::ptr::null_mut(), "pll_ref", "osc",
    0, 1, pll_ref_div);
// dt_clk = clk;
    if (pll_ref_freq)
// pll_ref_freq = *osc_freq / pll_ref_div;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_fixed_clk_init(tegra_clks: *mut tegra_clk) -> void __init {
    void __init tegra_fixed_clk_init(struct tegra_clk *tegra_clks)
    {
    struct clk *clk;
    struct clk **dt_clk;
// clk_32k
    dt_clk = tegra_lookup_dt_id(tegra_clk_clk_32k, tegra_clks);
    if (dt_clk) {
    clk = clk_register_fixed_rate(core::ptr::null_mut(), "clk_32k", core::ptr::null_mut(), 0, 32768);
// dt_clk = clk;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_clk_osc_resume(clk_base: *mut void __iomem) {
    void tegra_clk_osc_resume(void __iomem *clk_base)
    {
    u32 val;
    val = readl_relaxed(clk_base + OSC_CTRL) & ~OSC_CTRL_MASK;
    val |= osc_ctrl_ctx;
    writel_relaxed(val, clk_base + OSC_CTRL);
    fence_udelay(2, clk_base);
    }
