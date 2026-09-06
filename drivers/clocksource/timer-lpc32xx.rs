//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-lpc32xx.c
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
// Clocksource driver for NXP LPC32xx/18xx/43xx timer
//
// Copyright (C) 2015 Joachim Eastwood <manabian@gmail.com>
//
// Based on:
// time-efm32 Copyright (C) 2013 Pengutronix
// mach-lpc32xx/timer.c Copyright (C) 2009 - 2010 NXP Semiconductors
//

pub const LPC32XX_TIMER_IR: c_uint = 0x000;

pub const LPC32XX_TIMER_TCR: c_uint = 0x004;

pub const LPC32XX_TIMER_TC: c_uint = 0x008;
pub const LPC32XX_TIMER_PR: c_uint = 0x00c;
pub const LPC32XX_TIMER_MCR: c_uint = 0x014;

pub const LPC32XX_TIMER_MR0: c_uint = 0x018;
pub const LPC32XX_TIMER_CTCR: c_uint = 0x070;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc32xx_clock_event_ddata {
    pub evtdev: clock_event_device,
    pub base: *mut void __iomem,
    pub ticks_per_jiffy: u32,
}

// Needed for the sched clock
    static void __iomem *clocksource_timer_counter;
#[no_mangle]
unsafe extern "C" fn lpc32xx_read_sched_clock() -> u64 notrace {
    static u64 notrace lpc32xx_read_sched_clock(void)
    {
    return readl(clocksource_timer_counter);
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_delay_timer_read() -> c_ulong {
    static unsigned long lpc32xx_delay_timer_read(void)
    {
    return readl(clocksource_timer_counter);
    }
    static struct delay_timer lpc32xx_delay_timer = {
    .read_current_timer = lpc32xx_delay_timer_read,
    };
    static int lpc32xx_clkevt_next_event(unsigned long delta,
    struct clock_event_device *evtdev)
    {
    struct lpc32xx_clock_event_ddata *ddata =
    container_of(evtdev, struct lpc32xx_clock_event_ddata, evtdev);
//
// Place timer in reset and program the delta in the match
// channel 0 (MR0). When the timer counter matches the value
// in MR0 register the match will trigger an interrupt.
// After setup the timer is released from reset and enabled.
//
    writel_relaxed(LPC32XX_TIMER_TCR_CRST, ddata.base + LPC32XX_TIMER_TCR);
    writel_relaxed(delta, ddata.base + LPC32XX_TIMER_MR0);
    writel_relaxed(LPC32XX_TIMER_TCR_CEN, ddata.base + LPC32XX_TIMER_TCR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_clkevt_shutdown(evtdev: *mut clock_event_device) -> c_int {
    static int lpc32xx_clkevt_shutdown(struct clock_event_device *evtdev)
    {
    struct lpc32xx_clock_event_ddata *ddata =
    container_of(evtdev, struct lpc32xx_clock_event_ddata, evtdev);
// Disable the timer
    writel_relaxed(0, ddata.base + LPC32XX_TIMER_TCR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_clkevt_oneshot(evtdev: *mut clock_event_device) -> c_int {
    static int lpc32xx_clkevt_oneshot(struct clock_event_device *evtdev)
    {
    struct lpc32xx_clock_event_ddata *ddata =
    container_of(evtdev, struct lpc32xx_clock_event_ddata, evtdev);
//
// When using oneshot, we must also disable the timer
// to wait for the first call to set_next_event().
//
    writel_relaxed(0, ddata.base + LPC32XX_TIMER_TCR);
// Enable interrupt, reset on match and stop on match (MCR).
    writel_relaxed(LPC32XX_TIMER_MCR_MR0I | LPC32XX_TIMER_MCR_MR0R |
    LPC32XX_TIMER_MCR_MR0S, ddata.base + LPC32XX_TIMER_MCR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_clkevt_periodic(evtdev: *mut clock_event_device) -> c_int {
    static int lpc32xx_clkevt_periodic(struct clock_event_device *evtdev)
    {
    struct lpc32xx_clock_event_ddata *ddata =
    container_of(evtdev, struct lpc32xx_clock_event_ddata, evtdev);
// Enable interrupt and reset on match.
    writel_relaxed(LPC32XX_TIMER_MCR_MR0I | LPC32XX_TIMER_MCR_MR0R,
    ddata.base + LPC32XX_TIMER_MCR);
//
// Place timer in reset and program the delta in the match
// channel 0 (MR0).
//
    writel_relaxed(LPC32XX_TIMER_TCR_CRST, ddata.base + LPC32XX_TIMER_TCR);
    writel_relaxed(ddata.ticks_per_jiffy, ddata.base + LPC32XX_TIMER_MR0);
    writel_relaxed(LPC32XX_TIMER_TCR_CEN, ddata.base + LPC32XX_TIMER_TCR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_clock_event_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t lpc32xx_clock_event_handler(int irq, void *dev_id)
    {
    struct lpc32xx_clock_event_ddata *ddata = dev_id;
// Clear match on channel 0
    writel_relaxed(LPC32XX_TIMER_IR_MR0INT, ddata.base + LPC32XX_TIMER_IR);
    ddata.evtdev.event_handler(&ddata.evtdev);
    return IRQ_HANDLED;
    }
    static struct lpc32xx_clock_event_ddata lpc32xx_clk_event_ddata = {
    .evtdev = {
    .name			= "lpc3220 clockevent",
    .features		= CLOCK_EVT_FEAT_ONESHOT |
    CLOCK_EVT_FEAT_PERIODIC,
    .rating			= 300,
    .set_next_event		= lpc32xx_clkevt_next_event,
    .set_state_shutdown	= lpc32xx_clkevt_shutdown,
    .set_state_oneshot	= lpc32xx_clkevt_oneshot,
    .set_state_periodic	= lpc32xx_clkevt_periodic,
    },
    };
#[no_mangle]
unsafe extern "C" fn lpc32xx_clocksource_init(np: *mut device_node) -> int __init {
    static int __init lpc32xx_clocksource_init(struct device_node *np)
    {
    void __iomem *base;
    unsigned long rate;
    struct clk *clk;
    int ret;
    clk = of_clk_get_by_name(np, "timerclk");
    if (IS_ERR(clk)) {
    pr_err("clock get failed (%ld)\n", PTR_ERR(clk));
    return PTR_ERR(clk);
    }
    ret = clk_prepare_enable(clk);
    if (ret) {
    pr_err("clock enable failed (%d)\n", ret);
    goto err_clk_enable;
    }
    base = of_iomap(np, 0);
    if (!base) {
    pr_err("unable to map registers\n");
    ret = -EADDRNOTAVAIL;
    goto err_iomap;
    }
//
// Disable and reset timer then set it to free running timer
// mode (CTCR) with no prescaler (PR) or match operations (MCR).
// After setup the timer is released from reset and enabled.
//
    writel_relaxed(LPC32XX_TIMER_TCR_CRST, base + LPC32XX_TIMER_TCR);
    writel_relaxed(0, base + LPC32XX_TIMER_PR);
    writel_relaxed(0, base + LPC32XX_TIMER_MCR);
    writel_relaxed(0, base + LPC32XX_TIMER_CTCR);
    writel_relaxed(LPC32XX_TIMER_TCR_CEN, base + LPC32XX_TIMER_TCR);
    rate = clk_get_rate(clk);
    ret = clocksource_mmio_init(base + LPC32XX_TIMER_TC, "lpc3220 timer",
    rate, 300, 32, clocksource_mmio_readl_up);
    if (ret) {
    pr_err("failed to init clocksource (%d)\n", ret);
    goto err_clocksource_init;
    }
    clocksource_timer_counter = base + LPC32XX_TIMER_TC;
    lpc32xx_delay_timer.freq = rate;
    register_current_timer_delay(&lpc32xx_delay_timer);
    sched_clock_register(lpc32xx_read_sched_clock, 32, rate);
    return 0;
    err_clocksource_init:
    iounmap(base);
    err_iomap:
    clk_disable_unprepare(clk);
    err_clk_enable:
    clk_put(clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lpc32xx_clockevent_init(np: *mut device_node) -> int __init {
    static int __init lpc32xx_clockevent_init(struct device_node *np)
    {
    void __iomem *base;
    unsigned long rate;
    struct clk *clk;
    int ret, irq;
    clk = of_clk_get_by_name(np, "timerclk");
    if (IS_ERR(clk)) {
    pr_err("clock get failed (%ld)\n", PTR_ERR(clk));
    return PTR_ERR(clk);
    }
    ret = clk_prepare_enable(clk);
    if (ret) {
    pr_err("clock enable failed (%d)\n", ret);
    goto err_clk_enable;
    }
    base = of_iomap(np, 0);
    if (!base) {
    pr_err("unable to map registers\n");
    ret = -EADDRNOTAVAIL;
    goto err_iomap;
    }
    irq = irq_of_parse_and_map(np, 0);
    if (!irq) {
    pr_err("get irq failed\n");
    ret = -ENOENT;
    goto err_irq;
    }
//
// Disable timer and clear any pending interrupt (IR) on match
// channel 0 (MR0). Clear the prescaler as it's not used.
//
    writel_relaxed(0, base + LPC32XX_TIMER_TCR);
    writel_relaxed(0, base + LPC32XX_TIMER_PR);
    writel_relaxed(0, base + LPC32XX_TIMER_CTCR);
    writel_relaxed(LPC32XX_TIMER_IR_MR0INT, base + LPC32XX_TIMER_IR);
    rate = clk_get_rate(clk);
    lpc32xx_clk_event_ddata.base = base;
    lpc32xx_clk_event_ddata.ticks_per_jiffy = DIV_ROUND_CLOSEST(rate, HZ);
    clockevents_config_and_register(&lpc32xx_clk_event_ddata.evtdev,
    rate, 1, -1);
    ret = request_irq(irq, lpc32xx_clock_event_handler,
    IRQF_TIMER | IRQF_IRQPOLL, "lpc3220 clockevent",
    &lpc32xx_clk_event_ddata);
    if (ret) {
    pr_err("request irq failed\n");
    goto err_irq;
    }
    return 0;
    err_irq:
    iounmap(base);
    err_iomap:
    clk_disable_unprepare(clk);
    err_clk_enable:
    clk_put(clk);
    return ret;
    }
//
// This function asserts that we have exactly one clocksource and one
// clock_event_device in the end.
//
#[no_mangle]
unsafe extern "C" fn lpc32xx_timer_init(np: *mut device_node) -> int __init {
    static int __init lpc32xx_timer_init(struct device_node *np)
    {
    static int has_clocksource, has_clockevent;
    let mut ret: c_int = 0;
    if (!has_clocksource) {
    ret = lpc32xx_clocksource_init(np);
    if (!ret) {
    has_clocksource = 1;
    return 0;
    }
    }
    if (!has_clockevent) {
    ret = lpc32xx_clockevent_init(np);
    if (!ret) {
    has_clockevent = 1;
    return 0;
    }
    }
    return ret;
    }
    TIMER_OF_DECLARE(lpc32xx_timer, "nxp,lpc3220-timer", lpc32xx_timer_init);
