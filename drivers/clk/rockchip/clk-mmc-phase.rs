//! Automatically rewritten from C to Rust
//! Source: drivers/clk/rockchip/clk-mmc-phase.c
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
// Copyright 2014 Google, Inc
// Author: Alexandru M Stan <amstan@chromium.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_mmc_clock {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub grf: *mut regmap,
    pub grf_reg: c_int,
    pub shift: c_int,
    pub cached_phase: c_int,
    pub clk_rate_change_nb: notifier_block,
}

pub const RK3288_MMC_CLKGEN_DIV: c_int = 2;
    static unsigned long rockchip_mmc_recalc(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    return parent_rate / RK3288_MMC_CLKGEN_DIV;
    }

pub const ROCKCHIP_MMC_DEGREE_MASK: c_uint = 0x3;
pub const ROCKCHIP_MMC_DELAYNUM_OFFSET: c_int = 2;

//
// Each fine delay is between 44ps-77ps. Assume each fine delay is 60ps to
// simplify calculations. So 45degs could be anywhere between 33deg and 57.8deg.
//
pub const ROCKCHIP_MMC_DELAY_ELEMENT_PSEC: c_int = 60;
#[no_mangle]
unsafe extern "C" fn rockchip_mmc_get_phase(hw: *mut clk_hw) -> c_int {
    static int rockchip_mmc_get_phase(struct clk_hw *hw)
    {
    struct rockchip_mmc_clock *mmc_clock = to_mmc_clock(hw);
    let mut rate: c_ulong = clk_hw_get_rate(hw);
    u32 raw_value;
    u16 degrees;
    let mut delay_num: u32 = 0;
// Constant signal, no measurable phase shift
    if (!rate)
    return 0;
    if (mmc_clock.grf)
    regmap_read(mmc_clock.grf, mmc_clock.grf_reg, &raw_value);
    else
    raw_value = readl(mmc_clock.reg);
    raw_value >>= mmc_clock.shift;
    degrees = (raw_value & ROCKCHIP_MMC_DEGREE_MASK) * 90;
    if (raw_value & ROCKCHIP_MMC_DELAY_SEL) {
// degrees/delaynum * 1000000
    unsigned long factor = (ROCKCHIP_MMC_DELAY_ELEMENT_PSEC / 10) *
    36 * (rate / 10000);
    delay_num = (raw_value & ROCKCHIP_MMC_DELAYNUM_MASK);
    delay_num >>= ROCKCHIP_MMC_DELAYNUM_OFFSET;
    degrees += DIV_ROUND_CLOSEST(delay_num * factor, 1000000);
    }
    return degrees % 360;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_mmc_set_phase(hw: *mut clk_hw, degrees: c_int) -> c_int {
    static int rockchip_mmc_set_phase(struct clk_hw *hw, int degrees)
    {
    struct rockchip_mmc_clock *mmc_clock = to_mmc_clock(hw);
    let mut rate: c_ulong = clk_hw_get_rate(hw);
    u8 nineties, remainder;
    u8 delay_num;
    u32 raw_value;
    u32 delay;
//
// The below calculation is based on the output clock from
// MMC host to the card, which expects the phase clock inherits
// the clock rate from its parent, namely the output clock
// provider of MMC host. However, things may go wrong if
// (1) It is orphan.
// (2) It is assigned to the wrong parent.
//
// This check help debug the case (1), which seems to be the
// most likely problem we often face and which makes it difficult
// for people to debug unstable mmc tuning results.
//
    if (!rate) {
    pr_err("%s: invalid clk rate\n", __func__);
    return -EINVAL;
    }
    nineties = degrees / 90;
    remainder = (degrees % 90);
//
// Due to the inexact nature of the "fine" delay, we might
// actually go non-monotonic.  We don't go _too_ monotonic
// though, so we should be OK.  Here are options of how we may
// work:
//
// Ideally we end up with:
// 1.0, 2.0, ..., 69.0, 70.0, ...,  89.0, 90.0
//
// On one extreme (if delay is actually 44ps):
// .73, 1.5, ..., 50.6, 51.3, ...,  65.3, 90.0
// The other (if delay is actually 77ps):
// 1.3, 2.6, ..., 88.6. 89.8, ..., 114.0, 90
//
// It's possible we might make a delay that is up to 25
// degrees off from what we think we're making.  That's OK
// though because we should be REALLY far from any bad range.
//
// Convert to delay; do a little extra work to make sure we
// don't overflow 32-bit / 64-bit numbers.
//
    delay = 10000000; /* PSECS_PER_SEC / 10000 / 10 */
    delay *= remainder;
    delay = DIV_ROUND_CLOSEST(delay,
    (rate / 1000) * 36 *
    (ROCKCHIP_MMC_DELAY_ELEMENT_PSEC / 10));
    delay_num = (u8) min_t(u32, delay, 255);
    raw_value = delay_num ? ROCKCHIP_MMC_DELAY_SEL : 0;
    raw_value |= delay_num << ROCKCHIP_MMC_DELAYNUM_OFFSET;
    raw_value |= nineties;
    raw_value = HIWORD_UPDATE(raw_value, 0x07ff, mmc_clock.shift);
    if (mmc_clock.grf)
    regmap_write(mmc_clock.grf, mmc_clock.grf_reg, raw_value);
    else
    writel(raw_value, mmc_clock.reg);
    pr_debug("%s.set_phase(%d) delay_nums=%u reg[0x%p]=0x%03x actual_degrees=%d\n",
    clk_hw_get_name(hw), degrees, delay_num,
    mmc_clock.reg, raw_value>>(mmc_clock.shift),
    rockchip_mmc_get_phase(hw)
    );
    return 0;
    }
    static const struct clk_ops rockchip_mmc_clk_ops = {
    .recalc_rate	= rockchip_mmc_recalc,
    .get_phase	= rockchip_mmc_get_phase,
    .set_phase	= rockchip_mmc_set_phase,
    };

    container_of(x, struct rockchip_mmc_clock, clk_rate_change_nb)
    static int rockchip_mmc_clk_rate_notify(struct notifier_block *nb,
    unsigned long event, void *data)
    {
    struct rockchip_mmc_clock *mmc_clock = to_rockchip_mmc_clock(nb);
    struct clk_notifier_data *ndata = data;
//
// rockchip_mmc_clk is mostly used by mmc controllers to sample
// the input data, which expects the fixed phase after the tuning
// process. However if the clock rate is changed, the phase is stale
// and may break the data sampling. So here we try to restore the phase
// for that case, except that
// (1) cached_phase is invalid since we inevitably cached it when the
// clock provider be reparented from orphan to its real parent in the
// first place. Otherwise we may mess up the initialization of MMC cards
// since we only set the default sample phase and drive phase later on.
// (2) the new coming rate is higher than the older one since mmc driver
// set the max-frequency to match the boards' ability but we can't go
// over the heads of that, otherwise the tests smoke out the issue.
//
    if (ndata.old_rate <= ndata.new_rate)
    return NOTIFY_DONE;
    if (event == PRE_RATE_CHANGE)
    mmc_clock.cached_phase =
    rockchip_mmc_get_phase(&mmc_clock.hw);
    else if (mmc_clock.cached_phase != -EINVAL &&
    event == POST_RATE_CHANGE)
    rockchip_mmc_set_phase(&mmc_clock.hw, mmc_clock.cached_phase);
    return NOTIFY_DONE;
    }
    struct clk *rockchip_clk_register_mmc(const char *name,
    const char *const *parent_names, u8 num_parents,
    void __iomem *reg,
    struct regmap *grf, int grf_reg,
    int shift)
    {
    struct clk_init_data init;
    struct rockchip_mmc_clock *mmc_clock;
    struct clk *clk;
    int ret;
    mmc_clock = kmalloc_obj(*mmc_clock);
    if (!mmc_clock)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.flags = 0;
    init.num_parents = num_parents;
    init.parent_names = parent_names;
    init.ops = &rockchip_mmc_clk_ops;
    mmc_clock.hw.init = &init;
    mmc_clock.reg = reg;
    mmc_clock.grf = grf;
    mmc_clock.grf_reg = grf_reg;
    mmc_clock.shift = shift;
    clk = clk_register(core::ptr::null_mut(), &mmc_clock.hw);
    if (IS_ERR(clk)) {
    ret = PTR_ERR(clk);
    goto err_register;
    }
    mmc_clock.clk_rate_change_nb.notifier_call =
    &rockchip_mmc_clk_rate_notify;
    ret = clk_notifier_register(clk, &mmc_clock.clk_rate_change_nb);
    if (ret)
    goto err_notifier;
    return clk;
    err_notifier:
    clk_unregister(clk);
    err_register:
    kfree(mmc_clock);
    return ERR_PTR(ret);
    }
