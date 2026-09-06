//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-apple-nco.c
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Driver for an SoC block (Numerically Controlled Oscillator)
// found on t8103 (M1) and other Apple chips
//
// Copyright (C) The Asahi Linux Contributors
//

pub const NCO_CHANNEL_STRIDE: c_uint = 0x4000;
pub const NCO_CHANNEL_REGSIZE: c_int = 20;
pub const REG_CTRL: c_int = 0;

pub const REG_DIV: c_int = 4;

pub const REG_INC1: c_int = 8;
pub const REG_INC2: c_int = 12;
pub const REG_ACCINIT: c_int = 16;
//
// Theory of operation (postulated)
//
// The REG_DIV register indirectly expresses a base integer divisor, roughly
// corresponding to twice the desired ratio of input to output clock. This
// base divisor is adjusted on a cycle-by-cycle basis based on the state of a
// 32-bit phase accumulator to achieve a desired precise clock ratio over the
// long term.
//
// Specifically an output clock cycle is produced after (REG_DIV divisor)/2
// or (REG_DIV divisor + 1)/2 input cycles, the latter taking effect when top
// bit of the 32-bit accumulator is set. The accumulator is incremented each
// produced output cycle, by the value from either REG_INC1 or REG_INC2, which
// of the two is selected depending again on the accumulator's current top bit.
//
// Because the NCO hardware implements counting of input clock cycles in part
// in a Galois linear-feedback shift register, the higher bits of divisor
// are programmed into REG_DIV by picking an appropriate LFSR state. See
// applnco_compute_tables/applnco_div_translate for details on this.
//
pub const LFSR_POLY: c_uint = 0xa01;
pub const LFSR_INIT: c_uint = 0x7ff;
pub const LFSR_LEN: c_int = 11;

