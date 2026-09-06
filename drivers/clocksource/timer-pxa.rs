//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-pxa.c
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
// arch/arm/mach-pxa/time.c
//
// PXA clocksource, clockevents, and OST interrupt handlers.
// Copyright (c) 2007 by Bill Gatliff <bgat@billgatliff.com>.
//
// Derived from Nicolas Pitre's PXA timer handler Copyright (c) 2001
// by MontaVista Software, Inc.  (Nico, your code rocks!)
//

pub const OSMR0: c_uint = 0x00	/* OS Timer 0 Match Register */;
pub const OSMR1: c_uint = 0x04	/* OS Timer 1 Match Register */;
pub const OSMR2: c_uint = 0x08	/* OS Timer 2 Match Register */;
pub const OSMR3: c_uint = 0x0C	/* OS Timer 3 Match Register */;
pub const OSCR: c_uint = 0x10	/* OS Timer Counter Register */;
pub const OSSR: c_uint = 0x14	/* OS Timer Status Register */;
pub const OWER: c_uint = 0x18	/* OS Timer Watchdog Enable Register */;
pub const OIER: c_uint = 0x1C	/* OS Timer Interrupt Enable Register */;

//
// This is PXA's sched_clock implementation. This has a resolution
// of at least 308 ns and a maximum value of 208 days.
//
// The return value is guaranteed to be monotonic in that range as
// long as there is always less than 582 seconds between successive
// calls to sched_clock() which should always be the case in practice.
//

    static void __iomem *timer_base;
#[no_mangle]
unsafe extern "C" fn pxa_read_sched_clock() -> u64 notrace {
    static u64 notrace pxa_read_sched_clock(void)
    {
    return timer_readl(OSCR);
    }
pub const MIN_OSCR_DELTA: c_int = 16;
    static irqreturn_t
    pxa_ost0_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *c = dev_id;
// Disarm the compare/match, signal the event.
    timer_writel(timer_readl(OIER) & ~OIER_E0, OIER);
    timer_writel(OSSR_M0, OSSR);
    c.event_handler(c);
    return IRQ_HANDLED;
    }
    static int
    pxa_osmr0_set_next_event(unsigned long delta, struct clock_event_device *dev)
    {
    unsigned long next, oscr;
    timer_writel(timer_readl(OIER) | OIER_E0, OIER);
    next = timer_readl(OSCR) + delta;
    timer_writel(next, OSMR0);
    oscr = timer_readl(OSCR);
    return (signed)(next - oscr) <= MIN_OSCR_DELTA ? -ETIME : 0;
    }
