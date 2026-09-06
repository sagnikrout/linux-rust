//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-owl.c
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
// Actions Semi Owl timer
//
// Copyright 2012 Actions Semi Inc.
// Author: Actions Semi, Inc.
//
// Copyright (c) 2017 SUSE Linux GmbH
// Author: Andreas Färber
//

pub const OWL_Tx_CTL: c_uint = 0x0;
pub const OWL_Tx_CMP: c_uint = 0x4;
pub const OWL_Tx_VAL: c_uint = 0x8;

    static void __iomem *owl_timer_base;
    static void __iomem *owl_clksrc_base;
    static void __iomem *owl_clkevt_base;
#[no_mangle]
pub unsafe extern "C" fn owl_timer_reset(base: *mut void __iomem) {
    static inline void owl_timer_reset(void __iomem *base)
    {
    writel(0, base + OWL_Tx_CTL);
    writel(0, base + OWL_Tx_VAL);
    writel(0, base + OWL_Tx_CMP);
    }
#[no_mangle]
pub unsafe extern "C" fn owl_timer_set_enabled(base: *mut void __iomem, enabled: bool) {
    static inline void owl_timer_set_enabled(void __iomem *base, bool enabled)
    {
    let mut ctl: u32 = readl(base + OWL_Tx_CTL);
// PD bit is cleared when set
    ctl &= ~OWL_Tx_CTL_PD;
    if (enabled)
    ctl |= OWL_Tx_CTL_EN;
    else
    ctl &= ~OWL_Tx_CTL_EN;
    writel(ctl, base + OWL_Tx_CTL);
    }
#[no_mangle]
unsafe extern "C" fn owl_timer_sched_read() -> u64 notrace {
    static u64 notrace owl_timer_sched_read(void)
    {
    return (u64)readl(owl_clksrc_base + OWL_Tx_VAL);
    }
#[no_mangle]
unsafe extern "C" fn owl_timer_set_state_shutdown(evt: *mut clock_event_device) -> c_int {
    static int owl_timer_set_state_shutdown(struct clock_event_device *evt)
    {
    owl_timer_set_enabled(owl_clkevt_base, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn owl_timer_set_state_oneshot(evt: *mut clock_event_device) -> c_int {
    static int owl_timer_set_state_oneshot(struct clock_event_device *evt)
    {
    owl_timer_reset(owl_clkevt_base);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn owl_timer_tick_resume(evt: *mut clock_event_device) -> c_int {
    static int owl_timer_tick_resume(struct clock_event_device *evt)
    {
    return 0;
    }
    static int owl_timer_set_next_event(unsigned long evt,
    struct clock_event_device *ev)
    {
    void __iomem *base = owl_clkevt_base;
    owl_timer_set_enabled(base, false);
    writel(OWL_Tx_CTL_INTEN, base + OWL_Tx_CTL);
    writel(0, base + OWL_Tx_VAL);
    writel(evt, base + OWL_Tx_CMP);
    owl_timer_set_enabled(base, true);
    return 0;
    }
    static struct clock_event_device owl_clockevent = {
    .name			= "owl_tick",
    .rating			= 200,
    .features		= CLOCK_EVT_FEAT_ONESHOT |
    CLOCK_EVT_FEAT_DYNIRQ,
    .set_state_shutdown	= owl_timer_set_state_shutdown,
    .set_state_oneshot	= owl_timer_set_state_oneshot,
    .tick_resume		= owl_timer_tick_resume,
    .set_next_event		= owl_timer_set_next_event,
    };
#[no_mangle]
unsafe extern "C" fn owl_timer1_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t owl_timer1_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *evt = (struct clock_event_device *)dev_id;
    writel(OWL_Tx_CTL_PD, owl_clkevt_base + OWL_Tx_CTL);
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn owl_timer_init(node: *mut device_node) -> int __init {
    static int __init owl_timer_init(struct device_node *node)
    {
    struct clk *clk;
    unsigned long rate;
    int timer1_irq, ret;
    owl_timer_base = of_io_request_and_map(node, 0, "owl-timer");
    if (IS_ERR(owl_timer_base)) {
    pr_err("Can't map timer registers\n");
    return PTR_ERR(owl_timer_base);
    }
    owl_clksrc_base = owl_timer_base + 0x08;
    owl_clkevt_base = owl_timer_base + 0x14;
    timer1_irq = of_irq_get_byname(node, "timer1");
    if (timer1_irq <= 0) {
    pr_err("Can't parse timer1 IRQ\n");
    return -EINVAL;
    }
    clk = of_clk_get(node, 0);
    if (IS_ERR(clk)) {
    ret = PTR_ERR(clk);
    pr_err("Failed to get clock for clocksource (%d)\n", ret);
    return ret;
    }
    rate = clk_get_rate(clk);
    owl_timer_reset(owl_clksrc_base);
    owl_timer_set_enabled(owl_clksrc_base, true);
    sched_clock_register(owl_timer_sched_read, 32, rate);
    ret = clocksource_mmio_init(owl_clksrc_base + OWL_Tx_VAL, node.name,
    rate, 200, 32, clocksource_mmio_readl_up);
    if (ret) {
    pr_err("Failed to register clocksource (%d)\n", ret);
    return ret;
    }
    owl_timer_reset(owl_clkevt_base);
    ret = request_irq(timer1_irq, owl_timer1_interrupt, IRQF_TIMER,
    "owl-timer", &owl_clockevent);
    if (ret) {
    pr_err("failed to request irq %d\n", timer1_irq);
    return ret;
    }
    owl_clockevent.cpumask = cpumask_of(0);
    owl_clockevent.irq = timer1_irq;
    clockevents_config_and_register(&owl_clockevent, rate,
    0xf, 0xffffffff);
    return 0;
    }
    TIMER_OF_DECLARE(owl_s500, "actions,s500-timer", owl_timer_init);
    TIMER_OF_DECLARE(owl_s700, "actions,s700-timer", owl_timer_init);
    TIMER_OF_DECLARE(owl_s900, "actions,s900-timer", owl_timer_init);
