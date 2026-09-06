//! Automatically rewritten from C to Rust
//! Source: drivers/clk/spear/clk-frac-synth.c
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
// Copyright (C) 2012 ST Microelectronics
// Viresh Kumar <vireshk@kernel.org>
//
// Fractional Synthesizer clock implementation
//

pub const DIV_FACTOR_MASK: c_uint = 0x1FFFF;
//
// DOC: Fractional Synthesizer clock
//
// Fout from synthesizer can be given from below equation:
//
// Fout= Fin/2*div (division factor)
// div is 17 bits:-
// 0-13 (fractional part)
// 14-16 (integer part)
// div is (16-14 bits).(13-0 bits) (in binary)
//
// Fout = Fin/(2 * div)
// Fout = ((Fin / 10000)/(2 * div)) * 10000
// Fout = (2^14 * (Fin / 10000)/(2^14 * (2 * div))) * 10000
// Fout = (((Fin / 10000) << 14)/(2 * (div << 14))) * 10000
//
// div << 14 simply 17 bit value written at register.
// Max error due to scaling down by 10000 is 10 KHz
//

    static unsigned long frac_calc_rate(struct clk_hw *hw, unsigned long prate,
    int index)
    {
    struct clk_frac *frac = to_clk_frac(hw);
    struct frac_rate_tbl *rtbl = frac.rtbl;
    prate /= 10000;
    prate <<= 14;
    prate /= (2 * rtbl[index].div);
    prate *= 10000;
    return prate;
    }
    static int clk_frac_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_frac *frac = to_clk_frac(hw);
    int unused;
    req.rate = clk_round_rate_index(hw, req.rate, req.best_parent_rate,
    frac_calc_rate, frac.rtbl_cnt, &unused);
    return 0;
    }
    static unsigned long clk_frac_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_frac *frac = to_clk_frac(hw);
    let mut flags: c_ulong = 0;
    let mut div: c_uint = 1, val;
    if (frac.lock)
    spin_lock_irqsave(frac.lock, flags);
    val = readl_relaxed(frac.reg);
    if (frac.lock)
    spin_unlock_irqrestore(frac.lock, flags);
    div = val & DIV_FACTOR_MASK;
    if (!div)
    return 0;
    parent_rate = parent_rate / 10000;
    parent_rate = (parent_rate << 14) / (2 * div);
    return parent_rate * 10000;
    }
// Configures new clock rate of frac
    static int clk_frac_set_rate(struct clk_hw *hw, unsigned long drate,
    unsigned long prate)
    {
    struct clk_frac *frac = to_clk_frac(hw);
    struct frac_rate_tbl *rtbl = frac.rtbl;
    let mut flags: c_ulong = 0, val;
    int i;
    clk_round_rate_index(hw, drate, prate, frac_calc_rate, frac.rtbl_cnt,
    &i);
    if (frac.lock)
    spin_lock_irqsave(frac.lock, flags);
    val = readl_relaxed(frac.reg) & ~DIV_FACTOR_MASK;
    val |= rtbl[i].div & DIV_FACTOR_MASK;
    writel_relaxed(val, frac.reg);
    if (frac.lock)
    spin_unlock_irqrestore(frac.lock, flags);
    return 0;
    }
    static const struct clk_ops clk_frac_ops = {
    .recalc_rate = clk_frac_recalc_rate,
    .determine_rate = clk_frac_determine_rate,
    .set_rate = clk_frac_set_rate,
    };
    struct clk *clk_register_frac(const char *name, const char *parent_name,
    unsigned long flags, void __iomem *reg,
    struct frac_rate_tbl *rtbl, u8 rtbl_cnt, spinlock_t *lock)
    {
    struct clk_init_data init;
    struct clk_frac *frac;
    struct clk *clk;
    if (!name || !parent_name || !reg || !rtbl || !rtbl_cnt) {
    pr_err("Invalid arguments passed\n");
    return ERR_PTR(-EINVAL);
    }
    frac = kzalloc_obj(*frac);
    if (!frac)
    return ERR_PTR(-ENOMEM);
// struct clk_frac assignments
    frac.reg = reg;
    frac.rtbl = rtbl;
    frac.rtbl_cnt = rtbl_cnt;
    frac.lock = lock;
    frac.hw.init = &init;
    init.name = name;
    init.ops = &clk_frac_ops;
    init.flags = flags;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    clk = clk_register(core::ptr::null_mut(), &frac.hw);
    if (!IS_ERR_OR_NULL(clk))
    return clk;
    pr_err("clk register failed\n");
    kfree(frac);
    return core::ptr::null_mut();
    }
