//! Automatically rewritten from C to Rust
//! Source: drivers/clk/spear/clk-gpt-synth.c
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
// General Purpose Timer Synthesizer clock implementation
//

pub const GPT_MSCALE_MASK: c_uint = 0xFFF;
pub const GPT_NSCALE_SHIFT: c_int = 12;
pub const GPT_NSCALE_MASK: c_uint = 0xF;
//
// DOC: General Purpose Timer Synthesizer clock
//
// Calculates gpt synth clk rate for different values of mscale and nscale
//
// Fout= Fin/((2 ^ (N+1)) * (M+1))
//

    static unsigned long gpt_calc_rate(struct clk_hw *hw, unsigned long prate,
    int index)
    {
    struct clk_gpt *gpt = to_clk_gpt(hw);
    struct gpt_rate_tbl *rtbl = gpt.rtbl;
    prate /= ((1 << (rtbl[index].nscale + 1)) * (rtbl[index].mscale + 1));
    return prate;
    }
    static int clk_gpt_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_gpt *gpt = to_clk_gpt(hw);
    int unused;
    req.rate = clk_round_rate_index(hw, req.rate, req.best_parent_rate,
    gpt_calc_rate, gpt.rtbl_cnt, &unused);
    return 0;
    }
    static unsigned long clk_gpt_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_gpt *gpt = to_clk_gpt(hw);
    let mut flags: c_ulong = 0;
    let mut div: c_uint = 1, val;
    if (gpt.lock)
    spin_lock_irqsave(gpt.lock, flags);
    val = readl_relaxed(gpt.reg);
    if (gpt.lock)
    spin_unlock_irqrestore(gpt.lock, flags);
    div += val & GPT_MSCALE_MASK;
    div *= 1 << (((val >> GPT_NSCALE_SHIFT) & GPT_NSCALE_MASK) + 1);
    if (!div)
    return 0;
    return parent_rate / div;
    }
// Configures new clock rate of gpt
    static int clk_gpt_set_rate(struct clk_hw *hw, unsigned long drate,
    unsigned long prate)
    {
    struct clk_gpt *gpt = to_clk_gpt(hw);
    struct gpt_rate_tbl *rtbl = gpt.rtbl;
    let mut flags: c_ulong = 0, val;
    int i;
    clk_round_rate_index(hw, drate, prate, gpt_calc_rate, gpt.rtbl_cnt,
    &i);
    if (gpt.lock)
    spin_lock_irqsave(gpt.lock, flags);
    val = readl(gpt.reg) & ~GPT_MSCALE_MASK;
    val &= ~(GPT_NSCALE_MASK << GPT_NSCALE_SHIFT);
    val |= rtbl[i].mscale & GPT_MSCALE_MASK;
    val |= (rtbl[i].nscale & GPT_NSCALE_MASK) << GPT_NSCALE_SHIFT;
    writel_relaxed(val, gpt.reg);
    if (gpt.lock)
    spin_unlock_irqrestore(gpt.lock, flags);
    return 0;
    }
    static const struct clk_ops clk_gpt_ops = {
    .recalc_rate = clk_gpt_recalc_rate,
    .determine_rate = clk_gpt_determine_rate,
    .set_rate = clk_gpt_set_rate,
    };
    struct clk *clk_register_gpt(const char *name, const char *parent_name, unsigned
    long flags, void __iomem *reg, struct gpt_rate_tbl *rtbl, u8
    rtbl_cnt, spinlock_t *lock)
    {
    struct clk_init_data init;
    struct clk_gpt *gpt;
    struct clk *clk;
    if (!name || !parent_name || !reg || !rtbl || !rtbl_cnt) {
    pr_err("Invalid arguments passed\n");
    return ERR_PTR(-EINVAL);
    }
    gpt = kzalloc_obj(*gpt);
    if (!gpt)
    return ERR_PTR(-ENOMEM);
// struct clk_gpt assignments
    gpt.reg = reg;
    gpt.rtbl = rtbl;
    gpt.rtbl_cnt = rtbl_cnt;
    gpt.lock = lock;
    gpt.hw.init = &init;
    init.name = name;
    init.ops = &clk_gpt_ops;
    init.flags = flags;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    clk = clk_register(core::ptr::null_mut(), &gpt.hw);
    if (!IS_ERR_OR_NULL(clk))
    return clk;
    pr_err("clk register failed\n");
    kfree(gpt);
    return core::ptr::null_mut();
    }
