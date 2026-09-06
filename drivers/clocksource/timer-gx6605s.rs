//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-gx6605s.c
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
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

pub const CLKSRC_OFFSET: c_uint = 0x40;
pub const TIMER_STATUS: c_uint = 0x00;
pub const TIMER_VALUE: c_uint = 0x04;
pub const TIMER_CONTRL: c_uint = 0x10;
pub const TIMER_CONFIG: c_uint = 0x20;
pub const TIMER_DIV: c_uint = 0x24;
pub const TIMER_INI: c_uint = 0x28;

#[no_mangle]
unsafe extern "C" fn gx6605s_timer_interrupt(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t gx6605s_timer_interrupt(int irq, void *dev)
    {
    struct clock_event_device *ce = dev;
    void __iomem *base = timer_of_base(to_timer_of(ce));
    writel_relaxed(GX6605S_STATUS_CLR, base + TIMER_STATUS);
    writel_relaxed(0, base + TIMER_INI);
    ce.event_handler(ce);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn gx6605s_timer_set_oneshot(ce: *mut clock_event_device) -> c_int {
    static int gx6605s_timer_set_oneshot(struct clock_event_device *ce)
    {
    void __iomem *base = timer_of_base(to_timer_of(ce));
// reset and stop counter
    writel_relaxed(GX6605S_CONTRL_RST, base + TIMER_CONTRL);
// enable with irq and start
    writel_relaxed(GX6605S_CONFIG_EN | GX6605S_CONFIG_IRQ_EN,
    base + TIMER_CONFIG);
    return 0;
    }
    static int gx6605s_timer_set_next_event(unsigned long delta,
    struct clock_event_device *ce)
    {
    void __iomem *base = timer_of_base(to_timer_of(ce));
// use reset to pause timer
    writel_relaxed(GX6605S_CONTRL_RST, base + TIMER_CONTRL);
// config next timeout value
    writel_relaxed(ULONG_MAX - delta, base + TIMER_INI);
    writel_relaxed(GX6605S_CONTRL_START, base + TIMER_CONTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gx6605s_timer_shutdown(ce: *mut clock_event_device) -> c_int {
    static int gx6605s_timer_shutdown(struct clock_event_device *ce)
    {
    void __iomem *base = timer_of_base(to_timer_of(ce));
    writel_relaxed(0, base + TIMER_CONTRL);
    writel_relaxed(0, base + TIMER_CONFIG);
    return 0;
    }
    static struct timer_of to = {
    .flags = TIMER_OF_IRQ | TIMER_OF_BASE | TIMER_OF_CLOCK,
    .clkevt = {
    .rating			= 300,
    .features		= CLOCK_EVT_FEAT_DYNIRQ |
    CLOCK_EVT_FEAT_ONESHOT,
    .set_state_shutdown	= gx6605s_timer_shutdown,
    .set_state_oneshot	= gx6605s_timer_set_oneshot,
    .set_next_event		= gx6605s_timer_set_next_event,
    .cpumask		= cpu_possible_mask,
    },
    .of_irq = {
    .handler		= gx6605s_timer_interrupt,
    .flags			= IRQF_TIMER | IRQF_IRQPOLL,
    },
    };
#[no_mangle]
unsafe extern "C" fn gx6605s_sched_clock_read() -> u64 notrace {
    static u64 notrace gx6605s_sched_clock_read(void)
    {
    void __iomem *base;
    base = timer_of_base(&to) + CLKSRC_OFFSET;
    return (u64)readl_relaxed(base + TIMER_VALUE);
    }
#[no_mangle]
unsafe extern "C" fn gx6605s_clkevt_init(base: *mut void __iomem) {
    static void gx6605s_clkevt_init(void __iomem *base)
    {
    writel_relaxed(0, base + TIMER_DIV);
    writel_relaxed(0, base + TIMER_CONFIG);
    clockevents_config_and_register(&to.clkevt, timer_of_rate(&to), 2,
    ULONG_MAX);
    }
#[no_mangle]
unsafe extern "C" fn gx6605s_clksrc_init(base: *mut void __iomem) -> c_int {
    static int gx6605s_clksrc_init(void __iomem *base)
    {
    writel_relaxed(0, base + TIMER_DIV);
    writel_relaxed(0, base + TIMER_INI);
    writel_relaxed(GX6605S_CONTRL_RST, base + TIMER_CONTRL);
    writel_relaxed(GX6605S_CONFIG_EN, base + TIMER_CONFIG);
    writel_relaxed(GX6605S_CONTRL_START, base + TIMER_CONTRL);
    sched_clock_register(gx6605s_sched_clock_read, 32, timer_of_rate(&to));
    return clocksource_mmio_init(base + TIMER_VALUE, "gx6605s",
    timer_of_rate(&to), 200, 32, clocksource_mmio_readl_up);
    }
#[no_mangle]
unsafe extern "C" fn gx6605s_timer_init(np: *mut device_node) -> int __init {
    static int __init gx6605s_timer_init(struct device_node *np)
    {
    int ret;
//
// The timer driver is for nationalchip gx6605s SOC and there are two
// same timer in gx6605s. We use one for clkevt and another for clksrc.
//
// The timer is mmio map to access, so we need give mmio address in dts.
//
// It provides a 32bit countup timer and interrupt will be caused by
// count-overflow.
// So we need set-next-event by ULONG_MAX - delta in TIMER_INI reg.
//
// The counter at 0x0  offset is clock event.
// The counter at 0x40 offset is clock source.
// They are the same in hardware, just different used by driver.
//
    ret = timer_of_init(np, &to);
    if (ret)
    return ret;
    gx6605s_clkevt_init(timer_of_base(&to));
    return gx6605s_clksrc_init(timer_of_base(&to) + CLKSRC_OFFSET);
    }
    TIMER_OF_DECLARE(csky_gx6605s_timer, "csky,gx6605s-timer", gx6605s_timer_init);
