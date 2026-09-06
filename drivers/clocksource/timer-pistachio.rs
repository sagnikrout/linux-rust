//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-pistachio.c
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
//
// Pistachio clocksource based on general-purpose timers
//
// Copyright (C) 2015 Imagination Technologies
//

// Top level reg
pub const CR_TIMER_CTRL_CFG: c_uint = 0x00;

pub const CR_TIMER_REV: c_uint = 0x10;
// Timer specific registers
pub const TIMER_CFG: c_uint = 0x20;

pub const TIMER_RELOAD_VALUE: c_uint = 0x24;
pub const TIMER_CURRENT_VALUE: c_uint = 0x28;
pub const TIMER_CURRENT_OVERFLOW_VALUE: c_uint = 0x2C;
pub const TIMER_IRQ_STATUS: c_uint = 0x30;
pub const TIMER_IRQ_CLEAR: c_uint = 0x34;
pub const TIMER_IRQ_MASK: c_uint = 0x38;
pub const PERIP_TIMER_CONTROL: c_uint = 0x90;
// Timer specific configuration Values
pub const RELOAD_VALUE: c_uint = 0xffffffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pistachio_clocksource {
    pub base: *mut void __iomem,
    pub lock: raw_spinlock_t,
    pub cs: clocksource,
}

    static struct pistachio_clocksource pcs_gpt;

    container_of(cs, struct pistachio_clocksource, cs)
