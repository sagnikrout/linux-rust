//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mmp/clk-apbc.c
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
// mmp APB clock operation source file
//
// Copyright (C) 2012 Marvell
// Chao Xie <xiechao.mail@gmail.com>
//

// Common APB clock register bit definitions

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_apbc {
    pub hw: clk_hw,
    pub base: *mut void __iomem,
    pub delay: c_uint,
    pub flags: c_uint,
    pub lock: *mut spinlock_t,
}

#[no_mangle]
unsafe extern "C" fn clk_apbc_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_apbc_prepare(struct clk_hw *hw)
    {
    struct clk_apbc *apbc = to_clk_apbc(hw);
    unsigned int data;
    let mut flags: c_ulong = 0;
//
// It may share same register as MUX clock,
// and it will impact FNCLK enable. Spinlock is needed
//
    if (apbc.lock)
    spin_lock_irqsave(apbc.lock, flags);
    data = readl_relaxed(apbc.base);
    if (apbc.flags & APBC_POWER_CTRL)
    data |= APBC_POWER;
    data |= APBC_FNCLK;
    writel_relaxed(data, apbc.base);
    if (apbc.lock)
    spin_unlock_irqrestore(apbc.lock, flags);
    udelay(apbc.delay);
    if (apbc.lock)
    spin_lock_irqsave(apbc.lock, flags);
    data = readl_relaxed(apbc.base);
    data |= APBC_APBCLK;
    writel_relaxed(data, apbc.base);
    if (apbc.lock)
    spin_unlock_irqrestore(apbc.lock, flags);
    udelay(apbc.delay);
    if (!(apbc.flags & APBC_NO_BUS_CTRL)) {
    if (apbc.lock)
    spin_lock_irqsave(apbc.lock, flags);
    data = readl_relaxed(apbc.base);
    data &= ~APBC_RST;
    writel_relaxed(data, apbc.base);
    if (apbc.lock)
    spin_unlock_irqrestore(apbc.lock, flags);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_apbc_unprepare(hw: *mut clk_hw) {
    static void clk_apbc_unprepare(struct clk_hw *hw)
    {
    struct clk_apbc *apbc = to_clk_apbc(hw);
    unsigned long data;
    let mut flags: c_ulong = 0;
    if (apbc.lock)
    spin_lock_irqsave(apbc.lock, flags);
    data = readl_relaxed(apbc.base);
    if (apbc.flags & APBC_POWER_CTRL)
    data &= ~APBC_POWER;
    data &= ~APBC_FNCLK;
    writel_relaxed(data, apbc.base);
    if (apbc.lock)
    spin_unlock_irqrestore(apbc.lock, flags);
    udelay(10);
    if (apbc.lock)
    spin_lock_irqsave(apbc.lock, flags);
    data = readl_relaxed(apbc.base);
    data &= ~APBC_APBCLK;
    writel_relaxed(data, apbc.base);
    if (apbc.lock)
    spin_unlock_irqrestore(apbc.lock, flags);
    }
    static const struct clk_ops clk_apbc_ops = {
    .prepare = clk_apbc_prepare,
    .unprepare = clk_apbc_unprepare,
    };
    struct clk *mmp_clk_register_apbc(const char *name, const char *parent_name,
    void __iomem *base, unsigned int delay,
    unsigned int apbc_flags, spinlock_t *lock)
    {
    struct clk_apbc *apbc;
    struct clk *clk;
    struct clk_init_data init;
    apbc = kzalloc_obj(*apbc);
    if (!apbc)
    return core::ptr::null_mut();
    init.name = name;
    init.ops = &clk_apbc_ops;
    init.flags = CLK_SET_RATE_PARENT;
    init.parent_names = (parent_name ? &parent_name : core::ptr::null_mut());
    init.num_parents = (parent_name ? 1 : 0);
    apbc.base = base;
    apbc.delay = delay;
    apbc.flags = apbc_flags;
    apbc.lock = lock;
    apbc.hw.init = &init;
    clk = clk_register(core::ptr::null_mut(), &apbc.hw);
    if (IS_ERR(clk))
    kfree(apbc);
    return clk;
    }
