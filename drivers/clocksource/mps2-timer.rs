//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/mps2-timer.c
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
// Copyright (C) 2015 ARM Limited
//
// Author: Vladimir Murzin <vladimir.murzin@arm.com>
//

pub const TIMER_CTRL: c_uint = 0x0;

pub const TIMER_VALUE: c_uint = 0x4;
pub const TIMER_RELOAD: c_uint = 0x8;
pub const TIMER_INT: c_uint = 0xc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clockevent_mps2 {
    pub reg: *mut void __iomem,
    pub clock_count_per_tick: u32,
    pub clkevt: clock_event_device,
}

    static void __iomem *sched_clock_base;
#[no_mangle]
unsafe extern "C" fn mps2_sched_read() -> u64 notrace {
    static u64 notrace mps2_sched_read(void)
    {
    return ~readl_relaxed(sched_clock_base + TIMER_VALUE);
    }
    static inline struct clockevent_mps2 *to_mps2_clkevt(struct clock_event_device *c)
    {
    return container_of(c, struct clockevent_mps2, clkevt);
    }
#[no_mangle]
unsafe extern "C" fn clockevent_mps2_writel(val: u32, c: *mut clock_event_device, offset: u32) {
    static void clockevent_mps2_writel(u32 val, struct clock_event_device *c, u32 offset)
    {
    writel_relaxed(val, to_mps2_clkevt(c).reg + offset);
    }
#[no_mangle]
unsafe extern "C" fn mps2_timer_shutdown(ce: *mut clock_event_device) -> c_int {
    static int mps2_timer_shutdown(struct clock_event_device *ce)
    {
    clockevent_mps2_writel(0, ce, TIMER_RELOAD);
    clockevent_mps2_writel(0, ce, TIMER_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mps2_timer_set_next_event(next: c_ulong, ce: *mut clock_event_device) -> c_int {
    static int mps2_timer_set_next_event(unsigned long next, struct clock_event_device *ce)
    {
    clockevent_mps2_writel(next, ce, TIMER_VALUE);
    clockevent_mps2_writel(TIMER_CTRL_IE | TIMER_CTRL_ENABLE, ce, TIMER_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mps2_timer_set_periodic(ce: *mut clock_event_device) -> c_int {
    static int mps2_timer_set_periodic(struct clock_event_device *ce)
    {
    let mut clock_count_per_tick: u32 = to_mps2_clkevt(ce).clock_count_per_tick;
    clockevent_mps2_writel(clock_count_per_tick, ce, TIMER_RELOAD);
    clockevent_mps2_writel(clock_count_per_tick, ce, TIMER_VALUE);
    clockevent_mps2_writel(TIMER_CTRL_IE | TIMER_CTRL_ENABLE, ce, TIMER_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mps2_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mps2_timer_interrupt(int irq, void *dev_id)
    {
    struct clockevent_mps2 *ce = dev_id;
    let mut status: u32 = readl_relaxed(ce.reg + TIMER_INT);
    if (!status) {
    pr_warn("spurious interrupt\n");
    return IRQ_NONE;
    }
    writel_relaxed(1, ce.reg + TIMER_INT);
    ce.clkevt.event_handler(&ce.clkevt);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mps2_clockevent_init(np: *mut device_node) -> int __init {
    static int __init mps2_clockevent_init(struct device_node *np)
    {
    void __iomem *base;
    struct clk *clk = core::ptr::null_mut();
    struct clockevent_mps2 *ce;
    u32 rate;
    int irq, ret;
    const char *name = "mps2-clkevt";
    ret = of_property_read_u32(np, "clock-frequency", &rate);
    if (ret) {
    clk = of_clk_get(np, 0);
    if (IS_ERR(clk)) {
    ret = PTR_ERR(clk);
    pr_err("failed to get clock for clockevent: %d\n", ret);
    goto out;
    }
    ret = clk_prepare_enable(clk);
    if (ret) {
    pr_err("failed to enable clock for clockevent: %d\n", ret);
    goto out_clk_put;
    }
    rate = clk_get_rate(clk);
    }
    base = of_iomap(np, 0);
    if (!base) {
    ret = -EADDRNOTAVAIL;
    pr_err("failed to map register for clockevent: %d\n", ret);
    goto out_clk_disable;
    }
    irq = irq_of_parse_and_map(np, 0);
    if (!irq) {
    ret = -ENOENT;
    pr_err("failed to get irq for clockevent: %d\n", ret);
    goto out_iounmap;
    }
    ce = kzalloc_obj(*ce);
    if (!ce) {
    ret = -ENOMEM;
    goto out_iounmap;
    }
    ce.reg = base;
    ce.clock_count_per_tick = DIV_ROUND_CLOSEST(rate, HZ);
    ce.clkevt.irq = irq;
    ce.clkevt.name = name;
    ce.clkevt.rating = 200;
    ce.clkevt.features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT;
    ce.clkevt.cpumask = cpu_possible_mask;
    ce.clkevt.set_state_shutdown	= mps2_timer_shutdown;
    ce.clkevt.set_state_periodic	= mps2_timer_set_periodic;
    ce.clkevt.set_state_oneshot	= mps2_timer_shutdown;
    ce.clkevt.set_next_event	= mps2_timer_set_next_event;
// Ensure timer is disabled
    writel_relaxed(0, base + TIMER_CTRL);
    ret = request_irq(irq, mps2_timer_interrupt, IRQF_TIMER, name, ce);
    if (ret) {
    pr_err("failed to request irq for clockevent: %d\n", ret);
    goto out_kfree;
    }
    clockevents_config_and_register(&ce.clkevt, rate, 0xf, 0xffffffff);
    return 0;
    out_kfree:
    kfree(ce);
    out_iounmap:
    iounmap(base);
    out_clk_disable:
// clk_{disable, unprepare, put}() can handle NULL as a parameter
    clk_disable_unprepare(clk);
    out_clk_put:
    clk_put(clk);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mps2_clocksource_init(np: *mut device_node) -> int __init {
    static int __init mps2_clocksource_init(struct device_node *np)
    {
    void __iomem *base;
    struct clk *clk = core::ptr::null_mut();
    u32 rate;
    int ret;
    const char *name = "mps2-clksrc";
    ret = of_property_read_u32(np, "clock-frequency", &rate);
    if (ret) {
    clk = of_clk_get(np, 0);
    if (IS_ERR(clk)) {
    ret = PTR_ERR(clk);
    pr_err("failed to get clock for clocksource: %d\n", ret);
    goto out;
    }
    ret = clk_prepare_enable(clk);
    if (ret) {
    pr_err("failed to enable clock for clocksource: %d\n", ret);
    goto out_clk_put;
    }
    rate = clk_get_rate(clk);
    }
    base = of_iomap(np, 0);
    if (!base) {
    ret = -EADDRNOTAVAIL;
    pr_err("failed to map register for clocksource: %d\n", ret);
    goto out_clk_disable;
    }
// Ensure timer is disabled
    writel_relaxed(0, base + TIMER_CTRL);
// ... and set it up as free-running clocksource
    writel_relaxed(0xffffffff, base + TIMER_VALUE);
    writel_relaxed(0xffffffff, base + TIMER_RELOAD);
    writel_relaxed(TIMER_CTRL_ENABLE, base + TIMER_CTRL);
    ret = clocksource_mmio_init(base + TIMER_VALUE, name,
    rate, 200, 32,
    clocksource_mmio_readl_down);
    if (ret) {
    pr_err("failed to init clocksource: %d\n", ret);
    goto out_iounmap;
    }
    sched_clock_base = base;
    sched_clock_register(mps2_sched_read, 32, rate);
    return 0;
    out_iounmap:
    iounmap(base);
    out_clk_disable:
// clk_{disable, unprepare, put}() can handle NULL as a parameter
    clk_disable_unprepare(clk);
    out_clk_put:
    clk_put(clk);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mps2_timer_init(np: *mut device_node) -> int __init {
    static int __init mps2_timer_init(struct device_node *np)
    {
    static int has_clocksource, has_clockevent;
    int ret;
    if (!has_clocksource) {
    ret = mps2_clocksource_init(np);
    if (!ret) {
    has_clocksource = 1;
    return 0;
    }
    }
    if (!has_clockevent) {
    ret = mps2_clockevent_init(np);
    if (!ret) {
    has_clockevent = 1;
    return 0;
    }
    }
    return 0;
    }
    TIMER_OF_DECLARE(mps2_timer, "arm,mps2-timer", mps2_timer_init);
