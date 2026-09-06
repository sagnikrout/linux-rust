//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-rda.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// RDA8810PL SoC timer driver
//
// Copyright RDA Microelectronics Company Limited
// Copyright (c) 2017 Andreas Färber
// Copyright (c) 2018 Manivannan Sadhasivam
//
// RDA8810PL has two independent timers: OSTIMER (56 bit) and HWTIMER (64 bit).
// Each timer provides optional interrupt support. In this driver, OSTIMER is
// used for clockevents and HWTIMER is used for clocksource.
//

pub const RDA_OSTIMER_LOADVAL_L: c_uint = 0x000;
pub const RDA_OSTIMER_CTRL: c_uint = 0x004;
pub const RDA_HWTIMER_LOCKVAL_L: c_uint = 0x024;
pub const RDA_HWTIMER_LOCKVAL_H: c_uint = 0x028;
pub const RDA_TIMER_IRQ_MASK_SET: c_uint = 0x02c;
pub const RDA_TIMER_IRQ_MASK_CLR: c_uint = 0x030;
pub const RDA_TIMER_IRQ_CLR: c_uint = 0x034;

#[no_mangle]
unsafe extern "C" fn rda_ostimer_start(base: *mut void __iomem, periodic: bool, cycles: u64) -> c_int {
    static int rda_ostimer_start(void __iomem *base, bool periodic, u64 cycles)
    {
    u32 ctrl, load_l;
    load_l = (u32)cycles;
    ctrl = ((cycles >> 32) & 0xffffff);
    ctrl |= RDA_OSTIMER_CTRL_LOAD | RDA_OSTIMER_CTRL_ENABLE;
    if (periodic)
    ctrl |= RDA_OSTIMER_CTRL_REPEAT;
// Enable ostimer interrupt first
    writel_relaxed(RDA_TIMER_IRQ_MASK_OSTIMER,
    base + RDA_TIMER_IRQ_MASK_SET);
// Write low 32 bits first, high 24 bits are with ctrl
    writel_relaxed(load_l, base + RDA_OSTIMER_LOADVAL_L);
    writel_relaxed(ctrl, base + RDA_OSTIMER_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rda_ostimer_stop(base: *mut void __iomem) -> c_int {
    static int rda_ostimer_stop(void __iomem *base)
    {
// Disable ostimer interrupt first
    writel_relaxed(RDA_TIMER_IRQ_MASK_OSTIMER,
    base + RDA_TIMER_IRQ_MASK_CLR);
    writel_relaxed(0, base + RDA_OSTIMER_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rda_ostimer_set_state_shutdown(evt: *mut clock_event_device) -> c_int {
    static int rda_ostimer_set_state_shutdown(struct clock_event_device *evt)
    {
    struct timer_of *to = to_timer_of(evt);
    rda_ostimer_stop(timer_of_base(to));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rda_ostimer_set_state_oneshot(evt: *mut clock_event_device) -> c_int {
    static int rda_ostimer_set_state_oneshot(struct clock_event_device *evt)
    {
    struct timer_of *to = to_timer_of(evt);
    rda_ostimer_stop(timer_of_base(to));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rda_ostimer_set_state_periodic(evt: *mut clock_event_device) -> c_int {
    static int rda_ostimer_set_state_periodic(struct clock_event_device *evt)
    {
    struct timer_of *to = to_timer_of(evt);
    unsigned long cycles_per_jiffy;
    rda_ostimer_stop(timer_of_base(to));
    cycles_per_jiffy = ((unsigned long long)NSEC_PER_SEC / HZ *
    evt.mult) >> evt.shift;
    rda_ostimer_start(timer_of_base(to), true, cycles_per_jiffy);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rda_ostimer_tick_resume(evt: *mut clock_event_device) -> c_int {
    static int rda_ostimer_tick_resume(struct clock_event_device *evt)
    {
    return 0;
    }
    static int rda_ostimer_set_next_event(unsigned long evt,
    struct clock_event_device *ev)
    {
    struct timer_of *to = to_timer_of(ev);
    rda_ostimer_start(timer_of_base(to), false, evt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rda_ostimer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t rda_ostimer_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *evt = dev_id;
    struct timer_of *to = to_timer_of(evt);
// clear timer int
    writel_relaxed(RDA_TIMER_IRQ_CLR_OSTIMER,
    timer_of_base(to) + RDA_TIMER_IRQ_CLR);
    if (evt.event_handler)
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
    static struct timer_of rda_ostimer_of = {
    .flags = TIMER_OF_IRQ | TIMER_OF_BASE,
    .clkevt = {
    .name = "rda-ostimer",
    .rating = 250,
    .features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT |
    CLOCK_EVT_FEAT_DYNIRQ,
    .set_state_shutdown = rda_ostimer_set_state_shutdown,
    .set_state_oneshot = rda_ostimer_set_state_oneshot,
    .set_state_periodic = rda_ostimer_set_state_periodic,
    .tick_resume = rda_ostimer_tick_resume,
    .set_next_event	= rda_ostimer_set_next_event,
    },
    .of_base = {
    .name = "rda-timer",
    .index = 0,
    },
    .of_irq = {
    .name = "ostimer",
    .handler = rda_ostimer_interrupt,
    .flags = IRQF_TIMER,
    },
    };
#[no_mangle]
unsafe extern "C" fn rda_hwtimer_clocksource_read() -> u64 {
    static u64 rda_hwtimer_clocksource_read(void)
    {
    void __iomem *base = timer_of_base(&rda_ostimer_of);
    u32 lo, hi;
// Always read low 32 bits first
    do {
    lo = readl_relaxed(base + RDA_HWTIMER_LOCKVAL_L);
    hi = readl_relaxed(base + RDA_HWTIMER_LOCKVAL_H);
    } while (hi != readl_relaxed(base + RDA_HWTIMER_LOCKVAL_H));
    return ((u64)hi << 32) | lo;
    }
#[no_mangle]
unsafe extern "C" fn rda_hwtimer_read(cs: *mut clocksource) -> u64 {
    static u64 rda_hwtimer_read(struct clocksource *cs)
    {
    return rda_hwtimer_clocksource_read();
    }
    static struct clocksource rda_hwtimer_clocksource = {
    .name           = "rda-timer",
    .rating         = 400,
    .read           = rda_hwtimer_read,
    .mask           = CLOCKSOURCE_MASK(64),
    .flags          = CLOCK_SOURCE_IS_CONTINUOUS,
    };
#[no_mangle]
unsafe extern "C" fn rda_timer_init(np: *mut device_node) -> int __init {
    static int __init rda_timer_init(struct device_node *np)
    {
    let mut rate: c_ulong = 2000000;
    int ret;
    ret = timer_of_init(np, &rda_ostimer_of);
    if (ret)
    return ret;
    clocksource_register_hz(&rda_hwtimer_clocksource, rate);
    sched_clock_register(rda_hwtimer_clocksource_read, 64, rate);
    clockevents_config_and_register(&rda_ostimer_of.clkevt, rate,
    0x2, UINT_MAX);
    return 0;
    }
    TIMER_OF_DECLARE(rda8810pl, "rda,8810pl-timer", rda_timer_init);
