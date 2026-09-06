//! Automatically rewritten from C to Rust
//! Source: drivers/clk/hisilicon/clkgate-separated.c
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
// Hisilicon clock separated gate driver
//
// Copyright (c) 2012-2013 Hisilicon Limited.
// Copyright (c) 2012-2013 Linaro Limited.
//
// Author: Haojian Zhuang <haojian.zhuang@linaro.org>
// Xin Li <li.xin@linaro.org>
//

// clock separated gate register offset
pub const CLKGATE_SEPARATED_ENABLE: c_uint = 0x0;
pub const CLKGATE_SEPARATED_DISABLE: c_uint = 0x4;
pub const CLKGATE_SEPARATED_STATUS: c_uint = 0x8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clkgate_separated {
    pub hw: clk_hw,
    pub /: *mut *mut *mut void __iomem enable; / enable register,
    pub /: *mut *mut u8 bit_idx; / bits in enable/disable register,
    pub flags: u8,
    pub lock: *mut spinlock_t,
}

#[no_mangle]
unsafe extern "C" fn clkgate_separated_enable(hw: *mut clk_hw) -> c_int {
    static int clkgate_separated_enable(struct clk_hw *hw)
    {
    struct clkgate_separated *sclk;
    let mut flags: c_ulong = 0;
    u32 reg;
    sclk = container_of(hw, struct clkgate_separated, hw);
    if (sclk.lock)
    spin_lock_irqsave(sclk.lock, flags);
    reg = BIT(sclk.bit_idx);
    writel_relaxed(reg, sclk.enable);
    readl_relaxed(sclk.enable + CLKGATE_SEPARATED_STATUS);
    if (sclk.lock)
    spin_unlock_irqrestore(sclk.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clkgate_separated_disable(hw: *mut clk_hw) {
    static void clkgate_separated_disable(struct clk_hw *hw)
    {
    struct clkgate_separated *sclk;
    let mut flags: c_ulong = 0;
    u32 reg;
    sclk = container_of(hw, struct clkgate_separated, hw);
    if (sclk.lock)
    spin_lock_irqsave(sclk.lock, flags);
    reg = BIT(sclk.bit_idx);
    writel_relaxed(reg, sclk.enable + CLKGATE_SEPARATED_DISABLE);
    readl_relaxed(sclk.enable + CLKGATE_SEPARATED_STATUS);
    if (sclk.lock)
    spin_unlock_irqrestore(sclk.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn clkgate_separated_is_enabled(hw: *mut clk_hw) -> c_int {
    static int clkgate_separated_is_enabled(struct clk_hw *hw)
    {
    struct clkgate_separated *sclk;
    u32 reg;
    sclk = container_of(hw, struct clkgate_separated, hw);
    reg = readl_relaxed(sclk.enable + CLKGATE_SEPARATED_STATUS);
    reg &= BIT(sclk.bit_idx);
    return reg ? 1 : 0;
    }
    static const struct clk_ops clkgate_separated_ops = {
    .enable		= clkgate_separated_enable,
    .disable	= clkgate_separated_disable,
    .is_enabled	= clkgate_separated_is_enabled,
    };
    struct clk *hisi_register_clkgate_sep(struct device *dev, const char *name,
    const char *parent_name,
    unsigned long flags,
    void __iomem *reg, u8 bit_idx,
    u8 clk_gate_flags, spinlock_t *lock)
    {
    struct clkgate_separated *sclk;
    struct clk *clk;
    struct clk_init_data init;
    sclk = kzalloc_obj(*sclk);
    if (!sclk)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &clkgate_separated_ops;
    init.flags = flags;
    init.parent_names = (parent_name ? &parent_name : core::ptr::null_mut());
    init.num_parents = (parent_name ? 1 : 0);
    sclk.enable = reg + CLKGATE_SEPARATED_ENABLE;
    sclk.bit_idx = bit_idx;
    sclk.flags = clk_gate_flags;
    sclk.hw.init = &init;
    sclk.lock = lock;
    clk = clk_register(dev, &sclk.hw);
    if (IS_ERR(clk))
    kfree(sclk);
    return clk;
    }