#[no_mangle]
pub unsafe extern "C" fn gpt_readl(base: *mut void __iomem, offset: u32, gpt_id: u32) -> u32 {
    static inline u32 gpt_readl(void __iomem *base, u32 offset, u32 gpt_id)
    {
    return readl(base + 0x20 * gpt_id + offset);
    }
    static inline void gpt_writel(void __iomem *base, u32 value, u32 offset,
    u32 gpt_id)
    {
    writel(value, base + 0x20 * gpt_id + offset);
    }
    static u64 notrace
    pistachio_clocksource_read_cycles(struct clocksource *cs)
    {
    struct pistachio_clocksource *pcs = to_pistachio_clocksource(cs);
    __maybe_unused u32 overflow;
    u32 counter;
    unsigned long flags;
//
// The counter value is only refreshed after the overflow value is read.
// And they must be read in strict order, hence raw spin lock added.
//
    raw_spin_lock_irqsave(&pcs.lock, flags);
    overflow = gpt_readl(pcs.base, TIMER_CURRENT_OVERFLOW_VALUE, 0);
    counter = gpt_readl(pcs.base, TIMER_CURRENT_VALUE, 0);
    raw_spin_unlock_irqrestore(&pcs.lock, flags);
    return (u64)~counter;
    }
#[no_mangle]
unsafe extern "C" fn pistachio_read_sched_clock() -> u64 notrace {
    static u64 notrace pistachio_read_sched_clock(void)
    {
    return pistachio_clocksource_read_cycles(&pcs_gpt.cs);
    }
    static void pistachio_clksrc_set_mode(struct clocksource *cs, int timeridx,
    int enable)
    {
    struct pistachio_clocksource *pcs = to_pistachio_clocksource(cs);
    u32 val;
    val = gpt_readl(pcs.base, TIMER_CFG, timeridx);
    if (enable)
    val |= TIMER_ME_LOCAL;
    else
    val &= ~TIMER_ME_LOCAL;
    gpt_writel(pcs.base, val, TIMER_CFG, timeridx);
    }
#[no_mangle]
unsafe extern "C" fn pistachio_clksrc_enable(cs: *mut clocksource, timeridx: c_int) {
    static void pistachio_clksrc_enable(struct clocksource *cs, int timeridx)
    {
    struct pistachio_clocksource *pcs = to_pistachio_clocksource(cs);
// Disable GPT local before loading reload value
    pistachio_clksrc_set_mode(cs, timeridx, false);
    gpt_writel(pcs.base, RELOAD_VALUE, TIMER_RELOAD_VALUE, timeridx);
    pistachio_clksrc_set_mode(cs, timeridx, true);
    }
#[no_mangle]
unsafe extern "C" fn pistachio_clksrc_disable(cs: *mut clocksource, timeridx: c_int) {
    static void pistachio_clksrc_disable(struct clocksource *cs, int timeridx)
    {
// Disable GPT local
    pistachio_clksrc_set_mode(cs, timeridx, false);
    }
#[no_mangle]
unsafe extern "C" fn pistachio_clocksource_enable(cs: *mut clocksource) -> c_int {
    static int pistachio_clocksource_enable(struct clocksource *cs)
    {
    pistachio_clksrc_enable(cs, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pistachio_clocksource_disable(cs: *mut clocksource) {
    static void pistachio_clocksource_disable(struct clocksource *cs)
    {
    pistachio_clksrc_disable(cs, 0);
    }
// Desirable clock source for pistachio platform
    static struct pistachio_clocksource pcs_gpt = {
    .cs =	{
    .name		= "gptimer",
    .rating		= 300,
    .enable		= pistachio_clocksource_enable,
    .disable	= pistachio_clocksource_disable,
    .read		= pistachio_clocksource_read_cycles,
    .mask		= CLOCKSOURCE_MASK(32),
    .flags		= CLOCK_SOURCE_IS_CONTINUOUS |
    CLOCK_SOURCE_SUSPEND_NONSTOP,
    },
    };
#[no_mangle]
unsafe extern "C" fn pistachio_clksrc_of_init(node: *mut device_node) -> int __init {
    static int __init pistachio_clksrc_of_init(struct device_node *node)
    {
    struct clk *sys_clk, *fast_clk;
    struct regmap *periph_regs;
    unsigned long rate;
    int ret;
    pcs_gpt.base = of_iomap(node, 0);
    if (!pcs_gpt.base) {
    pr_err("cannot iomap\n");
    return -ENXIO;
    }
    periph_regs = syscon_regmap_lookup_by_phandle(node, "img,cr-periph");
    if (IS_ERR(periph_regs)) {
    pr_err("cannot get peripheral regmap (%ld)\n",
    PTR_ERR(periph_regs));
    return PTR_ERR(periph_regs);
    }
// Switch to using the fast counter clock
    ret = regmap_update_bits(periph_regs, PERIP_TIMER_CONTROL,
    0xf, 0x0);
    if (ret)
    return ret;
    sys_clk = of_clk_get_by_name(node, "sys");
    if (IS_ERR(sys_clk)) {
    pr_err("clock get failed (%ld)\n", PTR_ERR(sys_clk));
    return PTR_ERR(sys_clk);
    }
    fast_clk = of_clk_get_by_name(node, "fast");
    if (IS_ERR(fast_clk)) {
    pr_err("clock get failed (%lu)\n", PTR_ERR(fast_clk));
    return PTR_ERR(fast_clk);
    }
    ret = clk_prepare_enable(sys_clk);
    if (ret < 0) {
    pr_err("failed to enable clock (%d)\n", ret);
    return ret;
    }
    ret = clk_prepare_enable(fast_clk);
    if (ret < 0) {
    pr_err("failed to enable clock (%d)\n", ret);
    clk_disable_unprepare(sys_clk);
    return ret;
    }
    rate = clk_get_rate(fast_clk);
// Disable irq's for clocksource usage
    gpt_writel(pcs_gpt.base, 0, TIMER_IRQ_MASK, 0);
    gpt_writel(pcs_gpt.base, 0, TIMER_IRQ_MASK, 1);
    gpt_writel(pcs_gpt.base, 0, TIMER_IRQ_MASK, 2);
    gpt_writel(pcs_gpt.base, 0, TIMER_IRQ_MASK, 3);
// Enable timer block
    writel(TIMER_ME_GLOBAL, pcs_gpt.base);
    raw_spin_lock_init(&pcs_gpt.lock);
    sched_clock_register(pistachio_read_sched_clock, 32, rate);
    return clocksource_register_hz(&pcs_gpt.cs, rate);
    }
    TIMER_OF_DECLARE(pistachio_gptimer, "img,pistachio-gptimer",
    pistachio_clksrc_of_init);
