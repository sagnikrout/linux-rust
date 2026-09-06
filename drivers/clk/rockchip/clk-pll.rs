//! Automatically rewritten from C to Rust
//! Source: drivers/clk/rockchip/clk-pll.c
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
// Copyright (c) 2014 MundoReader S.L.
// Author: Heiko Stuebner <heiko@sntech.de>
//
// Copyright (c) 2015 Rockchip Electronics Co. Ltd.
// Author: Xing Zheng <zhengxing@rock-chips.com>
//

pub const PLL_MODE_MASK: c_uint = 0x3;
pub const PLL_MODE_SLOW: c_uint = 0x0;
pub const PLL_MODE_NORM: c_uint = 0x1;
pub const PLL_MODE_DEEP: c_uint = 0x2;
pub const PLL_RK3328_MODE_MASK: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_clk_pll {
    pub hw: clk_hw,
    pub pll_mux: clk_mux,
    pub pll_mux_ops: *const clk_ops,
    pub clk_nb: notifier_block,
    pub reg_base: *mut void __iomem,
    pub lock_offset: c_int,
    pub lock_shift: c_uint,
    pub type: enum rockchip_pll_type,
    pub flags: u8,
    pub rate_table: *const rockchip_pll_rate_table,
    pub rate_count: c_uint,
    pub lock: *mut spinlock_t,
    pub ctx: *mut rockchip_clk_provider,
}

    container_of(nb, struct rockchip_clk_pll, clk_nb)
    static const struct rockchip_pll_rate_table *rockchip_get_pll_settings(
    struct rockchip_clk_pll *pll, unsigned long rate)
    {
    const struct rockchip_pll_rate_table  *rate_table = pll.rate_table;
    int i;
    for (i = 0; i < pll.rate_count; i++) {
    if (rate == rate_table[i].rate)
    return &rate_table[i];
    }
    return core::ptr::null_mut();
    }
    static int rockchip_pll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    const struct rockchip_pll_rate_table *rate_table = pll.rate_table;
    int i;
// Assuming rate_table is in descending order
    for (i = 0; i < pll.rate_count; i++) {
    if (req.rate >= rate_table[i].rate) {
    req.rate = rate_table[i].rate;
    return 0;
    }
    }
// return minimum supported value
    req.rate = rate_table[i - 1].rate;
    return 0;
    }