#[no_mangle]
unsafe extern "C" fn pxa_osmr0_shutdown(evt: *mut clock_event_device) -> c_int {
    static int pxa_osmr0_shutdown(struct clock_event_device *evt)
    {
// initializing, released, or preparing for suspend
    timer_writel(timer_readl(OIER) & ~OIER_E0, OIER);
    timer_writel(OSSR_M0, OSSR);
    return 0;
    }

    static unsigned long osmr[4], oier, oscr;
#[no_mangle]
unsafe extern "C" fn pxa_timer_suspend(cedev: *mut clock_event_device) {
    static void pxa_timer_suspend(struct clock_event_device *cedev)
    {
    osmr[0] = timer_readl(OSMR0);
    osmr[1] = timer_readl(OSMR1);
    osmr[2] = timer_readl(OSMR2);
    osmr[3] = timer_readl(OSMR3);
    oier = timer_readl(OIER);
    oscr = timer_readl(OSCR);
    }
#[no_mangle]
unsafe extern "C" fn pxa_timer_resume(cedev: *mut clock_event_device) {
    static void pxa_timer_resume(struct clock_event_device *cedev)
    {
//
// Ensure that we have at least MIN_OSCR_DELTA between match
// register 0 and the OSCR, to guarantee that we will receive
// the one-shot timer interrupt.  We adjust OSMR0 in preference
// to OSCR to guarantee that OSCR is monotonically incrementing.
//
    if (osmr[0] - oscr < MIN_OSCR_DELTA)
    osmr[0] += MIN_OSCR_DELTA;
    timer_writel(osmr[0], OSMR0);
    timer_writel(osmr[1], OSMR1);
    timer_writel(osmr[2], OSMR2);
    timer_writel(osmr[3], OSMR3);
    timer_writel(oier, OIER);
    timer_writel(oscr, OSCR);
    }

    static struct clock_event_device ckevt_pxa_osmr0 = {
    .name			= "osmr0",
    .features		= CLOCK_EVT_FEAT_ONESHOT,
    .rating			= 200,
    .set_next_event		= pxa_osmr0_set_next_event,
    .set_state_shutdown	= pxa_osmr0_shutdown,
    .set_state_oneshot	= pxa_osmr0_shutdown,
    .suspend		= pxa_timer_suspend,
    .resume			= pxa_timer_resume,
    };
#[no_mangle]
unsafe extern "C" fn pxa_timer_common_init(irq: c_int, clock_tick_rate: c_ulong) -> int __init {
    static int __init pxa_timer_common_init(int irq, unsigned long clock_tick_rate)
    {
    int ret;
    timer_writel(0, OIER);
    timer_writel(OSSR_M0 | OSSR_M1 | OSSR_M2 | OSSR_M3, OSSR);
    sched_clock_register(pxa_read_sched_clock, 32, clock_tick_rate);
    ckevt_pxa_osmr0.cpumask = cpumask_of(0);
    ret = request_irq(irq, pxa_ost0_interrupt, IRQF_TIMER | IRQF_IRQPOLL,
    "ost0", &ckevt_pxa_osmr0);
    if (ret) {
    pr_err("Failed to setup irq\n");
    return ret;
    }
    ret = clocksource_mmio_init(timer_base + OSCR, "oscr0", clock_tick_rate, 200,
    32, clocksource_mmio_readl_up);
    if (ret) {
    pr_err("Failed to init clocksource\n");
    return ret;
    }
    clockevents_config_and_register(&ckevt_pxa_osmr0, clock_tick_rate,
    MIN_OSCR_DELTA * 2, 0x7fffffff);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pxa_timer_dt_init(np: *mut device_node) -> int __init {
    static int __init pxa_timer_dt_init(struct device_node *np)
    {
    struct clk *clk;
    int irq, ret;
// timer registers are shared with watchdog timer
    timer_base = of_iomap(np, 0);
    if (!timer_base) {
    pr_err("%pOFn: unable to map resource\n", np);
    return -ENXIO;
    }
    clk = of_clk_get(np, 0);
    if (IS_ERR(clk)) {
    pr_crit("%pOFn: unable to get clk\n", np);
    return PTR_ERR(clk);
    }
    ret = clk_prepare_enable(clk);
    if (ret) {
    pr_crit("Failed to prepare clock\n");
    return ret;
    }
// we are only interested in OS-timer0 irq
    irq = irq_of_parse_and_map(np, 0);
    if (irq <= 0) {
    pr_crit("%pOFn: unable to parse OS-timer0 irq\n", np);
    return -EINVAL;
    }
    return pxa_timer_common_init(irq, clk_get_rate(clk));
    }
    TIMER_OF_DECLARE(pxa_timer, "marvell,pxa-timer", pxa_timer_dt_init);
//
// Legacy timer init for non device-tree boards.
//
#[no_mangle]
pub unsafe extern "C" fn pxa_timer_nodt_init(irq: c_int, base: *mut void __iomem) -> void __init {
    void __init pxa_timer_nodt_init(int irq, void __iomem *base)
    {
    struct clk *clk;
    timer_base = base;
    clk = clk_get(core::ptr::null_mut(), "OSTIMER0");
    if (clk && !IS_ERR(clk)) {
    clk_prepare_enable(clk);
    pxa_timer_common_init(irq, clk_get_rate(clk));
    } else {
    pr_crit("%s: unable to get clk\n", __func__);
    }
    }
