//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/mcde/mcde_clk_div.c
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

// The MCDE internal clock dividers for FIFO A and B
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcde_clk_div {
    pub hw: clk_hw,
    pub mcde: *mut mcde,
    pub cr: u32,
    pub cr_div: u32,
}

#[no_mangle]
unsafe extern "C" fn mcde_clk_div_enable(hw: *mut clk_hw) -> c_int {
    static int mcde_clk_div_enable(struct clk_hw *hw)
    {
    struct mcde_clk_div *cdiv = container_of(hw, struct mcde_clk_div, hw);
    struct mcde *mcde = cdiv.mcde;
    u32 val;
    spin_lock(&mcde.fifo_crx1_lock);
    val = readl(mcde.regs + cdiv.cr);
//
// Select the PLL72 (LCD) clock as parent
// FIXME: implement other parents.
//
    val &= ~MCDE_CRX1_CLKSEL_MASK;
    val |= MCDE_CRX1_CLKSEL_CLKPLL72 << MCDE_CRX1_CLKSEL_SHIFT;
// Internal clock
    val |= MCDE_CRA1_CLKTYPE_TVXCLKSEL1;
// Clear then set the divider
    val &= ~(MCDE_CRX1_BCD | MCDE_CRX1_PCD_MASK);
    val |= cdiv.cr_div;
    writel(val, mcde.regs + cdiv.cr);
    spin_unlock(&mcde.fifo_crx1_lock);
    return 0;
    }
    static int mcde_clk_div_choose_div(struct clk_hw *hw, unsigned long rate,
    unsigned long *prate, bool set_parent)
    {
    let mut best_div: c_int = 1, div;
    struct clk_hw *parent = clk_hw_get_parent(hw);
    let mut best_prate: c_ulong = 0;
    let mut best_diff: c_ulong = ~0ul;
    let mut max_div: c_int = (1 << MCDE_CRX1_PCD_BITS) - 1;
    for (div = 1; div < max_div; div++) {
    unsigned long this_prate, div_rate, diff;
    if (set_parent)
    this_prate = clk_hw_round_rate(parent, rate * div);
    else
    this_prate = *prate;
    div_rate = DIV_ROUND_UP_ULL(this_prate, div);
    diff = abs(rate - div_rate);
    if (diff < best_diff) {
    best_div = div;
    best_diff = diff;
    best_prate = this_prate;
    }
    }
// prate = best_prate;
    return best_div;
    }
    static int mcde_clk_div_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    int div = mcde_clk_div_choose_div(hw, req.rate,
    &req.best_parent_rate, true);
    req.rate = DIV_ROUND_UP_ULL(req.best_parent_rate, div);
    return 0;
    }
    static unsigned long mcde_clk_div_recalc_rate(struct clk_hw *hw,
    unsigned long prate)
    {
    struct mcde_clk_div *cdiv = container_of(hw, struct mcde_clk_div, hw);
    struct mcde *mcde = cdiv.mcde;
    u32 cr;
    int div;
//
// If the MCDE is not powered we can't access registers.
// It will come up with 0 in the divider register bits, which
// means "divide by 2".
//
    if (!regulator_is_enabled(mcde.epod))
    return DIV_ROUND_UP_ULL(prate, 2);
    cr = readl(mcde.regs + cdiv.cr);
    if (cr & MCDE_CRX1_BCD)
    return prate;
// 0 in the PCD means "divide by 2", 1 means "divide by 3" etc
    div = cr & MCDE_CRX1_PCD_MASK;
    div += 2;
    return DIV_ROUND_UP_ULL(prate, div);
    }
    static int mcde_clk_div_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long prate)
    {
    struct mcde_clk_div *cdiv = container_of(hw, struct mcde_clk_div, hw);
    let mut div: c_int = mcde_clk_div_choose_div(hw, rate, &prate, false);
    let mut cr: u32 = 0;
//
// We cache the CR bits to set the divide in the state so that
// we can call this before we can even write to the hardware.
//
    if (div == 1) {
// Bypass clock divider
    cr |= MCDE_CRX1_BCD;
    } else {
    div -= 2;
    cr |= div & MCDE_CRX1_PCD_MASK;
    }
    cdiv.cr_div = cr;
    return 0;
    }
    static const struct clk_ops mcde_clk_div_ops = {
    .enable = mcde_clk_div_enable,
    .recalc_rate = mcde_clk_div_recalc_rate,
    .determine_rate = mcde_clk_div_determine_rate,
    .set_rate = mcde_clk_div_set_rate,
    };
#[no_mangle]
pub unsafe extern "C" fn mcde_init_clock_divider(mcde: *mut mcde) -> c_int {
    int mcde_init_clock_divider(struct mcde *mcde)
    {
    struct device *dev = mcde.dev;
    struct mcde_clk_div *fifoa;
    struct mcde_clk_div *fifob;
    const char *parent_name;
    struct clk_init_data fifoa_init = {
    .name = "fifoa",
    .ops = &mcde_clk_div_ops,
    .parent_names = &parent_name,
    .num_parents = 1,
    .flags = CLK_SET_RATE_PARENT,
    };
    struct clk_init_data fifob_init = {
    .name = "fifob",
    .ops = &mcde_clk_div_ops,
    .parent_names = &parent_name,
    .num_parents = 1,
    .flags = CLK_SET_RATE_PARENT,
    };
    int ret;
    spin_lock_init(&mcde.fifo_crx1_lock);
    parent_name = __clk_get_name(mcde.lcd_clk);
// Allocate 2 clocks
    fifoa = devm_kzalloc(dev, sizeof(*fifoa), GFP_KERNEL);
    if (!fifoa)
    return -ENOMEM;
    fifob = devm_kzalloc(dev, sizeof(*fifob), GFP_KERNEL);
    if (!fifob)
    return -ENOMEM;
    fifoa.mcde = mcde;
    fifoa.cr = MCDE_CRA1;
    fifoa.hw.init = &fifoa_init;
    ret = devm_clk_hw_register(dev, &fifoa.hw);
    if (ret) {
    dev_err(dev, "error registering FIFO A clock divider\n");
    return ret;
    }
    mcde.fifoa_clk = fifoa.hw.clk;
    fifob.mcde = mcde;
    fifob.cr = MCDE_CRB1;
    fifob.hw.init = &fifob_init;
    ret = devm_clk_hw_register(dev, &fifob.hw);
    if (ret) {
    dev_err(dev, "error registering FIFO B clock divider\n");
    return ret;
    }
    mcde.fifob_clk = fifob.hw.clk;
    return 0;
    }
