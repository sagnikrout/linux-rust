//! Automatically rewritten from C to Rust
//! Source: drivers/clk/rockchip/clk-ddr.c
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
// Copyright (c) 2016 Rockchip Electronics Co. Ltd.
// Author: Lin Huang <hl@rock-chips.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_ddrclk {
    pub hw: clk_hw,
    pub reg_base: *mut void __iomem,
    pub mux_offset: c_int,
    pub mux_shift: c_int,
    pub mux_width: c_int,
    pub div_shift: c_int,
    pub div_width: c_int,
    pub ddr_flag: c_int,
    pub lock: *mut spinlock_t,
}

    static int rockchip_ddrclk_sip_set_rate(struct clk_hw *hw, unsigned long drate,
    unsigned long prate)
    {
    struct rockchip_ddrclk *ddrclk = to_rockchip_ddrclk_hw(hw);
    unsigned long flags;
    struct arm_smccc_res res;
    spin_lock_irqsave(ddrclk.lock, flags);
    arm_smccc_smc(ROCKCHIP_SIP_DRAM_FREQ, drate, 0,
    ROCKCHIP_SIP_CONFIG_DRAM_SET_RATE,
    0, 0, 0, 0, &res);
    spin_unlock_irqrestore(ddrclk.lock, flags);
    return res.a0;
    }
    static unsigned long
    rockchip_ddrclk_sip_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct arm_smccc_res res;
    arm_smccc_smc(ROCKCHIP_SIP_DRAM_FREQ, 0, 0,
    ROCKCHIP_SIP_CONFIG_DRAM_GET_RATE,
    0, 0, 0, 0, &res);
    return res.a0;
    }
    static int rockchip_ddrclk_sip_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct arm_smccc_res res;
    arm_smccc_smc(ROCKCHIP_SIP_DRAM_FREQ, req.rate, 0,
    ROCKCHIP_SIP_CONFIG_DRAM_ROUND_RATE,
    0, 0, 0, 0, &res);
    req.rate = res.a0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_ddrclk_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 rockchip_ddrclk_get_parent(struct clk_hw *hw)
    {
    struct rockchip_ddrclk *ddrclk = to_rockchip_ddrclk_hw(hw);
    u32 val;
    val = readl(ddrclk.reg_base +
    ddrclk.mux_offset) >> ddrclk.mux_shift;
    val &= GENMASK(ddrclk.mux_width - 1, 0);
    return val;
    }
    static const struct clk_ops rockchip_ddrclk_sip_ops = {
    .recalc_rate = rockchip_ddrclk_sip_recalc_rate,
    .set_rate = rockchip_ddrclk_sip_set_rate,
    .determine_rate = rockchip_ddrclk_sip_determine_rate,
    .get_parent = rockchip_ddrclk_get_parent,
    };
    struct clk *rockchip_clk_register_ddrclk(const char *name, int flags,
    const char *const *parent_names,
    u8 num_parents, int mux_offset,
    int mux_shift, int mux_width,
    int div_shift, int div_width,
    int ddr_flag, void __iomem *reg_base,
    spinlock_t *lock)
    {
    struct rockchip_ddrclk *ddrclk;
    struct clk_init_data init;
    struct clk *clk;
    ddrclk = kzalloc_obj(*ddrclk);
    if (!ddrclk)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.parent_names = parent_names;
    init.num_parents = num_parents;
    init.flags = flags;
    init.flags |= CLK_SET_RATE_NO_REPARENT;
    switch (ddr_flag) {
    case ROCKCHIP_DDRCLK_SIP:
    init.ops = &rockchip_ddrclk_sip_ops;
    break;
    default:
    pr_err("%s: unsupported ddrclk type %d\n", __func__, ddr_flag);
    kfree(ddrclk);
    return ERR_PTR(-EINVAL);
    }
    ddrclk.reg_base = reg_base;
    ddrclk.lock = lock;
    ddrclk.hw.init = &init;
    ddrclk.mux_offset = mux_offset;
    ddrclk.mux_shift = mux_shift;
    ddrclk.mux_width = mux_width;
    ddrclk.div_shift = div_shift;
    ddrclk.div_width = div_width;
    ddrclk.ddr_flag = ddr_flag;
    clk = clk_register(core::ptr::null_mut(), &ddrclk.hw);
    if (IS_ERR(clk))
    kfree(ddrclk);
    return clk;
    }
    EXPORT_SYMBOL_GPL(rockchip_clk_register_ddrclk);
