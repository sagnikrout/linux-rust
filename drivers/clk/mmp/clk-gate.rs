//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mmp/clk-gate.c
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
// mmp gate clock operation source file
//
// Copyright (C) 2014 Marvell
// Chao Xie <chao.xie@marvell.com>
//

//
// Some clocks will have multiple bits to enable the clocks, and
// the bits to disable the clock is not same as enabling bits.
//

#[no_mangle]
unsafe extern "C" fn mmp_clk_gate_enable(hw: *mut clk_hw) -> c_int {
    static int mmp_clk_gate_enable(struct clk_hw *hw)
    {
    struct mmp_clk_gate *gate = to_clk_mmp_gate(hw);
    let mut flags: c_ulong = 0;
    unsigned long rate;
    u32 tmp;
    if (gate.lock)
    spin_lock_irqsave(gate.lock, flags);
    tmp = readl(gate.reg);
    tmp &= ~gate.mask;
    tmp |= gate.val_enable;
    writel(tmp, gate.reg);
    if (gate.lock)
    spin_unlock_irqrestore(gate.lock, flags);
    if (gate.flags & MMP_CLK_GATE_NEED_DELAY) {
    rate = clk_hw_get_rate(hw);
// Need delay 2 cycles.
    udelay(2000000/rate);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mmp_clk_gate_disable(hw: *mut clk_hw) {
    static void mmp_clk_gate_disable(struct clk_hw *hw)
    {
    struct mmp_clk_gate *gate = to_clk_mmp_gate(hw);
    let mut flags: c_ulong = 0;
    u32 tmp;
    if (gate.lock)
    spin_lock_irqsave(gate.lock, flags);
    tmp = readl(gate.reg);
    tmp &= ~gate.mask;
    tmp |= gate.val_disable;
    writel(tmp, gate.reg);
    if (gate.lock)
    spin_unlock_irqrestore(gate.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn mmp_clk_gate_is_enabled(hw: *mut clk_hw) -> c_int {
    static int mmp_clk_gate_is_enabled(struct clk_hw *hw)
    {
    struct mmp_clk_gate *gate = to_clk_mmp_gate(hw);
    let mut flags: c_ulong = 0;
    u32 tmp;
    if (gate.lock)
    spin_lock_irqsave(gate.lock, flags);
    tmp = readl(gate.reg);
    if (gate.lock)
    spin_unlock_irqrestore(gate.lock, flags);
    return (tmp & gate.mask) == gate.val_enable;
    }
    const struct clk_ops mmp_clk_gate_ops = {
    .enable = mmp_clk_gate_enable,
    .disable = mmp_clk_gate_disable,
    .is_enabled = mmp_clk_gate_is_enabled,
    };
    struct clk *mmp_clk_register_gate(struct device *dev, const char *name,
    const char *parent_name, unsigned long flags,
    void __iomem *reg, u32 mask, u32 val_enable, u32 val_disable,
    unsigned int gate_flags, spinlock_t *lock)
    {
    struct mmp_clk_gate *gate;
    struct clk *clk;
    struct clk_init_data init;
// allocate the gate
    gate = kzalloc_obj(*gate);
    if (!gate)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &mmp_clk_gate_ops;
    init.flags = flags;
    init.parent_names = (parent_name ? &parent_name : core::ptr::null_mut());
    init.num_parents = (parent_name ? 1 : 0);
// struct clk_gate assignments
    gate.reg = reg;
    gate.mask = mask;
    gate.val_enable = val_enable;
    gate.val_disable = val_disable;
    gate.flags = gate_flags;
    gate.lock = lock;
    gate.hw.init = &init;
    clk = clk_register(dev, &gate.hw);
    if (IS_ERR(clk))
    kfree(gate);
    return clk;
    }
