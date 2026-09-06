//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mmp/clk-apmu.c
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
// mmp AXI peripharal clock operation source file
//
// Copyright (C) 2012 Marvell
// Chao Xie <xiechao.mail@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_apmu {
    pub hw: clk_hw,
    pub base: *mut void __iomem,
    pub rst_mask: u32,
    pub enable_mask: u32,
    pub lock: *mut spinlock_t,
}

#[no_mangle]
unsafe extern "C" fn clk_apmu_enable(hw: *mut clk_hw) -> c_int {
    static int clk_apmu_enable(struct clk_hw *hw)
    {
    struct clk_apmu *apmu = to_clk_apmu(hw);
    unsigned long data;
    let mut flags: c_ulong = 0;
    if (apmu.lock)
    spin_lock_irqsave(apmu.lock, flags);
    data = readl_relaxed(apmu.base) | apmu.enable_mask;
    writel_relaxed(data, apmu.base);
    if (apmu.lock)
    spin_unlock_irqrestore(apmu.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_apmu_disable(hw: *mut clk_hw) {
    static void clk_apmu_disable(struct clk_hw *hw)
    {
    struct clk_apmu *apmu = to_clk_apmu(hw);
    unsigned long data;
    let mut flags: c_ulong = 0;
    if (apmu.lock)
    spin_lock_irqsave(apmu.lock, flags);
    data = readl_relaxed(apmu.base) & ~apmu.enable_mask;
    writel_relaxed(data, apmu.base);
    if (apmu.lock)
    spin_unlock_irqrestore(apmu.lock, flags);
    }
    static const struct clk_ops clk_apmu_ops = {
    .enable = clk_apmu_enable,
    .disable = clk_apmu_disable,
    };
    struct clk *mmp_clk_register_apmu(const char *name, const char *parent_name,
    void __iomem *base, u32 enable_mask, spinlock_t *lock)
    {
    struct clk_apmu *apmu;
    struct clk *clk;
    struct clk_init_data init;
    apmu = kzalloc_obj(*apmu);
    if (!apmu)
    return core::ptr::null_mut();
    init.name = name;
    init.ops = &clk_apmu_ops;
    init.flags = CLK_SET_RATE_PARENT;
    init.parent_names = (parent_name ? &parent_name : core::ptr::null_mut());
    init.num_parents = (parent_name ? 1 : 0);
    apmu.base = base;
    apmu.enable_mask = enable_mask;
    apmu.lock = lock;
    apmu.hw.init = &init;
    clk = clk_register(core::ptr::null_mut(), &apmu.hw);
    if (IS_ERR(clk))
    kfree(apmu);
    return clk;
    }
