//! Automatically rewritten from C to Rust
//! Source: drivers/clk/zynq/pll.c
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
// Zynq PLL driver
//
// Copyright (C) 2013 Xilinx
//
// Sören Brinkmann <soren.brinkmann@xilinx.com>
//

//
// struct zynq_pll - pll clock
// @hw:		Handle between common and hardware-specific interfaces
// @pll_ctrl:	PLL control register
// @pll_status:	PLL status register
// @lock:	Register lock
// @lockbit:	Indicates the associated PLL_LOCKED bit in the PLL status
// register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynq_pll {
    pub hw: clk_hw,
    pub pll_ctrl: *mut void __iomem,
    pub pll_status: *mut void __iomem,
    pub lock: *mut spinlock_t,
    pub lockbit: u8,
}

// Register bitfield defines
pub const PLLCTRL_FBDIV_MASK: c_uint = 0x7f000;
pub const PLLCTRL_FBDIV_SHIFT: c_int = 12;

pub const PLLCTRL_PWRDWN_MASK: c_int = 2;
pub const PLLCTRL_PWRDWN_SHIFT: c_int = 1;
pub const PLLCTRL_RESET_MASK: c_int = 1;
pub const PLLCTRL_RESET_SHIFT: c_int = 0;
pub const PLL_FBDIV_MIN: c_int = 13;
pub const PLL_FBDIV_MAX: c_int = 66;
//
// zynq_pll_determine_rate() - Round a clock frequency
// @hw:		Handle between common and hardware-specific interfaces
// @req:	Clock rate request, updated with the frequency closest to the
// requested one that the hardware can generate
// Return:	0 always
//
    static int zynq_pll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    u32 fbdiv;
    fbdiv = DIV_ROUND_CLOSEST(req.rate, req.best_parent_rate);
    if (fbdiv < PLL_FBDIV_MIN)
    fbdiv = PLL_FBDIV_MIN;
#[no_mangle]
pub unsafe extern "C" fn if(PLL_FBDIV_MAX: fbdiv >) -> else {
    else if (fbdiv > PLL_FBDIV_MAX)
    fbdiv = PLL_FBDIV_MAX;
    req.rate = req.best_parent_rate * fbdiv;
    return 0;
    }
//
// zynq_pll_recalc_rate() - Recalculate clock frequency
// @hw:			Handle between common and hardware-specific interfaces
// @parent_rate:	Clock frequency of parent clock
// Return:		current clock frequency.
//
    static unsigned long zynq_pll_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct zynq_pll *clk = to_zynq_pll(hw);
    u32 fbdiv;
//
// makes probably sense to redundantly save fbdiv in the struct
// zynq_pll to save the IO access.
//
    fbdiv = (readl(clk.pll_ctrl) & PLLCTRL_FBDIV_MASK) >>
    PLLCTRL_FBDIV_SHIFT;
    return parent_rate * fbdiv;
    }