//
// Wait for the pll to reach the locked state.
// The calling set_rate function is responsible for making sure the
// grf regmap is available.
//
#[no_mangle]
unsafe extern "C" fn rockchip_pll_wait_lock(pll: *mut rockchip_clk_pll) -> c_int {
    static int rockchip_pll_wait_lock(struct rockchip_clk_pll *pll)
    {
    struct regmap *grf = pll.ctx.grf;
    unsigned int val;
    int ret;
    ret = regmap_read_poll_timeout(grf, pll.lock_offset, val,
    val & BIT(pll.lock_shift), 0, 1000);
    if (ret)
    pr_err("%s: timeout waiting for pll to lock\n", __func__);
    return ret;
    }
//
// PLL used in RK3036
//

pub const RK3036_PLLCON0_FBDIV_MASK: c_uint = 0xfff;
pub const RK3036_PLLCON0_FBDIV_SHIFT: c_int = 0;
pub const RK3036_PLLCON0_POSTDIV1_MASK: c_uint = 0x7;
pub const RK3036_PLLCON0_POSTDIV1_SHIFT: c_int = 12;
pub const RK3036_PLLCON1_REFDIV_MASK: c_uint = 0x3f;
pub const RK3036_PLLCON1_REFDIV_SHIFT: c_int = 0;
pub const RK3036_PLLCON1_POSTDIV2_MASK: c_uint = 0x7;
pub const RK3036_PLLCON1_POSTDIV2_SHIFT: c_int = 6;

pub const RK3036_PLLCON1_DSMPD_MASK: c_uint = 0x1;
pub const RK3036_PLLCON1_DSMPD_SHIFT: c_int = 12;

pub const RK3036_PLLCON2_FRAC_MASK: c_uint = 0xffffff;
pub const RK3036_PLLCON2_FRAC_SHIFT: c_int = 0;
#[no_mangle]
unsafe extern "C" fn rockchip_rk3036_pll_wait_lock(pll: *mut rockchip_clk_pll) -> c_int {
    static int rockchip_rk3036_pll_wait_lock(struct rockchip_clk_pll *pll)
    {
    u32 pllcon;
    int ret;
//
// Lock time typical 250, max 500 input clock cycles @24MHz
// So define a very safe maximum of 1000us, meaning 24000 cycles.
//
    ret = readl_relaxed_poll_timeout(pll.reg_base + RK3036_PLLCON(1),
    pllcon,
    pllcon & RK3036_PLLCON1_LOCK_STATUS,
    0, 1000);
    if (ret)
    pr_err("%s: timeout waiting for pll to lock\n", __func__);
    return ret;
    }
    static void rockchip_rk3036_pll_get_params(struct rockchip_clk_pll *pll,
    struct rockchip_pll_rate_table *rate)
    {
    u32 pllcon;
    pllcon = readl_relaxed(pll.reg_base + RK3036_PLLCON(0));
    rate.fbdiv = ((pllcon >> RK3036_PLLCON0_FBDIV_SHIFT)
    & RK3036_PLLCON0_FBDIV_MASK);
    rate.postdiv1 = ((pllcon >> RK3036_PLLCON0_POSTDIV1_SHIFT)
    & RK3036_PLLCON0_POSTDIV1_MASK);
    pllcon = readl_relaxed(pll.reg_base + RK3036_PLLCON(1));
    rate.refdiv = ((pllcon >> RK3036_PLLCON1_REFDIV_SHIFT)
    & RK3036_PLLCON1_REFDIV_MASK);
    rate.postdiv2 = ((pllcon >> RK3036_PLLCON1_POSTDIV2_SHIFT)
    & RK3036_PLLCON1_POSTDIV2_MASK);
    rate.dsmpd = ((pllcon >> RK3036_PLLCON1_DSMPD_SHIFT)
    & RK3036_PLLCON1_DSMPD_MASK);
    pllcon = readl_relaxed(pll.reg_base + RK3036_PLLCON(2));
    rate.frac = ((pllcon >> RK3036_PLLCON2_FRAC_SHIFT)
    & RK3036_PLLCON2_FRAC_MASK);
    }
    static unsigned long rockchip_rk3036_pll_recalc_rate(struct clk_hw *hw,
    unsigned long prate)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    struct rockchip_pll_rate_table cur;
    let mut rate64: u64 = prate;
    rockchip_rk3036_pll_get_params(pll, &cur);
    rate64 *= cur.fbdiv;
    do_div(rate64, cur.refdiv);
    if (cur.dsmpd == 0) {
// fractional mode
    let mut frac_rate64: u64 = prate * cur.frac;
    do_div(frac_rate64, cur.refdiv);
    rate64 += frac_rate64 >> 24;
    }
    do_div(rate64, cur.postdiv1);
    do_div(rate64, cur.postdiv2);
    return (unsigned long)rate64;
    }
    static int rockchip_rk3036_pll_set_params(struct rockchip_clk_pll *pll,
    const struct rockchip_pll_rate_table *rate)
    {
    const struct clk_ops *pll_mux_ops = pll.pll_mux_ops;
    struct clk_mux *pll_mux = &pll.pll_mux;
    struct rockchip_pll_rate_table cur;
    u32 pllcon;
    let mut rate_change_remuxed: c_int = 0;
    int cur_parent;
    int ret;
    pr_debug("%s: rate settings for %lu fbdiv: %d, postdiv1: %d, refdiv: %d, postdiv2: %d, dsmpd: %d, frac: %d\n",
    __func__, rate.rate, rate.fbdiv, rate.postdiv1, rate.refdiv,
    rate.postdiv2, rate.dsmpd, rate.frac);
    rockchip_rk3036_pll_get_params(pll, &cur);
    cur.rate = 0;
    if (!(pll.flags & ROCKCHIP_PLL_FIXED_MODE)) {
    cur_parent = pll_mux_ops.get_parent(&pll_mux.hw);
    if (cur_parent == PLL_MODE_NORM) {
    pll_mux_ops.set_parent(&pll_mux.hw, PLL_MODE_SLOW);
    rate_change_remuxed = 1;
    }
    }
// update pll values
    writel_relaxed(HIWORD_UPDATE(rate.fbdiv, RK3036_PLLCON0_FBDIV_MASK,
    RK3036_PLLCON0_FBDIV_SHIFT) |
    HIWORD_UPDATE(rate.postdiv1, RK3036_PLLCON0_POSTDIV1_MASK,
    RK3036_PLLCON0_POSTDIV1_SHIFT),
    pll.reg_base + RK3036_PLLCON(0));
    writel_relaxed(HIWORD_UPDATE(rate.refdiv, RK3036_PLLCON1_REFDIV_MASK,
    RK3036_PLLCON1_REFDIV_SHIFT) |
    HIWORD_UPDATE(rate.postdiv2, RK3036_PLLCON1_POSTDIV2_MASK,
    RK3036_PLLCON1_POSTDIV2_SHIFT) |
    HIWORD_UPDATE(rate.dsmpd, RK3036_PLLCON1_DSMPD_MASK,
    RK3036_PLLCON1_DSMPD_SHIFT),
    pll.reg_base + RK3036_PLLCON(1));
// GPLL CON2 is not HIWORD_MASK
    pllcon = readl_relaxed(pll.reg_base + RK3036_PLLCON(2));
    pllcon &= ~(RK3036_PLLCON2_FRAC_MASK << RK3036_PLLCON2_FRAC_SHIFT);
    pllcon |= rate.frac << RK3036_PLLCON2_FRAC_SHIFT;
    writel_relaxed(pllcon, pll.reg_base + RK3036_PLLCON(2));
// wait for the pll to lock
    ret = rockchip_rk3036_pll_wait_lock(pll);
    if (ret) {
    pr_warn("%s: pll update unsuccessful, trying to restore old params\n",
    __func__);
    rockchip_rk3036_pll_set_params(pll, &cur);
    }
    if (rate_change_remuxed)
    pll_mux_ops.set_parent(&pll_mux.hw, PLL_MODE_NORM);
    return ret;
    }
    static int rockchip_rk3036_pll_set_rate(struct clk_hw *hw, unsigned long drate,
    unsigned long prate)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    const struct rockchip_pll_rate_table *rate;
    pr_debug("%s: changing %s to %lu with a parent rate of %lu\n",
    __func__, __clk_get_name(hw.clk), drate, prate);
// Get required rate settings from table
    rate = rockchip_get_pll_settings(pll, drate);
    if (!rate) {
    pr_err("%s: Invalid rate : %lu for pll clk %s\n", __func__,
    drate, __clk_get_name(hw.clk));
    return -EINVAL;
    }
    return rockchip_rk3036_pll_set_params(pll, rate);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_rk3036_pll_enable(hw: *mut clk_hw) -> c_int {
    static int rockchip_rk3036_pll_enable(struct clk_hw *hw)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    writel(HIWORD_UPDATE(0, RK3036_PLLCON1_PWRDOWN, 0),
    pll.reg_base + RK3036_PLLCON(1));
    rockchip_rk3036_pll_wait_lock(pll);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_rk3036_pll_disable(hw: *mut clk_hw) {
    static void rockchip_rk3036_pll_disable(struct clk_hw *hw)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    writel(HIWORD_UPDATE(RK3036_PLLCON1_PWRDOWN,
    RK3036_PLLCON1_PWRDOWN, 0),
    pll.reg_base + RK3036_PLLCON(1));
    }
#[no_mangle]
unsafe extern "C" fn rockchip_rk3036_pll_is_enabled(hw: *mut clk_hw) -> c_int {
    static int rockchip_rk3036_pll_is_enabled(struct clk_hw *hw)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    let mut pllcon: u32 = readl(pll.reg_base + RK3036_PLLCON(1));
    return !(pllcon & RK3036_PLLCON1_PWRDOWN);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_rk3036_pll_init(hw: *mut clk_hw) -> c_int {
    static int rockchip_rk3036_pll_init(struct clk_hw *hw)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    const struct rockchip_pll_rate_table *rate;
    struct rockchip_pll_rate_table cur;
    unsigned long drate;
    if (!(pll.flags & ROCKCHIP_PLL_SYNC_RATE))
    return 0;
    drate = clk_hw_get_rate(hw);
    rate = rockchip_get_pll_settings(pll, drate);
// when no rate setting for the current rate, rely on clk_set_rate
    if (!rate)
    return 0;
    rockchip_rk3036_pll_get_params(pll, &cur);
    pr_debug("%s: pll %s@%lu: Hz\n", __func__, __clk_get_name(hw.clk),
    drate);
    pr_debug("old - fbdiv: %d, postdiv1: %d, refdiv: %d, postdiv2: %d, dsmpd: %d, frac: %d\n",
    cur.fbdiv, cur.postdiv1, cur.refdiv, cur.postdiv2,
    cur.dsmpd, cur.frac);
    pr_debug("new - fbdiv: %d, postdiv1: %d, refdiv: %d, postdiv2: %d, dsmpd: %d, frac: %d\n",
    rate.fbdiv, rate.postdiv1, rate.refdiv, rate.postdiv2,
    rate.dsmpd, rate.frac);
    if (rate.fbdiv != cur.fbdiv || rate.postdiv1 != cur.postdiv1 ||
    rate.refdiv != cur.refdiv || rate.postdiv2 != cur.postdiv2 ||
    rate.dsmpd != cur.dsmpd ||
    (!cur.dsmpd && (rate.frac != cur.frac))) {
    struct clk *parent = clk_get_parent(hw.clk);
    if (!parent) {
    pr_warn("%s: parent of %s not available\n",
    __func__, __clk_get_name(hw.clk));
    return 0;
    }
    pr_debug("%s: pll %s: rate params do not match rate table, adjusting\n",
    __func__, __clk_get_name(hw.clk));
    rockchip_rk3036_pll_set_params(pll, rate);
    }
    return 0;
    }
    static const struct clk_ops rockchip_rk3036_pll_clk_norate_ops = {
    .recalc_rate = rockchip_rk3036_pll_recalc_rate,
    .enable = rockchip_rk3036_pll_enable,
    .disable = rockchip_rk3036_pll_disable,
    .is_enabled = rockchip_rk3036_pll_is_enabled,
    };
    static const struct clk_ops rockchip_rk3036_pll_clk_ops = {
    .recalc_rate = rockchip_rk3036_pll_recalc_rate,
    .determine_rate = rockchip_pll_determine_rate,
    .set_rate = rockchip_rk3036_pll_set_rate,
    .enable = rockchip_rk3036_pll_enable,
    .disable = rockchip_rk3036_pll_disable,
    .is_enabled = rockchip_rk3036_pll_is_enabled,
    .init = rockchip_rk3036_pll_init,
    };
//
// PLL used in RK3066, RK3188 and RK3288
//

pub const RK3066_PLLCON0_OD_MASK: c_uint = 0xf;
pub const RK3066_PLLCON0_OD_SHIFT: c_int = 0;
pub const RK3066_PLLCON0_NR_MASK: c_uint = 0x3f;
pub const RK3066_PLLCON0_NR_SHIFT: c_int = 8;
pub const RK3066_PLLCON1_NF_MASK: c_uint = 0x1fff;
pub const RK3066_PLLCON1_NF_SHIFT: c_int = 0;
pub const RK3066_PLLCON2_NB_MASK: c_uint = 0xfff;
pub const RK3066_PLLCON2_NB_SHIFT: c_int = 0;

    static void rockchip_rk3066_pll_get_params(struct rockchip_clk_pll *pll,
    struct rockchip_pll_rate_table *rate)
    {
    u32 pllcon;
    pllcon = readl_relaxed(pll.reg_base + RK3066_PLLCON(0));
    rate.nr = ((pllcon >> RK3066_PLLCON0_NR_SHIFT)
    & RK3066_PLLCON0_NR_MASK) + 1;
    rate.no = ((pllcon >> RK3066_PLLCON0_OD_SHIFT)
    & RK3066_PLLCON0_OD_MASK) + 1;
    pllcon = readl_relaxed(pll.reg_base + RK3066_PLLCON(1));
    rate.nf = ((pllcon >> RK3066_PLLCON1_NF_SHIFT)
    & RK3066_PLLCON1_NF_MASK) + 1;
    pllcon = readl_relaxed(pll.reg_base + RK3066_PLLCON(2));
    rate.nb = ((pllcon >> RK3066_PLLCON2_NB_SHIFT)
    & RK3066_PLLCON2_NB_MASK) + 1;
    }
    static unsigned long rockchip_rk3066_pll_recalc_rate(struct clk_hw *hw,
    unsigned long prate)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    struct rockchip_pll_rate_table cur;
    let mut rate64: u64 = prate;
    u32 pllcon;
    pllcon = readl_relaxed(pll.reg_base + RK3066_PLLCON(3));
    if (pllcon & RK3066_PLLCON3_BYPASS) {
    pr_debug("%s: pll %s is bypassed\n", __func__,
    clk_hw_get_name(hw));
    return prate;
    }
    rockchip_rk3066_pll_get_params(pll, &cur);
    rate64 *= cur.nf;
    do_div(rate64, cur.nr);
    do_div(rate64, cur.no);
    return (unsigned long)rate64;
    }
    static int rockchip_rk3066_pll_set_params(struct rockchip_clk_pll *pll,
    const struct rockchip_pll_rate_table *rate)
    {
    const struct clk_ops *pll_mux_ops = pll.pll_mux_ops;
    struct clk_mux *pll_mux = &pll.pll_mux;
    struct rockchip_pll_rate_table cur;
    let mut rate_change_remuxed: c_int = 0;
    int cur_parent;
    int ret;
    pr_debug("%s: rate settings for %lu (nr, no, nf): (%d, %d, %d)\n",
    __func__, rate.rate, rate.nr, rate.no, rate.nf);
    rockchip_rk3066_pll_get_params(pll, &cur);
    cur.rate = 0;
    cur_parent = pll_mux_ops.get_parent(&pll_mux.hw);
    if (cur_parent == PLL_MODE_NORM) {
    pll_mux_ops.set_parent(&pll_mux.hw, PLL_MODE_SLOW);
    rate_change_remuxed = 1;
    }
// enter reset mode
    writel(HIWORD_UPDATE(RK3066_PLLCON3_RESET, RK3066_PLLCON3_RESET, 0),
    pll.reg_base + RK3066_PLLCON(3));
// update pll values
    writel(HIWORD_UPDATE(rate.nr - 1, RK3066_PLLCON0_NR_MASK,
    RK3066_PLLCON0_NR_SHIFT) |
    HIWORD_UPDATE(rate.no - 1, RK3066_PLLCON0_OD_MASK,
    RK3066_PLLCON0_OD_SHIFT),
    pll.reg_base + RK3066_PLLCON(0));
    writel_relaxed(HIWORD_UPDATE(rate.nf - 1, RK3066_PLLCON1_NF_MASK,
    RK3066_PLLCON1_NF_SHIFT),
    pll.reg_base + RK3066_PLLCON(1));
    writel_relaxed(HIWORD_UPDATE(rate.nb - 1, RK3066_PLLCON2_NB_MASK,
    RK3066_PLLCON2_NB_SHIFT),
    pll.reg_base + RK3066_PLLCON(2));
// leave reset and wait the reset_delay
    writel(HIWORD_UPDATE(0, RK3066_PLLCON3_RESET, 0),
    pll.reg_base + RK3066_PLLCON(3));
    udelay(RK3066_PLL_RESET_DELAY(rate.nr));
// wait for the pll to lock
    ret = rockchip_pll_wait_lock(pll);
    if (ret) {
    pr_warn("%s: pll update unsuccessful, trying to restore old params\n",
    __func__);
    rockchip_rk3066_pll_set_params(pll, &cur);
    }
    if (rate_change_remuxed)
    pll_mux_ops.set_parent(&pll_mux.hw, PLL_MODE_NORM);
    return ret;
    }
    static int rockchip_rk3066_pll_set_rate(struct clk_hw *hw, unsigned long drate,
    unsigned long prate)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    const struct rockchip_pll_rate_table *rate;
    pr_debug("%s: changing %s to %lu with a parent rate of %lu\n",
    __func__, clk_hw_get_name(hw), drate, prate);
// Get required rate settings from table
    rate = rockchip_get_pll_settings(pll, drate);
    if (!rate) {
    pr_err("%s: Invalid rate : %lu for pll clk %s\n", __func__,
    drate, clk_hw_get_name(hw));
    return -EINVAL;
    }
    return rockchip_rk3066_pll_set_params(pll, rate);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_rk3066_pll_enable(hw: *mut clk_hw) -> c_int {
    static int rockchip_rk3066_pll_enable(struct clk_hw *hw)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    writel(HIWORD_UPDATE(0, RK3066_PLLCON3_PWRDOWN, 0),
    pll.reg_base + RK3066_PLLCON(3));
    rockchip_pll_wait_lock(pll);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_rk3066_pll_disable(hw: *mut clk_hw) {
    static void rockchip_rk3066_pll_disable(struct clk_hw *hw)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    writel(HIWORD_UPDATE(RK3066_PLLCON3_PWRDOWN,
    RK3066_PLLCON3_PWRDOWN, 0),
    pll.reg_base + RK3066_PLLCON(3));
    }
#[no_mangle]
unsafe extern "C" fn rockchip_rk3066_pll_is_enabled(hw: *mut clk_hw) -> c_int {
    static int rockchip_rk3066_pll_is_enabled(struct clk_hw *hw)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    let mut pllcon: u32 = readl(pll.reg_base + RK3066_PLLCON(3));
    return !(pllcon & RK3066_PLLCON3_PWRDOWN);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_rk3066_pll_init(hw: *mut clk_hw) -> c_int {
    static int rockchip_rk3066_pll_init(struct clk_hw *hw)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    const struct rockchip_pll_rate_table *rate;
    struct rockchip_pll_rate_table cur;
    unsigned long drate;
    if (!(pll.flags & ROCKCHIP_PLL_SYNC_RATE))
    return 0;
    drate = clk_hw_get_rate(hw);
    rate = rockchip_get_pll_settings(pll, drate);
// when no rate setting for the current rate, rely on clk_set_rate
    if (!rate)
    return 0;
    rockchip_rk3066_pll_get_params(pll, &cur);
    pr_debug("%s: pll %s@%lu: nr (%d:%d); no (%d:%d); nf(%d:%d), nb(%d:%d)\n",
    __func__, clk_hw_get_name(hw), drate, rate.nr, cur.nr,
    rate.no, cur.no, rate.nf, cur.nf, rate.nb, cur.nb);
    if (rate.nr != cur.nr || rate.no != cur.no || rate.nf != cur.nf
    || rate.nb != cur.nb) {
    pr_debug("%s: pll %s: rate params do not match rate table, adjusting\n",
    __func__, clk_hw_get_name(hw));
    rockchip_rk3066_pll_set_params(pll, rate);
    }
    return 0;
    }
    static const struct clk_ops rockchip_rk3066_pll_clk_norate_ops = {
    .recalc_rate = rockchip_rk3066_pll_recalc_rate,
    .enable = rockchip_rk3066_pll_enable,
    .disable = rockchip_rk3066_pll_disable,
    .is_enabled = rockchip_rk3066_pll_is_enabled,
    };
    static const struct clk_ops rockchip_rk3066_pll_clk_ops = {
    .recalc_rate = rockchip_rk3066_pll_recalc_rate,
    .determine_rate = rockchip_pll_determine_rate,
    .set_rate = rockchip_rk3066_pll_set_rate,
    .enable = rockchip_rk3066_pll_enable,
    .disable = rockchip_rk3066_pll_disable,
    .is_enabled = rockchip_rk3066_pll_is_enabled,
    .init = rockchip_rk3066_pll_init,
    };
//
// PLL used in RK3399
//

pub const RK3399_PLLCON0_FBDIV_MASK: c_uint = 0xfff;
pub const RK3399_PLLCON0_FBDIV_SHIFT: c_int = 0;
pub const RK3399_PLLCON1_REFDIV_MASK: c_uint = 0x3f;
pub const RK3399_PLLCON1_REFDIV_SHIFT: c_int = 0;
pub const RK3399_PLLCON1_POSTDIV1_MASK: c_uint = 0x7;
pub const RK3399_PLLCON1_POSTDIV1_SHIFT: c_int = 8;
pub const RK3399_PLLCON1_POSTDIV2_MASK: c_uint = 0x7;
pub const RK3399_PLLCON1_POSTDIV2_SHIFT: c_int = 12;
pub const RK3399_PLLCON2_FRAC_MASK: c_uint = 0xffffff;
pub const RK3399_PLLCON2_FRAC_SHIFT: c_int = 0;

pub const RK3399_PLLCON3_DSMPD_MASK: c_uint = 0x1;
pub const RK3399_PLLCON3_DSMPD_SHIFT: c_int = 3;
#[no_mangle]
unsafe extern "C" fn rockchip_rk3399_pll_wait_lock(pll: *mut rockchip_clk_pll) -> c_int {
    static int rockchip_rk3399_pll_wait_lock(struct rockchip_clk_pll *pll)
    {
    u32 pllcon;
    int ret;
//
// Lock time typical 250, max 500 input clock cycles @24MHz
// So define a very safe maximum of 1000us, meaning 24000 cycles.
//
    ret = readl_relaxed_poll_timeout(pll.reg_base + RK3399_PLLCON(2),
    pllcon,
    pllcon & RK3399_PLLCON2_LOCK_STATUS,
    0, 1000);
    if (ret)
    pr_err("%s: timeout waiting for pll to lock\n", __func__);
    return ret;
    }
    static void rockchip_rk3399_pll_get_params(struct rockchip_clk_pll *pll,
    struct rockchip_pll_rate_table *rate)
    {
    u32 pllcon;
    pllcon = readl_relaxed(pll.reg_base + RK3399_PLLCON(0));
    rate.fbdiv = ((pllcon >> RK3399_PLLCON0_FBDIV_SHIFT)
    & RK3399_PLLCON0_FBDIV_MASK);
    pllcon = readl_relaxed(pll.reg_base + RK3399_PLLCON(1));
    rate.refdiv = ((pllcon >> RK3399_PLLCON1_REFDIV_SHIFT)
    & RK3399_PLLCON1_REFDIV_MASK);
    rate.postdiv1 = ((pllcon >> RK3399_PLLCON1_POSTDIV1_SHIFT)
    & RK3399_PLLCON1_POSTDIV1_MASK);
    rate.postdiv2 = ((pllcon >> RK3399_PLLCON1_POSTDIV2_SHIFT)
    & RK3399_PLLCON1_POSTDIV2_MASK);
    pllcon = readl_relaxed(pll.reg_base + RK3399_PLLCON(2));
    rate.frac = ((pllcon >> RK3399_PLLCON2_FRAC_SHIFT)
    & RK3399_PLLCON2_FRAC_MASK);
    pllcon = readl_relaxed(pll.reg_base + RK3399_PLLCON(3));
    rate.dsmpd = ((pllcon >> RK3399_PLLCON3_DSMPD_SHIFT)
    & RK3399_PLLCON3_DSMPD_MASK);
    }
    static unsigned long rockchip_rk3399_pll_recalc_rate(struct clk_hw *hw,
    unsigned long prate)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    struct rockchip_pll_rate_table cur;
    let mut rate64: u64 = prate;
    rockchip_rk3399_pll_get_params(pll, &cur);
    rate64 *= cur.fbdiv;
    do_div(rate64, cur.refdiv);
    if (cur.dsmpd == 0) {
// fractional mode
    let mut frac_rate64: u64 = prate * cur.frac;
    do_div(frac_rate64, cur.refdiv);
    rate64 += frac_rate64 >> 24;
    }
    do_div(rate64, cur.postdiv1);
    do_div(rate64, cur.postdiv2);
    return (unsigned long)rate64;
    }
    static int rockchip_rk3399_pll_set_params(struct rockchip_clk_pll *pll,
    const struct rockchip_pll_rate_table *rate)
    {
    const struct clk_ops *pll_mux_ops = pll.pll_mux_ops;
    struct clk_mux *pll_mux = &pll.pll_mux;
    struct rockchip_pll_rate_table cur;
    u32 pllcon;
    let mut rate_change_remuxed: c_int = 0;
    int cur_parent;
    int ret;
    pr_debug("%s: rate settings for %lu fbdiv: %d, postdiv1: %d, refdiv: %d, postdiv2: %d, dsmpd: %d, frac: %d\n",
    __func__, rate.rate, rate.fbdiv, rate.postdiv1, rate.refdiv,
    rate.postdiv2, rate.dsmpd, rate.frac);
    rockchip_rk3399_pll_get_params(pll, &cur);
    cur.rate = 0;
    cur_parent = pll_mux_ops.get_parent(&pll_mux.hw);
    if (cur_parent == PLL_MODE_NORM) {
    pll_mux_ops.set_parent(&pll_mux.hw, PLL_MODE_SLOW);
    rate_change_remuxed = 1;
    }
// update pll values
    writel_relaxed(HIWORD_UPDATE(rate.fbdiv, RK3399_PLLCON0_FBDIV_MASK,
    RK3399_PLLCON0_FBDIV_SHIFT),
    pll.reg_base + RK3399_PLLCON(0));
    writel_relaxed(HIWORD_UPDATE(rate.refdiv, RK3399_PLLCON1_REFDIV_MASK,
    RK3399_PLLCON1_REFDIV_SHIFT) |
    HIWORD_UPDATE(rate.postdiv1, RK3399_PLLCON1_POSTDIV1_MASK,
    RK3399_PLLCON1_POSTDIV1_SHIFT) |
    HIWORD_UPDATE(rate.postdiv2, RK3399_PLLCON1_POSTDIV2_MASK,
    RK3399_PLLCON1_POSTDIV2_SHIFT),
    pll.reg_base + RK3399_PLLCON(1));
// xPLL CON2 is not HIWORD_MASK
    pllcon = readl_relaxed(pll.reg_base + RK3399_PLLCON(2));
    pllcon &= ~(RK3399_PLLCON2_FRAC_MASK << RK3399_PLLCON2_FRAC_SHIFT);
    pllcon |= rate.frac << RK3399_PLLCON2_FRAC_SHIFT;
    writel_relaxed(pllcon, pll.reg_base + RK3399_PLLCON(2));
    writel_relaxed(HIWORD_UPDATE(rate.dsmpd, RK3399_PLLCON3_DSMPD_MASK,
    RK3399_PLLCON3_DSMPD_SHIFT),
    pll.reg_base + RK3399_PLLCON(3));
// wait for the pll to lock
    ret = rockchip_rk3399_pll_wait_lock(pll);
    if (ret) {
    pr_warn("%s: pll update unsuccessful, trying to restore old params\n",
    __func__);
    rockchip_rk3399_pll_set_params(pll, &cur);
    }
    if (rate_change_remuxed)
    pll_mux_ops.set_parent(&pll_mux.hw, PLL_MODE_NORM);
    return ret;
    }
    static int rockchip_rk3399_pll_set_rate(struct clk_hw *hw, unsigned long drate,
    unsigned long prate)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    const struct rockchip_pll_rate_table *rate;
    pr_debug("%s: changing %s to %lu with a parent rate of %lu\n",
    __func__, __clk_get_name(hw.clk), drate, prate);
// Get required rate settings from table
    rate = rockchip_get_pll_settings(pll, drate);
    if (!rate) {
    pr_err("%s: Invalid rate : %lu for pll clk %s\n", __func__,
    drate, __clk_get_name(hw.clk));
    return -EINVAL;
    }
    return rockchip_rk3399_pll_set_params(pll, rate);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_rk3399_pll_enable(hw: *mut clk_hw) -> c_int {
    static int rockchip_rk3399_pll_enable(struct clk_hw *hw)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    writel(HIWORD_UPDATE(0, RK3399_PLLCON3_PWRDOWN, 0),
    pll.reg_base + RK3399_PLLCON(3));
    rockchip_rk3399_pll_wait_lock(pll);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_rk3399_pll_disable(hw: *mut clk_hw) {
    static void rockchip_rk3399_pll_disable(struct clk_hw *hw)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    writel(HIWORD_UPDATE(RK3399_PLLCON3_PWRDOWN,
    RK3399_PLLCON3_PWRDOWN, 0),
    pll.reg_base + RK3399_PLLCON(3));
    }
#[no_mangle]
unsafe extern "C" fn rockchip_rk3399_pll_is_enabled(hw: *mut clk_hw) -> c_int {
    static int rockchip_rk3399_pll_is_enabled(struct clk_hw *hw)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    let mut pllcon: u32 = readl(pll.reg_base + RK3399_PLLCON(3));
    return !(pllcon & RK3399_PLLCON3_PWRDOWN);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_rk3399_pll_init(hw: *mut clk_hw) -> c_int {
    static int rockchip_rk3399_pll_init(struct clk_hw *hw)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    const struct rockchip_pll_rate_table *rate;
    struct rockchip_pll_rate_table cur;
    unsigned long drate;
    if (!(pll.flags & ROCKCHIP_PLL_SYNC_RATE))
    return 0;
    drate = clk_hw_get_rate(hw);
    rate = rockchip_get_pll_settings(pll, drate);
// when no rate setting for the current rate, rely on clk_set_rate
    if (!rate)
    return 0;
    rockchip_rk3399_pll_get_params(pll, &cur);
    pr_debug("%s: pll %s@%lu: Hz\n", __func__, __clk_get_name(hw.clk),
    drate);
    pr_debug("old - fbdiv: %d, postdiv1: %d, refdiv: %d, postdiv2: %d, dsmpd: %d, frac: %d\n",
    cur.fbdiv, cur.postdiv1, cur.refdiv, cur.postdiv2,
    cur.dsmpd, cur.frac);
    pr_debug("new - fbdiv: %d, postdiv1: %d, refdiv: %d, postdiv2: %d, dsmpd: %d, frac: %d\n",
    rate.fbdiv, rate.postdiv1, rate.refdiv, rate.postdiv2,
    rate.dsmpd, rate.frac);
    if (rate.fbdiv != cur.fbdiv || rate.postdiv1 != cur.postdiv1 ||
    rate.refdiv != cur.refdiv || rate.postdiv2 != cur.postdiv2 ||
    rate.dsmpd != cur.dsmpd ||
    (!cur.dsmpd && (rate.frac != cur.frac))) {
    struct clk *parent = clk_get_parent(hw.clk);
    if (!parent) {
    pr_warn("%s: parent of %s not available\n",
    __func__, __clk_get_name(hw.clk));
    return 0;
    }
    pr_debug("%s: pll %s: rate params do not match rate table, adjusting\n",
    __func__, __clk_get_name(hw.clk));
    rockchip_rk3399_pll_set_params(pll, rate);
    }
    return 0;
    }
    static const struct clk_ops rockchip_rk3399_pll_clk_norate_ops = {
    .recalc_rate = rockchip_rk3399_pll_recalc_rate,
    .enable = rockchip_rk3399_pll_enable,
    .disable = rockchip_rk3399_pll_disable,
    .is_enabled = rockchip_rk3399_pll_is_enabled,
    };
    static const struct clk_ops rockchip_rk3399_pll_clk_ops = {
    .recalc_rate = rockchip_rk3399_pll_recalc_rate,
    .determine_rate = rockchip_pll_determine_rate,
    .set_rate = rockchip_rk3399_pll_set_rate,
    .enable = rockchip_rk3399_pll_enable,
    .disable = rockchip_rk3399_pll_disable,
    .is_enabled = rockchip_rk3399_pll_is_enabled,
    .init = rockchip_rk3399_pll_init,
    };
//
// PLL used in RK3588
//

pub const RK3588_PLLCON0_M_MASK: c_uint = 0x3ff;
pub const RK3588_PLLCON0_M_SHIFT: c_int = 0;
pub const RK3588_PLLCON1_P_MASK: c_uint = 0x3f;
pub const RK3588_PLLCON1_P_SHIFT: c_int = 0;
pub const RK3588_PLLCON1_S_MASK: c_uint = 0x7;
pub const RK3588_PLLCON1_S_SHIFT: c_int = 6;
pub const RK3588_PLLCON2_K_MASK: c_uint = 0xffff;
pub const RK3588_PLLCON2_K_SHIFT: c_int = 0;

#[no_mangle]
unsafe extern "C" fn rockchip_rk3588_pll_wait_lock(pll: *mut rockchip_clk_pll) -> c_int {
    static int rockchip_rk3588_pll_wait_lock(struct rockchip_clk_pll *pll)
    {
    u32 pllcon;
    int ret;
//
// Lock time typical 250, max 500 input clock cycles @24MHz
// So define a very safe maximum of 1000us, meaning 24000 cycles.
//
    ret = readl_relaxed_poll_timeout(pll.reg_base + RK3588_PLLCON(6),
    pllcon,
    pllcon & RK3588_PLLCON6_LOCK_STATUS,
    0, 1000);
    if (ret)
    pr_err("%s: timeout waiting for pll to lock\n", __func__);
    return ret;
    }
    static void rockchip_rk3588_pll_get_params(struct rockchip_clk_pll *pll,
    struct rockchip_pll_rate_table *rate)
    {
    u32 pllcon;
    pllcon = readl_relaxed(pll.reg_base + RK3588_PLLCON(0));
    rate.m = ((pllcon >> RK3588_PLLCON0_M_SHIFT) & RK3588_PLLCON0_M_MASK);
    pllcon = readl_relaxed(pll.reg_base + RK3588_PLLCON(1));
    rate.p = ((pllcon >> RK3588_PLLCON1_P_SHIFT) & RK3588_PLLCON1_P_MASK);
    rate.s = ((pllcon >> RK3588_PLLCON1_S_SHIFT) & RK3588_PLLCON1_S_MASK);
    pllcon = readl_relaxed(pll.reg_base + RK3588_PLLCON(2));
    rate.k = ((pllcon >> RK3588_PLLCON2_K_SHIFT) & RK3588_PLLCON2_K_MASK);
    }
//
// 2250 MHz <= Fvco <= 4500 MHz
// For Fvco > 3 GHz: period jitter +-1% frac PLL, +-0.75% int PLL
// For Fvco < 3 GHz: period jitter +-2% frac PLL, +-1.50% int PLL
// Fvco = ((m + k / 65536) * Fin) / p
// Fout = ((m + k / 65536) * Fin) / (p * 2^s)
// -32768 <= k <= 32767 (only available in frac PLLs, not int PLLs)
//
#[no_mangle]
unsafe extern "C" fn rockchip_rk3588_pll_recalc_rate(hw: *mut clk_hw, prate: c_ulong) -> c_ulong {
    static unsigned long rockchip_rk3588_pll_recalc_rate(struct clk_hw *hw, unsigned long prate)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    struct rockchip_pll_rate_table cur;
    let mut rate64: u64 = prate, postdiv;
    rockchip_rk3588_pll_get_params(pll, &cur);
    rate64 *= cur.m;
    do_div(rate64, cur.p);
    if (cur.k) {
// fractional mode
    let mut frac_rate64: i64 = (s64)prate * cur.k;
    postdiv = cur.p * 65536;
    rate64 += div_s64(frac_rate64, postdiv);
    }
    rate64 = rate64 >> cur.s;
    if (pll.type == pll_rk3588_ddr)
    return (unsigned long)rate64 * 2;
    else
    return (unsigned long)rate64;
    }
    static int rockchip_rk3588_pll_set_params(struct rockchip_clk_pll *pll,
    const struct rockchip_pll_rate_table *rate)
    {
    const struct clk_ops *pll_mux_ops = pll.pll_mux_ops;
    struct clk_mux *pll_mux = &pll.pll_mux;
    struct rockchip_pll_rate_table cur;
    let mut rate_change_remuxed: c_int = 0;
    int cur_parent;
    int ret;
    pr_debug("%s: rate settings for %lu p: %d, m: %d, s: %d, k: %d\n",
    __func__, rate.rate, rate.p, rate.m, rate.s, rate.k);
    rockchip_rk3588_pll_get_params(pll, &cur);
    cur.rate = 0;
    if (pll.type == pll_rk3588) {
    cur_parent = pll_mux_ops.get_parent(&pll_mux.hw);
    if (cur_parent == PLL_MODE_NORM) {
    pll_mux_ops.set_parent(&pll_mux.hw, PLL_MODE_SLOW);
    rate_change_remuxed = 1;
    }
    }
// set pll power down
    writel(HIWORD_UPDATE(RK3588_PLLCON1_PWRDOWN,
    RK3588_PLLCON1_PWRDOWN, 0),
    pll.reg_base + RK3399_PLLCON(1));
// update pll values
    writel_relaxed(HIWORD_UPDATE(rate.m, RK3588_PLLCON0_M_MASK, RK3588_PLLCON0_M_SHIFT),
    pll.reg_base + RK3399_PLLCON(0));
    writel_relaxed(HIWORD_UPDATE(rate.p, RK3588_PLLCON1_P_MASK, RK3588_PLLCON1_P_SHIFT) |
    HIWORD_UPDATE(rate.s, RK3588_PLLCON1_S_MASK, RK3588_PLLCON1_S_SHIFT),
    pll.reg_base + RK3399_PLLCON(1));
    writel_relaxed(HIWORD_UPDATE(rate.k, RK3588_PLLCON2_K_MASK, RK3588_PLLCON2_K_SHIFT),
    pll.reg_base + RK3399_PLLCON(2));
// set pll power up
    writel(HIWORD_UPDATE(0, RK3588_PLLCON1_PWRDOWN, 0),
    pll.reg_base + RK3588_PLLCON(1));
// wait for the pll to lock
    ret = rockchip_rk3588_pll_wait_lock(pll);
    if (ret) {
    pr_warn("%s: pll update unsuccessful, trying to restore old params\n",
    __func__);
    rockchip_rk3588_pll_set_params(pll, &cur);
    }
    if ((pll.type == pll_rk3588) && rate_change_remuxed)
    pll_mux_ops.set_parent(&pll_mux.hw, PLL_MODE_NORM);
    return ret;
    }
    static int rockchip_rk3588_pll_set_rate(struct clk_hw *hw, unsigned long drate,
    unsigned long prate)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    const struct rockchip_pll_rate_table *rate;
    pr_debug("%s: changing %s to %lu with a parent rate of %lu\n",
    __func__, __clk_get_name(hw.clk), drate, prate);
// Get required rate settings from table
    rate = rockchip_get_pll_settings(pll, drate);
    if (!rate) {
    pr_err("%s: Invalid rate : %lu for pll clk %s\n", __func__,
    drate, __clk_get_name(hw.clk));
    return -EINVAL;
    }
    return rockchip_rk3588_pll_set_params(pll, rate);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_rk3588_pll_enable(hw: *mut clk_hw) -> c_int {
    static int rockchip_rk3588_pll_enable(struct clk_hw *hw)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    writel(HIWORD_UPDATE(0, RK3588_PLLCON1_PWRDOWN, 0),
    pll.reg_base + RK3588_PLLCON(1));
    rockchip_rk3588_pll_wait_lock(pll);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_rk3588_pll_disable(hw: *mut clk_hw) {
    static void rockchip_rk3588_pll_disable(struct clk_hw *hw)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    writel(HIWORD_UPDATE(RK3588_PLLCON1_PWRDOWN, RK3588_PLLCON1_PWRDOWN, 0),
    pll.reg_base + RK3588_PLLCON(1));
    }
#[no_mangle]
unsafe extern "C" fn rockchip_rk3588_pll_is_enabled(hw: *mut clk_hw) -> c_int {
    static int rockchip_rk3588_pll_is_enabled(struct clk_hw *hw)
    {
    struct rockchip_clk_pll *pll = to_rockchip_clk_pll(hw);
    let mut pllcon: u32 = readl(pll.reg_base + RK3588_PLLCON(1));
    return !(pllcon & RK3588_PLLCON1_PWRDOWN);
    }
    static const struct clk_ops rockchip_rk3588_pll_clk_norate_ops = {
    .recalc_rate = rockchip_rk3588_pll_recalc_rate,
    .enable = rockchip_rk3588_pll_enable,
    .disable = rockchip_rk3588_pll_disable,
    .is_enabled = rockchip_rk3588_pll_is_enabled,
    };
    static const struct clk_ops rockchip_rk3588_pll_clk_ops = {
    .recalc_rate = rockchip_rk3588_pll_recalc_rate,
    .determine_rate = rockchip_pll_determine_rate,
    .set_rate = rockchip_rk3588_pll_set_rate,
    .enable = rockchip_rk3588_pll_enable,
    .disable = rockchip_rk3588_pll_disable,
    .is_enabled = rockchip_rk3588_pll_is_enabled,
    };
//
// Common registering of pll clocks
//
    struct clk *rockchip_clk_register_pll(struct rockchip_clk_provider *ctx,
    enum rockchip_pll_type pll_type,
    const char *name, const char *const *parent_names,
    u8 num_parents, int con_offset, int grf_lock_offset,
    int lock_shift, int mode_offset, int mode_shift,
    struct rockchip_pll_rate_table *rate_table,
    unsigned long flags, u8 clk_pll_flags)
    {
    const char *pll_parents[3];
    struct clk_init_data init;
    struct rockchip_clk_pll *pll;
    struct clk_mux *pll_mux;
    struct clk *pll_clk, *mux_clk;
    char pll_name[20];
    if ((pll_type != pll_rk3328 && num_parents != 2) ||
    (pll_type == pll_rk3328 && num_parents != 1)) {
    pr_err("%s: needs two parent clocks\n", __func__);
    return ERR_PTR(-EINVAL);
    }
// name the actual pll
    snprintf(pll_name, sizeof(pll_name), "pll_%s", name);
    pll = kzalloc_obj(*pll);
    if (!pll)
    return ERR_PTR(-ENOMEM);
// create the mux on top of the real pll
    pll.pll_mux_ops = &clk_mux_ops;
    pll_mux = &pll.pll_mux;
    pll_mux.reg = ctx.reg_base + mode_offset;
    pll_mux.shift = mode_shift;
    if (pll_type == pll_rk3328)
    pll_mux.mask = PLL_RK3328_MODE_MASK;
    else
    pll_mux.mask = PLL_MODE_MASK;
    pll_mux.flags = 0;
    pll_mux.lock = &ctx.lock;
    pll_mux.hw.init = &init;
    if (pll_type == pll_rk3036 ||
    pll_type == pll_rk3066 ||
    pll_type == pll_rk3328 ||
    pll_type == pll_rk3399 ||
    pll_type == pll_rk3588)
    pll_mux.flags |= CLK_MUX_HIWORD_MASK;
// the actual muxing is xin24m, pll-output, xin32k
    pll_parents[0] = parent_names[0];
    pll_parents[1] = pll_name;
    pll_parents[2] = parent_names[1];
    init.name = name;
    init.flags = CLK_SET_RATE_PARENT;
    init.ops = pll.pll_mux_ops;
    init.parent_names = pll_parents;
    if (pll_type == pll_rk3328)
    init.num_parents = 2;
    else
    init.num_parents = ARRAY_SIZE(pll_parents);
    mux_clk = clk_register(core::ptr::null_mut(), &pll_mux.hw);
    if (IS_ERR(mux_clk))
    goto err_mux;
// now create the actual pll
    init.name = pll_name;
// keep all plls untouched for now
    init.flags = flags | CLK_IGNORE_UNUSED;
    init.parent_names = &parent_names[0];
    init.num_parents = 1;
    if (rate_table) {
    int len;
// find count of rates in rate_table
    for (len = 0; rate_table[len].rate != 0; )
    len++;
    pll.rate_count = len;
    pll.rate_table = kmemdup_array(rate_table,
    pll.rate_count,
    sizeof(*pll.rate_table),
    GFP_KERNEL);
    WARN(!pll.rate_table,
    "%s: could not allocate rate table for %s\n",
    __func__, name);
    }
    switch (pll_type) {
    case pll_rk3036:
    case pll_rk3328:
    if (!pll.rate_table)
    init.ops = &rockchip_rk3036_pll_clk_norate_ops;
    else
    init.ops = &rockchip_rk3036_pll_clk_ops;
    break;
    case pll_rk3066:
    if (!pll.rate_table || IS_ERR(ctx.grf))
    init.ops = &rockchip_rk3066_pll_clk_norate_ops;
    else
    init.ops = &rockchip_rk3066_pll_clk_ops;
    break;
    case pll_rk3399:
    if (!pll.rate_table)
    init.ops = &rockchip_rk3399_pll_clk_norate_ops;
    else
    init.ops = &rockchip_rk3399_pll_clk_ops;
    break;
    case pll_rk3588:
    case pll_rk3588_core:
    case pll_rk3588_ddr:
    if (!pll.rate_table)
    init.ops = &rockchip_rk3588_pll_clk_norate_ops;
    else
    init.ops = &rockchip_rk3588_pll_clk_ops;
    init.flags = flags;
    break;
    default:
    pr_warn("%s: Unknown pll type for pll clk %s\n",
    __func__, name);
    }
    pll.hw.init = &init;
    pll.type = pll_type;
    pll.reg_base = ctx.reg_base + con_offset;
    pll.lock_offset = grf_lock_offset;
    pll.lock_shift = lock_shift;
    pll.flags = clk_pll_flags;
    pll.lock = &ctx.lock;
    pll.ctx = ctx;
    pll_clk = clk_register(core::ptr::null_mut(), &pll.hw);
    if (IS_ERR(pll_clk)) {
    pr_err("%s: failed to register pll clock %s : %ld\n",
    __func__, name, PTR_ERR(pll_clk));
    goto err_pll;
    }
    return mux_clk;
    err_pll:
    kfree(pll.rate_table);
    clk_unregister(mux_clk);
    mux_clk = pll_clk;
    err_mux:
    kfree(pll);
    return mux_clk;
    }
