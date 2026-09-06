//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-loongson1-pwm.c
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
// Clocksource driver for Loongson-1 SoC
//
// Copyright (c) 2023 Keguang Zhang <keguang.zhang@gmail.com>
//

// Loongson-1 PWM Timer Register Definitions
pub const PWM_CNTR: c_uint = 0x0;
pub const PWM_HRC: c_uint = 0x4;
pub const PWM_LRC: c_uint = 0x8;
pub const PWM_CTRL: c_uint = 0xc;
// PWM Control Register Bits

pub const CNTR_WIDTH: c_int = 24;
    static DEFINE_RAW_SPINLOCK(ls1x_timer_lock);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ls1x_clocksource {
    pub reg_base: *mut void __iomem,
    pub ticks_per_jiffy: c_ulong,
    pub clksrc: clocksource,
}

    static inline struct ls1x_clocksource *to_ls1x_clksrc(struct clocksource *c)
    {
    return container_of(c, struct ls1x_clocksource, clksrc);
    }
    static inline void ls1x_pwmtimer_set_period(unsigned int period,
    struct timer_of *to)
    {
    writel(period, timer_of_base(to) + PWM_LRC);
    writel(period, timer_of_base(to) + PWM_HRC);
    }
#[no_mangle]
pub unsafe extern "C" fn ls1x_pwmtimer_clear(to: *mut timer_of) {
    static inline void ls1x_pwmtimer_clear(struct timer_of *to)
    {
    writel(0, timer_of_base(to) + PWM_CNTR);
    }
#[no_mangle]
pub unsafe extern "C" fn ls1x_pwmtimer_start(to: *mut timer_of) {
    static inline void ls1x_pwmtimer_start(struct timer_of *to)
    {
    writel((INT_EN | PWM_OE | CNT_EN), timer_of_base(to) + PWM_CTRL);
    }
#[no_mangle]
pub unsafe extern "C" fn ls1x_pwmtimer_stop(to: *mut timer_of) {
    static inline void ls1x_pwmtimer_stop(struct timer_of *to)
    {
    writel(0, timer_of_base(to) + PWM_CTRL);
    }
#[no_mangle]
pub unsafe extern "C" fn ls1x_pwmtimer_irq_ack(to: *mut timer_of) {
    static inline void ls1x_pwmtimer_irq_ack(struct timer_of *to)
    {
    int val;
    val = readl(timer_of_base(to) + PWM_CTRL);
    val |= INT_SR;
    writel(val, timer_of_base(to) + PWM_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn ls1x_clockevent_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ls1x_clockevent_isr(int irq, void *dev_id)
    {
    struct clock_event_device *clkevt = dev_id;
    struct timer_of *to = to_timer_of(clkevt);
    ls1x_pwmtimer_irq_ack(to);
    ls1x_pwmtimer_clear(to);
    ls1x_pwmtimer_start(to);
    clkevt.event_handler(clkevt);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ls1x_clockevent_set_state_periodic(clkevt: *mut clock_event_device) -> c_int {
    static int ls1x_clockevent_set_state_periodic(struct clock_event_device *clkevt)
    {
    struct timer_of *to = to_timer_of(clkevt);
    raw_spin_lock(&ls1x_timer_lock);
    ls1x_pwmtimer_set_period(timer_of_period(to), to);
    ls1x_pwmtimer_clear(to);
    ls1x_pwmtimer_start(to);
    raw_spin_unlock(&ls1x_timer_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls1x_clockevent_tick_resume(clkevt: *mut clock_event_device) -> c_int {
    static int ls1x_clockevent_tick_resume(struct clock_event_device *clkevt)
    {
    raw_spin_lock(&ls1x_timer_lock);
    ls1x_pwmtimer_start(to_timer_of(clkevt));
    raw_spin_unlock(&ls1x_timer_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ls1x_clockevent_set_state_shutdown(clkevt: *mut clock_event_device) -> c_int {
    static int ls1x_clockevent_set_state_shutdown(struct clock_event_device *clkevt)
    {
    raw_spin_lock(&ls1x_timer_lock);
    ls1x_pwmtimer_stop(to_timer_of(clkevt));
    raw_spin_unlock(&ls1x_timer_lock);
    return 0;
    }
    static int ls1x_clockevent_set_next(unsigned long evt,
    struct clock_event_device *clkevt)
    {
    struct timer_of *to = to_timer_of(clkevt);
    raw_spin_lock(&ls1x_timer_lock);
    ls1x_pwmtimer_set_period(evt, to);
    ls1x_pwmtimer_clear(to);
    ls1x_pwmtimer_start(to);
    raw_spin_unlock(&ls1x_timer_lock);
    return 0;
    }
    static struct timer_of ls1x_to = {
    .flags = TIMER_OF_IRQ | TIMER_OF_BASE | TIMER_OF_CLOCK,
    .clkevt = {
    .name			= "ls1x-pwmtimer",
    .features		= CLOCK_EVT_FEAT_PERIODIC |
    CLOCK_EVT_FEAT_ONESHOT,
    .rating			= 300,
    .set_next_event		= ls1x_clockevent_set_next,
    .set_state_periodic	= ls1x_clockevent_set_state_periodic,
    .set_state_oneshot	= ls1x_clockevent_set_state_shutdown,
    .set_state_shutdown	= ls1x_clockevent_set_state_shutdown,
    .tick_resume		= ls1x_clockevent_tick_resume,
    },
    .of_irq = {
    .handler		= ls1x_clockevent_isr,
    .flags			= IRQF_TIMER,
    },
    };
//
// Since the PWM timer overflows every two ticks, its not very useful
// to just read by itself. So use jiffies to emulate a free
// running counter:
//
#[no_mangle]
unsafe extern "C" fn ls1x_clocksource_read(cs: *mut clocksource) -> u64 {
    static u64 ls1x_clocksource_read(struct clocksource *cs)
    {
    struct ls1x_clocksource *ls1x_cs = to_ls1x_clksrc(cs);
    unsigned long flags;
    int count;
    u32 jifs;
    static int old_count;
    static u32 old_jifs;
    raw_spin_lock_irqsave(&ls1x_timer_lock, flags);
//
// Although our caller may have the read side of xtime_lock,
// this is now a seqlock, and we are cheating in this routine
// by having side effects on state that we cannot undo if
// there is a collision on the seqlock and our caller has to
// retry.  (Namely, old_jifs and old_count.)  So we must treat
// jiffies as volatile despite the lock.  We read jiffies
// before latching the timer count to guarantee that although
// the jiffies value might be older than the count (that is,
// the counter may underflow between the last point where
// jiffies was incremented and the point where we latch the
// count), it cannot be newer.
//
    jifs = jiffies;
// read the count
    count = readl(ls1x_cs.reg_base + PWM_CNTR);
//
// It's possible for count to appear to go the wrong way for this
// reason:
//
// The timer counter underflows, but we haven't handled the resulting
// interrupt and incremented jiffies yet.
//
// Previous attempts to handle these cases intelligently were buggy, so
// we just do the simple thing now.
//
    if (count < old_count && jifs == old_jifs)
    count = old_count;
    old_count = count;
    old_jifs = jifs;
    raw_spin_unlock_irqrestore(&ls1x_timer_lock, flags);
    return (u64)(jifs * ls1x_cs.ticks_per_jiffy) + count;
    }
    static struct ls1x_clocksource ls1x_clocksource = {
    .clksrc = {
    .name           = "ls1x-pwmtimer",
    .rating		= 300,
    .read           = ls1x_clocksource_read,
    .mask           = CLOCKSOURCE_MASK(CNTR_WIDTH),
    .flags          = CLOCK_SOURCE_IS_CONTINUOUS,
    },
    };
#[no_mangle]
unsafe extern "C" fn ls1x_pwm_clocksource_init(np: *mut device_node) -> int __init {
    static int __init ls1x_pwm_clocksource_init(struct device_node *np)
    {
    struct timer_of *to = &ls1x_to;
    int ret;
    ret = timer_of_init(np, to);
    if (ret)
    return ret;
    clockevents_config_and_register(&to.clkevt, timer_of_rate(to),
    0x1, GENMASK(CNTR_WIDTH - 1, 0));
    ls1x_clocksource.reg_base = timer_of_base(to);
    ls1x_clocksource.ticks_per_jiffy = timer_of_period(to);
    return clocksource_register_hz(&ls1x_clocksource.clksrc,
    timer_of_rate(to));
    }
    TIMER_OF_DECLARE(ls1x_pwm_clocksource, "loongson,ls1b-pwmtimer",
    ls1x_pwm_clocksource_init);
