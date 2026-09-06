//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-digicolor.c
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
// Conexant Digicolor timer driver
//
// Author: Baruch Siach <baruch@tkos.co.il>
//
// Copyright (C) 2014 Paradox Innovation Ltd.
//
// Based on:
// Allwinner SoCs hstimer driver
//
// Copyright (C) 2013 Maxime Ripard
//
// Maxime Ripard <maxime.ripard@free-electrons.com>
//
// Conexant Digicolor SoCs have 8 configurable timers, named from "Timer A" to
// "Timer H". Timer A is the only one with watchdog support, so it is dedicated
// to the watchdog driver. This driver uses Timer B for sched_clock(), and
// Timer C for clockevents.
//

    enum {
    TIMER_A,
    TIMER_B,
    TIMER_C,
    TIMER_D,
    TIMER_E,
    TIMER_F,
    TIMER_G,
    TIMER_H,
    };

pub const CONTROL_DISABLE: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct digicolor_timer {
    pub ce: clock_event_device,
    pub base: *mut void __iomem,
    pub ticks_per_jiffy: u32,
    pub /: *mut *mut *mut int timer_id; / one of TIMER_,
}

    static struct digicolor_timer *dc_timer(struct clock_event_device *ce)
    {
    return container_of(ce, struct digicolor_timer, ce);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_timer_disable(ce: *mut clock_event_device) {
    static inline void dc_timer_disable(struct clock_event_device *ce)
    {
    struct digicolor_timer *dt = dc_timer(ce);
    writeb(CONTROL_DISABLE, dt.base + CONTROL(dt.timer_id));
    }
#[no_mangle]
pub unsafe extern "C" fn dc_timer_enable(ce: *mut clock_event_device, mode: u32) {
    static inline void dc_timer_enable(struct clock_event_device *ce, u32 mode)
    {
    struct digicolor_timer *dt = dc_timer(ce);
    writeb(CONTROL_ENABLE | mode, dt.base + CONTROL(dt.timer_id));
    }
    static inline void dc_timer_set_count(struct clock_event_device *ce,
    unsigned long count)
    {
    struct digicolor_timer *dt = dc_timer(ce);
    writel(count, dt.base + COUNT(dt.timer_id));
    }
#[no_mangle]
unsafe extern "C" fn digicolor_clkevt_shutdown(ce: *mut clock_event_device) -> c_int {
    static int digicolor_clkevt_shutdown(struct clock_event_device *ce)
    {
    dc_timer_disable(ce);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn digicolor_clkevt_set_oneshot(ce: *mut clock_event_device) -> c_int {
    static int digicolor_clkevt_set_oneshot(struct clock_event_device *ce)
    {
    dc_timer_disable(ce);
    dc_timer_enable(ce, CONTROL_MODE_ONESHOT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn digicolor_clkevt_set_periodic(ce: *mut clock_event_device) -> c_int {
    static int digicolor_clkevt_set_periodic(struct clock_event_device *ce)
    {
    struct digicolor_timer *dt = dc_timer(ce);
    dc_timer_disable(ce);
    dc_timer_set_count(ce, dt.ticks_per_jiffy);
    dc_timer_enable(ce, CONTROL_MODE_PERIODIC);
    return 0;
    }
    static int digicolor_clkevt_next_event(unsigned long evt,
    struct clock_event_device *ce)
    {
    dc_timer_disable(ce);
    dc_timer_set_count(ce, evt);
    dc_timer_enable(ce, CONTROL_MODE_ONESHOT);
    return 0;
    }
    static struct digicolor_timer dc_timer_dev = {
    .ce = {
    .name = "digicolor_tick",
    .rating = 340,
    .features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT,
    .set_state_shutdown = digicolor_clkevt_shutdown,
    .set_state_periodic = digicolor_clkevt_set_periodic,
    .set_state_oneshot = digicolor_clkevt_set_oneshot,
    .tick_resume = digicolor_clkevt_shutdown,
    .set_next_event = digicolor_clkevt_next_event,
    },
    .timer_id = TIMER_C,
    };
#[no_mangle]
unsafe extern "C" fn digicolor_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t digicolor_timer_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *evt = dev_id;
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn digicolor_timer_sched_read() -> u64 notrace {
    static u64 notrace digicolor_timer_sched_read(void)
    {
    return ~readl(dc_timer_dev.base + COUNT(TIMER_B));
    }
#[no_mangle]
unsafe extern "C" fn digicolor_timer_init(node: *mut device_node) -> int __init {
    static int __init digicolor_timer_init(struct device_node *node)
    {
    unsigned long rate;
    struct clk *clk;
    int ret, irq;
//
// timer registers are shared with the watchdog timer;
// don't map exclusively
//
    dc_timer_dev.base = of_iomap(node, 0);
    if (!dc_timer_dev.base) {
    pr_err("Can't map registers\n");
    return -ENXIO;
    }
    irq = irq_of_parse_and_map(node, dc_timer_dev.timer_id);
    if (irq <= 0) {
    pr_err("Can't parse IRQ\n");
    return -EINVAL;
    }
    clk = of_clk_get(node, 0);
    if (IS_ERR(clk)) {
    pr_err("Can't get timer clock\n");
    return PTR_ERR(clk);
    }
    clk_prepare_enable(clk);
    rate = clk_get_rate(clk);
    dc_timer_dev.ticks_per_jiffy = DIV_ROUND_UP(rate, HZ);
    writeb(CONTROL_DISABLE, dc_timer_dev.base + CONTROL(TIMER_B));
    writel(UINT_MAX, dc_timer_dev.base + COUNT(TIMER_B));
    writeb(CONTROL_ENABLE, dc_timer_dev.base + CONTROL(TIMER_B));
    sched_clock_register(digicolor_timer_sched_read, 32, rate);
    clocksource_mmio_init(dc_timer_dev.base + COUNT(TIMER_B), node.name,
    rate, 340, 32, clocksource_mmio_readl_down);
    ret = request_irq(irq, digicolor_timer_interrupt,
    IRQF_TIMER | IRQF_IRQPOLL, "digicolor_timerC",
    &dc_timer_dev.ce);
    if (ret) {
    pr_warn("request of timer irq %d failed (%d)\n", irq, ret);
    return ret;
    }
    dc_timer_dev.ce.cpumask = cpu_possible_mask;
    dc_timer_dev.ce.irq = irq;
    clockevents_config_and_register(&dc_timer_dev.ce, rate, 0, 0xffffffff);
    return 0;
    }
    TIMER_OF_DECLARE(conexant_digicolor, "cnxt,cx92755-timer",
    digicolor_timer_init);
