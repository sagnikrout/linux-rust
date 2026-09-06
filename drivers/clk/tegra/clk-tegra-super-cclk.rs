//! Automatically rewritten from C to Rust
//! Source: drivers/clk/tegra/clk-tegra-super-cclk.c
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
// Based on clk-super.c
// Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
//
// Based on older tegra20-cpufreq driver by Colin Cross <ccross@google.com>
// Copyright (C) 2010 Google, Inc.
//
// Author: Dmitry Osipenko <digetx@gmail.com>
// Copyright (C) 2019 GRATE-DRIVER project
//

pub const PLLP_INDEX: c_int = 4;
pub const PLLX_INDEX: c_int = 8;

    static struct tegra_clk_super_mux *cclk_super;
    static bool cclk_on_pllx;
#[no_mangle]
unsafe extern "C" fn cclk_super_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 cclk_super_get_parent(struct clk_hw *hw)
    {
    return tegra_clk_super_ops.get_parent(hw);
    }
#[no_mangle]
unsafe extern "C" fn cclk_super_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int cclk_super_set_parent(struct clk_hw *hw, u8 index)
    {
    return tegra_clk_super_ops.set_parent(hw, index);
    }
    static int cclk_super_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    return tegra_clk_super_ops.set_rate(hw, rate, parent_rate);
    }
    static unsigned long cclk_super_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct tegra_clk_super_mux *super = to_clk_super_mux(hw);
    let mut val: u32 = readl_relaxed(super.reg);
    unsigned int div2;
// check whether thermal throttling is active
    if (val & TSENSOR_SLOWDOWN)
    div2 = 1;
    else
    div2 = 0;
    if (cclk_super_get_parent(hw) == PLLX_INDEX)
    return parent_rate >> div2;
    return tegra_clk_super_ops.recalc_rate(hw, parent_rate) >> div2;
    }
    static int cclk_super_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_hw *pllp_hw = clk_hw_get_parent_by_index(hw, PLLP_INDEX);
    struct clk_hw *pllx_hw = clk_hw_get_parent_by_index(hw, PLLX_INDEX);
    struct tegra_clk_super_mux *super = to_clk_super_mux(hw);
    unsigned long pllp_rate;
    let mut rate: c_long = req.rate;
    if (WARN_ON_ONCE(!pllp_hw || !pllx_hw))
    return -EINVAL;
//
// Switch parent to PLLP for all CCLK rates that are suitable for PLLP.
// PLLX will be disabled in this case, saving some power.
//
    pllp_rate = clk_hw_get_rate(pllp_hw);
    if (rate <= pllp_rate) {
    if (super.flags & TEGRA20_SUPER_CLK)
    rate = pllp_rate;
    else {
    struct clk_rate_request parent = {
    .rate = req.rate,
    .best_parent_rate = pllp_rate,
    };
    clk_hw_get_rate_range(hw, &parent.min_rate,
    &parent.max_rate);
    tegra_clk_super_ops.determine_rate(hw, &parent);
    pllp_rate = parent.best_parent_rate;
    rate = parent.rate;
    }
    req.best_parent_rate = pllp_rate;
    req.best_parent_hw = pllp_hw;
    req.rate = rate;
    } else {
    rate = clk_hw_round_rate(pllx_hw, rate);
    req.best_parent_rate = rate;
    req.best_parent_hw = pllx_hw;
    req.rate = rate;
    }
    if (WARN_ON_ONCE(rate <= 0))
    return -EINVAL;
    return 0;
    }
    static const struct clk_ops tegra_cclk_super_ops = {
    .get_parent = cclk_super_get_parent,
    .set_parent = cclk_super_set_parent,
    .set_rate = cclk_super_set_rate,
    .recalc_rate = cclk_super_recalc_rate,
    .determine_rate = cclk_super_determine_rate,
    };
    static const struct clk_ops tegra_cclk_super_mux_ops = {
    .get_parent = cclk_super_get_parent,
    .set_parent = cclk_super_set_parent,
    .determine_rate = cclk_super_determine_rate,
    };
    struct clk *tegra_clk_register_super_cclk(const char *name,
    const char * const *parent_names, u8 num_parents,
    unsigned long flags, void __iomem *reg, u8 clk_super_flags,
    spinlock_t *lock)
    {
    struct tegra_clk_super_mux *super;
    struct clk *clk;
    struct clk_init_data init;
    u32 val;
    if (WARN_ON(cclk_super))
    return ERR_PTR(-EBUSY);
    super = kzalloc_obj(*super);
    if (!super)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.flags = flags;
    init.parent_names = parent_names;
    init.num_parents = num_parents;
    super.reg = reg;
    super.lock = lock;
    super.width = 4;
    super.flags = clk_super_flags;
    super.hw.init = &init;
    if (super.flags & TEGRA20_SUPER_CLK) {
    init.ops = &tegra_cclk_super_mux_ops;
    } else {
    init.ops = &tegra_cclk_super_ops;
    super.frac_div.reg = reg + 4;
    super.frac_div.shift = 16;
    super.frac_div.width = 8;
    super.frac_div.frac_width = 1;
    super.frac_div.lock = lock;
    super.div_ops = &tegra_clk_frac_div_ops;
    }
//
// Tegra30+ has the following CPUG clock topology:
//
// +---+  +-------+  +-+            +-+                +-+
// PLLP+->+   +->+DIVIDER+->+0|  +-------->+0|  ------------->+0|
// |   |  +-------+  | |  |  +---+  | |  |             | |
// PLLC+->+MUX|             | +->+  | S |  | +->+             | +->+CPU
// ...   |   |             | |  |  | K |  | |  |  +-------+  | |
// PLLX+->+-->+------------>+1|  +->+ I +->+1|  +->+ DIV2  +->+1|
// +---+             +++     | P |  +++     |SKIPPER|  +++
// ^      | P |   ^      +-------+   ^
// |      | E |   |                  |
// PLLX_SEL+--+      | R |   |       OVERHEAT+--+
// +---+   |
// |
// SUPER_CDIV_ENB+--+
//
// Tegra20 is similar, but simpler. It doesn't have the divider and
// thermal DIV2 skipper.
//
// At least for now we're not going to use clock-skipper, hence let's
// ensure that it is disabled.
//
    val = readl_relaxed(reg + 4);
    val &= ~SUPER_CDIV_ENB;
    writel_relaxed(val, reg + 4);
    clk = clk_register(core::ptr::null_mut(), &super.hw);
    if (IS_ERR(clk))
    kfree(super);
    else
    cclk_super = super;
    return clk;
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_cclk_pre_pllx_rate_change() -> c_int {
    int tegra_cclk_pre_pllx_rate_change(void)
    {
    if (IS_ERR_OR_NULL(cclk_super))
    return -EINVAL;
    if (cclk_super_get_parent(&cclk_super.hw) == PLLX_INDEX)
    cclk_on_pllx = true;
    else
    cclk_on_pllx = false;
//
// CPU needs to be temporarily re-parented away from PLLX if PLLX
// changes its rate. PLLP is a safe parent for CPU on all Tegra SoCs.
//
    if (cclk_on_pllx)
    cclk_super_set_parent(&cclk_super.hw, PLLP_INDEX);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_cclk_post_pllx_rate_change() {
    void tegra_cclk_post_pllx_rate_change(void)
    {
    if (cclk_on_pllx)
    cclk_super_set_parent(&cclk_super.hw, PLLX_INDEX);
    }
