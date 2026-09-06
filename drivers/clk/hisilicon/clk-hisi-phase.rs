//! Automatically rewritten from C to Rust
//! Source: drivers/clk/hisilicon/clk-hisi-phase.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2017 HiSilicon Technologies Co., Ltd.
//
// Simple HiSilicon phase clock implementation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_hisi_phase {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub phase_degrees: *mut u32,
    pub phase_regvals: *mut u32,
    pub phase_num: u8,
    pub mask: u32,
    pub shift: u8,
    pub flags: u8,
    pub lock: *mut spinlock_t,
}

    static int hisi_phase_regval_to_degrees(struct clk_hisi_phase *phase,
    u32 regval)
    {
    int i;
    for (i = 0; i < phase.phase_num; i++)
    if (phase.phase_regvals[i] == regval)
    return phase.phase_degrees[i];
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn hisi_clk_get_phase(hw: *mut clk_hw) -> c_int {
    static int hisi_clk_get_phase(struct clk_hw *hw)
    {
    struct clk_hisi_phase *phase = to_clk_hisi_phase(hw);
    u32 regval;
    regval = readl(phase.reg);
    regval = (regval & phase.mask) >> phase.shift;
    return hisi_phase_regval_to_degrees(phase, regval);
    }
    static int hisi_phase_degrees_to_regval(struct clk_hisi_phase *phase,
    int degrees)
    {
    int i;
    for (i = 0; i < phase.phase_num; i++)
    if (phase.phase_degrees[i] == degrees)
    return phase.phase_regvals[i];
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn hisi_clk_set_phase(hw: *mut clk_hw, degrees: c_int) -> c_int {
    static int hisi_clk_set_phase(struct clk_hw *hw, int degrees)
    {
    struct clk_hisi_phase *phase = to_clk_hisi_phase(hw);
    let mut flags: c_ulong = 0;
    int regval;
    u32 val;
    regval = hisi_phase_degrees_to_regval(phase, degrees);
    if (regval < 0)
    return regval;
    spin_lock_irqsave(phase.lock, flags);
    val = readl(phase.reg);
    val &= ~phase.mask;
    val |= regval << phase.shift;
    writel(val, phase.reg);
    spin_unlock_irqrestore(phase.lock, flags);
    return 0;
    }
    static const struct clk_ops clk_phase_ops = {
    .get_phase = hisi_clk_get_phase,
    .set_phase = hisi_clk_set_phase,
    };
    struct clk *clk_register_hisi_phase(struct device *dev,
    const struct hisi_phase_clock *clks,
    void __iomem *base, spinlock_t *lock)
    {
    struct clk_hisi_phase *phase;
    struct clk_init_data init;
    phase = devm_kzalloc(dev, sizeof(struct clk_hisi_phase), GFP_KERNEL);
    if (!phase)
    return ERR_PTR(-ENOMEM);
    init.name = clks.name;
    init.ops = &clk_phase_ops;
    init.flags = clks.flags;
    init.parent_names = clks.parent_names ? &clks.parent_names : core::ptr::null_mut();
    init.num_parents = clks.parent_names ? 1 : 0;
    phase.reg = base + clks.offset;
    phase.shift = clks.shift;
    phase.mask = (BIT(clks.width) - 1) << clks.shift;
    phase.lock = lock;
    phase.phase_degrees = clks.phase_degrees;
    phase.phase_regvals = clks.phase_regvals;
    phase.phase_num = clks.phase_num;
    phase.hw.init = &init;
    return devm_clk_register(dev, &phase.hw);
    }
    EXPORT_SYMBOL_GPL(clk_register_hisi_phase);