// The minimal attainable coarse divisor (first value in table)
pub const COARSE_DIV_OFFSET: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct applnco_tables {
    pub fwd: [u16; LFSR_TBLSIZE],
    pub inv: [u16; LFSR_TBLSIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct applnco_channel {
    pub base: *mut void __iomem,
    pub tbl: *mut applnco_tables,
    pub hw: clk_hw,
    pub lock: spinlock_t,
}

#[no_mangle]
unsafe extern "C" fn applnco_enable_nolock(hw: *mut clk_hw) {
    static void applnco_enable_nolock(struct clk_hw *hw)
    {
    struct applnco_channel *chan = to_applnco_channel(hw);
    u32 val;
    val = readl_relaxed(chan.base + REG_CTRL);
    writel_relaxed(val | CTRL_ENABLE, chan.base + REG_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn applnco_disable_nolock(hw: *mut clk_hw) {
    static void applnco_disable_nolock(struct clk_hw *hw)
    {
    struct applnco_channel *chan = to_applnco_channel(hw);
    u32 val;
    val = readl_relaxed(chan.base + REG_CTRL);
    writel_relaxed(val & ~CTRL_ENABLE, chan.base + REG_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn applnco_is_enabled(hw: *mut clk_hw) -> c_int {
    static int applnco_is_enabled(struct clk_hw *hw)
    {
    struct applnco_channel *chan = to_applnco_channel(hw);
    return (readl_relaxed(chan.base + REG_CTRL) & CTRL_ENABLE) != 0;
    }
#[no_mangle]
unsafe extern "C" fn applnco_compute_tables(tbl: *mut applnco_tables) {
    static void applnco_compute_tables(struct applnco_tables *tbl)
    {
    int i;
    let mut state: u32 = LFSR_INIT;
//
// Go through the states of a Galois LFSR and build
// a coarse divisor translation table.
//
    for (i = LFSR_PERIOD; i > 0; i--) {
    if (state & 1)
    state = (state >> 1) ^ (LFSR_POLY >> 1);
    else
    state = (state >> 1);
    tbl.fwd[i] = state;
    tbl.inv[state] = i;
    }
// Zero value is special-cased
    tbl.fwd[0] = 0;
    tbl.inv[0] = 0;
    }
#[no_mangle]
unsafe extern "C" fn applnco_div_out_of_range(div: c_uint) -> bool {
    static bool applnco_div_out_of_range(unsigned int div)
    {
    let mut coarse: c_uint = div / 4;
    return coarse < COARSE_DIV_OFFSET ||
    coarse >= COARSE_DIV_OFFSET + LFSR_TBLSIZE;
    }
#[no_mangle]
unsafe extern "C" fn applnco_div_translate(tbl: *mut applnco_tables, div: c_uint) -> u32 {
    static u32 applnco_div_translate(struct applnco_tables *tbl, unsigned int div)
    {
    let mut coarse: c_uint = div / 4;
    if (WARN_ON(applnco_div_out_of_range(div)))
    return 0;
    return FIELD_PREP(DIV_COARSE, tbl.fwd[coarse - COARSE_DIV_OFFSET]) |
    FIELD_PREP(DIV_FINE, div % 4);
    }
#[no_mangle]
unsafe extern "C" fn applnco_div_translate_inv(tbl: *mut applnco_tables, regval: u32) -> c_uint {
    static unsigned int applnco_div_translate_inv(struct applnco_tables *tbl, u32 regval)
    {
    unsigned int coarse, fine;
    coarse = tbl.inv[FIELD_GET(DIV_COARSE, regval)] + COARSE_DIV_OFFSET;
    fine = FIELD_GET(DIV_FINE, regval);
    return coarse * 4 + fine;
    }
    static int applnco_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct applnco_channel *chan = to_applnco_channel(hw);
    unsigned long flags;
    u32 div, inc1, inc2;
    bool was_enabled;
    div = 2 * parent_rate / rate;
    inc1 = 2 * parent_rate - div * rate;
    inc2 = inc1 - rate;
    if (applnco_div_out_of_range(div))
    return -EINVAL;
    div = applnco_div_translate(chan.tbl, div);
    spin_lock_irqsave(&chan.lock, flags);
    was_enabled = applnco_is_enabled(hw);
    applnco_disable_nolock(hw);
    writel_relaxed(div,  chan.base + REG_DIV);
    writel_relaxed(inc1, chan.base + REG_INC1);
    writel_relaxed(inc2, chan.base + REG_INC2);
// Presumably a neutral initial value for accumulator
    writel_relaxed(1 << 31, chan.base + REG_ACCINIT);
    if (was_enabled)
    applnco_enable_nolock(hw);
    spin_unlock_irqrestore(&chan.lock, flags);
    return 0;
    }
    static unsigned long applnco_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct applnco_channel *chan = to_applnco_channel(hw);
    u32 div, inc1, inc2, incbase;
    div = applnco_div_translate_inv(chan.tbl,
    readl_relaxed(chan.base + REG_DIV));
    inc1 = readl_relaxed(chan.base + REG_INC1);
    inc2 = readl_relaxed(chan.base + REG_INC2);
//
// We don't support wraparound of accumulator
// nor the edge case of both increments being zero
//
    if (inc1 >= (1 << 31) || inc2 < (1 << 31) || (inc1 == 0 && inc2 == 0))
    return 0;
// Scale both sides of division by incbase to maintain precision
    incbase = inc1 - inc2;
    return div64_u64(((u64) parent_rate) * 2 * incbase,
    ((u64) div) * incbase + inc1);
    }
    static int applnco_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    let mut lo: c_ulong = req.best_parent_rate / (COARSE_DIV_OFFSET + LFSR_TBLSIZE) + 1;
    let mut hi: c_ulong = req.best_parent_rate / COARSE_DIV_OFFSET;
    req.rate = clamp(req.rate, lo, hi);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn applnco_enable(hw: *mut clk_hw) -> c_int {
    static int applnco_enable(struct clk_hw *hw)
    {
    struct applnco_channel *chan = to_applnco_channel(hw);
    unsigned long flags;
    spin_lock_irqsave(&chan.lock, flags);
    applnco_enable_nolock(hw);
    spin_unlock_irqrestore(&chan.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn applnco_disable(hw: *mut clk_hw) {
    static void applnco_disable(struct clk_hw *hw)
    {
    struct applnco_channel *chan = to_applnco_channel(hw);
    unsigned long flags;
    spin_lock_irqsave(&chan.lock, flags);
    applnco_disable_nolock(hw);
    spin_unlock_irqrestore(&chan.lock, flags);
    }
    static const struct clk_ops applnco_ops = {
    .set_rate = applnco_set_rate,
    .recalc_rate = applnco_recalc_rate,
    .determine_rate = applnco_determine_rate,
    .enable = applnco_enable,
    .disable = applnco_disable,
    .is_enabled = applnco_is_enabled,
    };
#[no_mangle]
unsafe extern "C" fn applnco_probe(pdev: *mut platform_device) -> c_int {
    static int applnco_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    let mut pdata: clk_parent_data = { .index = 0 };
    struct clk_init_data init;
    struct clk_hw_onecell_data *onecell_data;
    void __iomem *base;
    struct resource *res;
    struct applnco_tables *tbl;
    unsigned int nchannels;
    int ret, i;
    base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(base))
    return PTR_ERR(base);
    if (resource_size(res) < NCO_CHANNEL_REGSIZE)
    return -EINVAL;
    nchannels = (resource_size(res) - NCO_CHANNEL_REGSIZE)
    / NCO_CHANNEL_STRIDE + 1;
    onecell_data = devm_kzalloc(&pdev.dev, struct_size(onecell_data, hws,
    nchannels), GFP_KERNEL);
    if (!onecell_data)
    return -ENOMEM;
    onecell_data.num = nchannels;
    tbl = devm_kzalloc(&pdev.dev, sizeof(*tbl), GFP_KERNEL);
    if (!tbl)
    return -ENOMEM;
    applnco_compute_tables(tbl);
    for (i = 0; i < nchannels; i++) {
    struct applnco_channel *chan;
    chan = devm_kzalloc(&pdev.dev, sizeof(*chan), GFP_KERNEL);
    if (!chan)
    return -ENOMEM;
    chan.base = base + NCO_CHANNEL_STRIDE * i;
    chan.tbl = tbl;
    spin_lock_init(&chan.lock);
    memset(&init, 0, sizeof(init));
    init.name = devm_kasprintf(&pdev.dev, GFP_KERNEL,
    "%s-%d", np.name, i);
    if (!init.name)
    return -ENOMEM;
    init.ops = &applnco_ops;
    init.parent_data = &pdata;
    init.num_parents = 1;
    init.flags = 0;
    chan.hw.init = &init;
    ret = devm_clk_hw_register(&pdev.dev, &chan.hw);
    if (ret)
    return ret;
    onecell_data.hws[i] = &chan.hw;
    }
    return devm_of_clk_add_hw_provider(&pdev.dev, of_clk_hw_onecell_get,
    onecell_data);
    }
    static const struct of_device_id applnco_ids[] = {
    { .compatible = "apple,t8103-nco" },
    { .compatible = "apple,nco" },
    { }
    };
    MODULE_DEVICE_TABLE(of, applnco_ids);
    static struct platform_driver applnco_driver = {
    .driver = {
    .name = "apple-nco",
    .of_match_table = applnco_ids,
    },
    .probe = applnco_probe,
    };
    module_platform_driver(applnco_driver);
    MODULE_AUTHOR("Martin Povišer <povik+lin@cutebit.org>");
    MODULE_DESCRIPTION("Clock driver for NCO blocks on Apple SoCs");
    MODULE_LICENSE("GPL");
