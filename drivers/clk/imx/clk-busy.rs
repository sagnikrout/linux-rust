//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-busy.c
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
// Copyright 2012 Freescale Semiconductor, Inc.
// Copyright 2012 Linaro Ltd.
//

#[no_mangle]
unsafe extern "C" fn clk_busy_wait(reg: *mut void __iomem, shift: u8) -> c_int {
    static int clk_busy_wait(void __iomem *reg, u8 shift)
    {
    let mut timeout: c_ulong = jiffies + msecs_to_jiffies(10);
    while (readl_relaxed(reg) & (1 << shift))
    if (time_after(jiffies, timeout))
    return -ETIMEDOUT;
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_busy_divider {
    pub div: clk_divider,
    pub div_ops: *const clk_ops,
    pub reg: *mut void __iomem,
    pub shift: u8,
}

    static inline struct clk_busy_divider *to_clk_busy_divider(struct clk_hw *hw)
    {
    struct clk_divider *div = to_clk_divider(hw);
    return container_of(div, struct clk_busy_divider, div);
    }
    static unsigned long clk_busy_divider_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_busy_divider *busy = to_clk_busy_divider(hw);
    return busy.div_ops.recalc_rate(&busy.div.hw, parent_rate);
    }
    static int clk_busy_divider_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_busy_divider *busy = to_clk_busy_divider(hw);
    return busy.div_ops.determine_rate(&busy.div.hw, req);
    }
    static int clk_busy_divider_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_busy_divider *busy = to_clk_busy_divider(hw);
    int ret;
    ret = busy.div_ops.set_rate(&busy.div.hw, rate, parent_rate);
    if (!ret)
    ret = clk_busy_wait(busy.reg, busy.shift);
    return ret;
    }
    static const struct clk_ops clk_busy_divider_ops = {
    .recalc_rate = clk_busy_divider_recalc_rate,
    .determine_rate = clk_busy_divider_determine_rate,
    .set_rate = clk_busy_divider_set_rate,
    };
    struct clk_hw *imx_clk_hw_busy_divider(const char *name, const char *parent_name,
    void __iomem *reg, u8 shift, u8 width,
    void __iomem *busy_reg, u8 busy_shift)
    {
    struct clk_busy_divider *busy;
    struct clk_hw *hw;
    struct clk_init_data init;
    int ret;
    busy = kzalloc_obj(*busy);
    if (!busy)
    return ERR_PTR(-ENOMEM);
    busy.reg = busy_reg;
    busy.shift = busy_shift;
    busy.div.reg = reg;
    busy.div.shift = shift;
    busy.div.width = width;
    busy.div.lock = &imx_ccm_lock;
    busy.div_ops = &clk_divider_ops;
    init.name = name;
    init.ops = &clk_busy_divider_ops;
    init.flags = CLK_SET_RATE_PARENT | CLK_IS_CRITICAL;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    busy.div.hw.init = &init;
    hw = &busy.div.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if (ret) {
    kfree(busy);
    return ERR_PTR(ret);
    }
    return hw;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_busy_mux {
    pub mux: clk_mux,
    pub mux_ops: *const clk_ops,
    pub reg: *mut void __iomem,
    pub shift: u8,
}

    static inline struct clk_busy_mux *to_clk_busy_mux(struct clk_hw *hw)
    {
    struct clk_mux *mux = to_clk_mux(hw);
    return container_of(mux, struct clk_busy_mux, mux);
    }
#[no_mangle]
unsafe extern "C" fn clk_busy_mux_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 clk_busy_mux_get_parent(struct clk_hw *hw)
    {
    struct clk_busy_mux *busy = to_clk_busy_mux(hw);
    return busy.mux_ops.get_parent(&busy.mux.hw);
    }
#[no_mangle]
unsafe extern "C" fn clk_busy_mux_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int clk_busy_mux_set_parent(struct clk_hw *hw, u8 index)
    {
    struct clk_busy_mux *busy = to_clk_busy_mux(hw);
    int ret;
    ret = busy.mux_ops.set_parent(&busy.mux.hw, index);
    if (!ret)
    ret = clk_busy_wait(busy.reg, busy.shift);
    return ret;
    }
    static const struct clk_ops clk_busy_mux_ops = {
    .determine_rate = clk_hw_determine_rate_no_reparent,
    .get_parent = clk_busy_mux_get_parent,
    .set_parent = clk_busy_mux_set_parent,
    };
    struct clk_hw *imx_clk_hw_busy_mux(const char *name, void __iomem *reg, u8 shift,
    u8 width, void __iomem *busy_reg, u8 busy_shift,
    const char * const *parent_names, int num_parents)
    {
    struct clk_busy_mux *busy;
    struct clk_hw *hw;
    struct clk_init_data init;
    int ret;
    busy = kzalloc_obj(*busy);
    if (!busy)
    return ERR_PTR(-ENOMEM);
    busy.reg = busy_reg;
    busy.shift = busy_shift;
    busy.mux.reg = reg;
    busy.mux.shift = shift;
    busy.mux.mask = BIT(width) - 1;
    busy.mux.lock = &imx_ccm_lock;
    busy.mux_ops = &clk_mux_ops;
    init.name = name;
    init.ops = &clk_busy_mux_ops;
    init.flags = CLK_IS_CRITICAL;
    init.parent_names = parent_names;
    init.num_parents = num_parents;
    busy.mux.hw.init = &init;
    hw = &busy.mux.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if (ret) {
    kfree(busy);
    return ERR_PTR(ret);
    }
    return hw;
    }
