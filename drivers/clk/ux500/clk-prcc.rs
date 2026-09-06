//! Automatically rewritten from C to Rust
//! Source: drivers/clk/ux500/clk-prcc.c
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
// PRCC clock implementation for ux500 platform.
//
// Copyright (C) 2012 ST-Ericsson SA
// Author: Ulf Hansson <ulf.hansson@linaro.org>
//

pub const PRCC_PCKEN: c_uint = 0x000;
pub const PRCC_PCKDIS: c_uint = 0x004;
pub const PRCC_KCKEN: c_uint = 0x008;
pub const PRCC_KCKDIS: c_uint = 0x00C;
pub const PRCC_PCKSR: c_uint = 0x010;
pub const PRCC_KCKSR: c_uint = 0x014;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_prcc {
    pub hw: clk_hw,
    pub base: *mut void __iomem,
    pub cg_sel: u32,
    pub is_enabled: c_int,
}

// PRCC clock operations.
#[no_mangle]
unsafe extern "C" fn clk_prcc_pclk_enable(hw: *mut clk_hw) -> c_int {
    static int clk_prcc_pclk_enable(struct clk_hw *hw)
    {
    struct clk_prcc *clk = to_clk_prcc(hw);
    writel(clk.cg_sel, (clk.base + PRCC_PCKEN));
    while (!(readl(clk.base + PRCC_PCKSR) & clk.cg_sel))
    cpu_relax();
    clk.is_enabled = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_prcc_pclk_disable(hw: *mut clk_hw) {
    static void clk_prcc_pclk_disable(struct clk_hw *hw)
    {
    struct clk_prcc *clk = to_clk_prcc(hw);
    writel(clk.cg_sel, (clk.base + PRCC_PCKDIS));
    clk.is_enabled = 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_prcc_kclk_enable(hw: *mut clk_hw) -> c_int {
    static int clk_prcc_kclk_enable(struct clk_hw *hw)
    {
    struct clk_prcc *clk = to_clk_prcc(hw);
    writel(clk.cg_sel, (clk.base + PRCC_KCKEN));
    while (!(readl(clk.base + PRCC_KCKSR) & clk.cg_sel))
    cpu_relax();
    clk.is_enabled = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_prcc_kclk_disable(hw: *mut clk_hw) {
    static void clk_prcc_kclk_disable(struct clk_hw *hw)
    {
    struct clk_prcc *clk = to_clk_prcc(hw);
    writel(clk.cg_sel, (clk.base + PRCC_KCKDIS));
    clk.is_enabled = 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_prcc_is_enabled(hw: *mut clk_hw) -> c_int {
    static int clk_prcc_is_enabled(struct clk_hw *hw)
    {
    struct clk_prcc *clk = to_clk_prcc(hw);
    return clk.is_enabled;
    }
    static const struct clk_ops clk_prcc_pclk_ops = {
    .enable = clk_prcc_pclk_enable,
    .disable = clk_prcc_pclk_disable,
    .is_enabled = clk_prcc_is_enabled,
    };
    static const struct clk_ops clk_prcc_kclk_ops = {
    .enable = clk_prcc_kclk_enable,
    .disable = clk_prcc_kclk_disable,
    .is_enabled = clk_prcc_is_enabled,
    };
    static struct clk *clk_reg_prcc(const char *name,
    const char *parent_name,
    resource_size_t phy_base,
    u32 cg_sel,
    unsigned long flags,
    const struct clk_ops *clk_prcc_ops)
    {
    struct clk_prcc *clk;
    struct clk_init_data clk_prcc_init;
    struct clk *clk_reg;
    if (!name) {
    pr_err("clk_prcc: %s invalid arguments passed\n", __func__);
    return ERR_PTR(-EINVAL);
    }
    clk = kzalloc_obj(*clk);
    if (!clk)
    return ERR_PTR(-ENOMEM);
    clk.base = ioremap(phy_base, SZ_4K);
    if (!clk.base)
    goto free_clk;
    clk.cg_sel = cg_sel;
    clk.is_enabled = 1;
    clk_prcc_init.name = name;
    clk_prcc_init.ops = clk_prcc_ops;
    clk_prcc_init.flags = flags;
    clk_prcc_init.parent_names = (parent_name ? &parent_name : core::ptr::null_mut());
    clk_prcc_init.num_parents = (parent_name ? 1 : 0);
    clk.hw.init = &clk_prcc_init;
    clk_reg = clk_register(core::ptr::null_mut(), &clk.hw);
    if (IS_ERR_OR_NULL(clk_reg))
    goto unmap_clk;
    return clk_reg;
    unmap_clk:
    iounmap(clk.base);
    free_clk:
    kfree(clk);
    pr_err("clk_prcc: %s failed to register clk\n", __func__);
    return ERR_PTR(-ENOMEM);
    }
    struct clk *clk_reg_prcc_pclk(const char *name,
    const char *parent_name,
    resource_size_t phy_base,
    u32 cg_sel,
    unsigned long flags)
    {
    return clk_reg_prcc(name, parent_name, phy_base, cg_sel, flags,
    &clk_prcc_pclk_ops);
    }
    struct clk *clk_reg_prcc_kclk(const char *name,
    const char *parent_name,
    resource_size_t phy_base,
    u32 cg_sel,
    unsigned long flags)
    {
    return clk_reg_prcc(name, parent_name, phy_base, cg_sel, flags,
    &clk_prcc_kclk_ops);
    }
