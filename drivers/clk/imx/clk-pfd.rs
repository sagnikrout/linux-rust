//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-pfd.c
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

//
// struct clk_pfd - IMX PFD clock
// @hw:		clock source
// @reg:	PFD register address
// @idx:	the index of PFD encoded in the register
//
// PFD clock found on i.MX6 series.  Each register for PFD has 4 clk_pfd
// data encoded, and member idx is used to specify the one.  And each
// register has SET, CLR and TOG registers at offset 0x4 0x8 and 0xc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_pfd {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub idx: u8,
}

pub const SET: c_uint = 0x4;
pub const CLR: c_uint = 0x8;
pub const OTG: c_uint = 0xc;
#[no_mangle]
unsafe extern "C" fn clk_pfd_enable(hw: *mut clk_hw) -> c_int {
    static int clk_pfd_enable(struct clk_hw *hw)
    {
    struct clk_pfd *pfd = to_clk_pfd(hw);
    writel_relaxed(1 << ((pfd.idx + 1) * 8 - 1), pfd.reg + CLR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_pfd_disable(hw: *mut clk_hw) {
    static void clk_pfd_disable(struct clk_hw *hw)
    {
    struct clk_pfd *pfd = to_clk_pfd(hw);
    writel_relaxed(1 << ((pfd.idx + 1) * 8 - 1), pfd.reg + SET);
    }
    static unsigned long clk_pfd_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_pfd *pfd = to_clk_pfd(hw);
    let mut tmp: u64 = parent_rate;
    let mut frac: u8 = (readl_relaxed(pfd.reg) >> (pfd.idx * 8)) & 0x3f;
    tmp *= 18;
    do_div(tmp, frac);
    return tmp;
    }
    static int clk_pfd_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    let mut tmp: u64 = req.best_parent_rate;
    u8 frac;
    tmp = tmp * 18 + req.rate / 2;
    do_div(tmp, req.rate);
    frac = tmp;
    if (frac < 12)
    frac = 12;
#[no_mangle]
pub unsafe extern "C" fn if(35: frac >) -> else {
    else if (frac > 35)
    frac = 35;
    tmp = req.best_parent_rate;
    tmp *= 18;
    do_div(tmp, frac);
    req.rate = tmp;
    return 0;
    }
    static int clk_pfd_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_pfd *pfd = to_clk_pfd(hw);
    let mut tmp: u64 = parent_rate;
    u8 frac;
    tmp = tmp * 18 + rate / 2;
    do_div(tmp, rate);
    frac = tmp;
    if (frac < 12)
    frac = 12;
#[no_mangle]
pub unsafe extern "C" fn if(35: frac >) -> else {
    else if (frac > 35)
    frac = 35;
    writel_relaxed(0x3f << (pfd.idx * 8), pfd.reg + CLR);
    writel_relaxed(frac << (pfd.idx * 8), pfd.reg + SET);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_pfd_is_enabled(hw: *mut clk_hw) -> c_int {
    static int clk_pfd_is_enabled(struct clk_hw *hw)
    {
    struct clk_pfd *pfd = to_clk_pfd(hw);
    if (readl_relaxed(pfd.reg) & (1 << ((pfd.idx + 1) * 8 - 1)))
    return 0;
    return 1;
    }
    static const struct clk_ops clk_pfd_ops = {
    .enable		= clk_pfd_enable,
    .disable	= clk_pfd_disable,
    .recalc_rate	= clk_pfd_recalc_rate,
    .determine_rate = clk_pfd_determine_rate,
    .set_rate	= clk_pfd_set_rate,
    .is_enabled     = clk_pfd_is_enabled,
    };
    struct clk_hw *imx_clk_hw_pfd(const char *name, const char *parent_name,
    void __iomem *reg, u8 idx)
    {
    struct clk_pfd *pfd;
    struct clk_hw *hw;
    struct clk_init_data init;
    int ret;
    pfd = kzalloc_obj(*pfd);
    if (!pfd)
    return ERR_PTR(-ENOMEM);
    pfd.reg = reg;
    pfd.idx = idx;
    init.name = name;
    init.ops = &clk_pfd_ops;
    init.flags = 0;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    pfd.hw.init = &init;
    hw = &pfd.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if (ret) {
    kfree(pfd);
    return ERR_PTR(ret);
    }
    return hw;
    }
    EXPORT_SYMBOL_GPL(imx_clk_hw_pfd);