//
// zynq_pll_is_enabled - Check if a clock is enabled
// @hw:		Handle between common and hardware-specific interfaces
// Return:	1 if the clock is enabled, 0 otherwise.
//
// Not sure this is a good idea, but since disabled means bypassed for
// this clock implementation we say we are always enabled.
//
#[no_mangle]
unsafe extern "C" fn zynq_pll_is_enabled(hw: *mut clk_hw) -> c_int {
    static int zynq_pll_is_enabled(struct clk_hw *hw)
    {
    let mut flags: c_ulong = 0;
    u32 reg;
    struct zynq_pll *clk = to_zynq_pll(hw);
    spin_lock_irqsave(clk.lock, flags);
    reg = readl(clk.pll_ctrl);
    spin_unlock_irqrestore(clk.lock, flags);
    return !(reg & (PLLCTRL_RESET_MASK | PLLCTRL_PWRDWN_MASK));
    }
//
// zynq_pll_enable - Enable clock
// @hw:		Handle between common and hardware-specific interfaces
// Return: 0 on success
//
#[no_mangle]
unsafe extern "C" fn zynq_pll_enable(hw: *mut clk_hw) -> c_int {
    static int zynq_pll_enable(struct clk_hw *hw)
    {
    let mut flags: c_ulong = 0;
    u32 reg;
    struct zynq_pll *clk = to_zynq_pll(hw);
    if (zynq_pll_is_enabled(hw))
    return 0;
    pr_info("PLL: enable\n");
// Power up PLL and wait for lock
    spin_lock_irqsave(clk.lock, flags);
    reg = readl(clk.pll_ctrl);
    reg &= ~(PLLCTRL_RESET_MASK | PLLCTRL_PWRDWN_MASK);
    writel(reg, clk.pll_ctrl);
    while (!(readl(clk.pll_status) & (1 << clk.lockbit)))
    ;
    spin_unlock_irqrestore(clk.lock, flags);
    return 0;
    }
//
// zynq_pll_disable - Disable clock
// @hw:		Handle between common and hardware-specific interfaces
// Returns 0 on success
//
#[no_mangle]
unsafe extern "C" fn zynq_pll_disable(hw: *mut clk_hw) {
    static void zynq_pll_disable(struct clk_hw *hw)
    {
    let mut flags: c_ulong = 0;
    u32 reg;
    struct zynq_pll *clk = to_zynq_pll(hw);
    if (!zynq_pll_is_enabled(hw))
    return;
    pr_info("PLL: shutdown\n");
// shut down PLL
    spin_lock_irqsave(clk.lock, flags);
    reg = readl(clk.pll_ctrl);
    reg |= PLLCTRL_RESET_MASK | PLLCTRL_PWRDWN_MASK;
    writel(reg, clk.pll_ctrl);
    spin_unlock_irqrestore(clk.lock, flags);
    }
    static const struct clk_ops zynq_pll_ops = {
    .enable = zynq_pll_enable,
    .disable = zynq_pll_disable,
    .is_enabled = zynq_pll_is_enabled,
    .determine_rate = zynq_pll_determine_rate,
    .recalc_rate = zynq_pll_recalc_rate
    };
//
// clk_register_zynq_pll() - Register PLL with the clock framework
// @name:	PLL name
// @parent:	Parent clock name
// @pll_ctrl:	Pointer to PLL control register
// @pll_status:	Pointer to PLL status register
// @lock_index:	Bit index to this PLL's lock status bit in @pll_status
// @lock:	Register lock
// Return:	handle to the registered clock.
//
    struct clk *clk_register_zynq_pll(const char *name, const char *parent,
    void __iomem *pll_ctrl, void __iomem *pll_status, u8 lock_index,
    spinlock_t *lock)
    {
    struct zynq_pll *pll;
    struct clk *clk;
    u32 reg;
    const char *parent_arr[1] = {parent};
    let mut flags: c_ulong = 0;
    struct clk_init_data initd = {
    .name = name,
    .parent_names = parent_arr,
    .ops = &zynq_pll_ops,
    .num_parents = 1,
    .flags = 0
    };
    pll = kmalloc_obj(*pll);
    if (!pll)
    return ERR_PTR(-ENOMEM);
// Populate the struct
    pll.hw.init = &initd;
    pll.pll_ctrl = pll_ctrl;
    pll.pll_status = pll_status;
    pll.lockbit = lock_index;
    pll.lock = lock;
    spin_lock_irqsave(pll.lock, flags);
    reg = readl(pll.pll_ctrl);
    reg &= ~PLLCTRL_BPQUAL_MASK;
    writel(reg, pll.pll_ctrl);
    spin_unlock_irqrestore(pll.lock, flags);
    clk = clk_register(core::ptr::null_mut(), &pll.hw);
    if (WARN_ON(IS_ERR(clk)))
    goto free_pll;
    return clk;
    free_pll:
    kfree(pll);
    return clk;
    }
