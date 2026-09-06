//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-sprd.c
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
// Copyright (C) 2017 Spreadtrum Communications Inc.
//

pub const TIMER_LOAD_LO: c_uint = 0x0;
pub const TIMER_LOAD_HI: c_uint = 0x4;
pub const TIMER_VALUE_LO: c_uint = 0x8;
pub const TIMER_VALUE_HI: c_uint = 0xc;
pub const TIMER_CTL: c_uint = 0x10;

pub const TIMER_INT: c_uint = 0x14;

pub const TIMER_VALUE_SHDW_LO: c_uint = 0x18;
pub const TIMER_VALUE_SHDW_HI: c_uint = 0x1c;

#[no_mangle]
unsafe extern "C" fn sprd_timer_enable(base: *mut void __iomem, flag: u32) {
    static void sprd_timer_enable(void __iomem *base, u32 flag)
    {
    let mut val: u32 = readl_relaxed(base + TIMER_CTL);
    val |= TIMER_CTL_ENABLE;
    if (flag & TIMER_CTL_64BIT_WIDTH)
    val |= TIMER_CTL_64BIT_WIDTH;
    else
    val &= ~TIMER_CTL_64BIT_WIDTH;
    if (flag & TIMER_CTL_PERIOD_MODE)
    val |= TIMER_CTL_PERIOD_MODE;
    else
    val &= ~TIMER_CTL_PERIOD_MODE;
    writel_relaxed(val, base + TIMER_CTL);
    }
#[no_mangle]
unsafe extern "C" fn sprd_timer_disable(base: *mut void __iomem) {
    static void sprd_timer_disable(void __iomem *base)
    {
    let mut val: u32 = readl_relaxed(base + TIMER_CTL);
    val &= ~TIMER_CTL_ENABLE;
    writel_relaxed(val, base + TIMER_CTL);
    }
#[no_mangle]
unsafe extern "C" fn sprd_timer_update_counter(base: *mut void __iomem, cycles: c_ulong) {
    static void sprd_timer_update_counter(void __iomem *base, unsigned long cycles)
    {
    writel_relaxed(cycles & TIMER_VALUE_LO_MASK, base + TIMER_LOAD_LO);
    writel_relaxed(0, base + TIMER_LOAD_HI);
    }
#[no_mangle]
unsafe extern "C" fn sprd_timer_enable_interrupt(base: *mut void __iomem) {
    static void sprd_timer_enable_interrupt(void __iomem *base)
    {
    writel_relaxed(TIMER_INT_EN, base + TIMER_INT);
    }
#[no_mangle]
unsafe extern "C" fn sprd_timer_clear_interrupt(base: *mut void __iomem) {
    static void sprd_timer_clear_interrupt(void __iomem *base)
    {
    let mut val: u32 = readl_relaxed(base + TIMER_INT);
    val |= TIMER_INT_CLR;
    writel_relaxed(val, base + TIMER_INT);
    }
    static int sprd_timer_set_next_event(unsigned long cycles,
    struct clock_event_device *ce)
    {
    struct timer_of *to = to_timer_of(ce);
    sprd_timer_disable(timer_of_base(to));
    sprd_timer_update_counter(timer_of_base(to), cycles);
    sprd_timer_enable(timer_of_base(to), 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_timer_set_periodic(ce: *mut clock_event_device) -> c_int {
    static int sprd_timer_set_periodic(struct clock_event_device *ce)
    {
    struct timer_of *to = to_timer_of(ce);
    sprd_timer_disable(timer_of_base(to));
    sprd_timer_update_counter(timer_of_base(to), timer_of_period(to));
    sprd_timer_enable(timer_of_base(to), TIMER_CTL_PERIOD_MODE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_timer_shutdown(ce: *mut clock_event_device) -> c_int {
    static int sprd_timer_shutdown(struct clock_event_device *ce)
    {
    struct timer_of *to = to_timer_of(ce);
    sprd_timer_disable(timer_of_base(to));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sprd_timer_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *ce = (struct clock_event_device *)dev_id;
    struct timer_of *to = to_timer_of(ce);
    sprd_timer_clear_interrupt(timer_of_base(to));
    if (clockevent_state_oneshot(ce))
    sprd_timer_disable(timer_of_base(to));
    ce.event_handler(ce);
    return IRQ_HANDLED;
    }
    static struct timer_of to = {
    .flags = TIMER_OF_IRQ | TIMER_OF_BASE | TIMER_OF_CLOCK,
    .clkevt = {
    .name = TIMER_NAME,
    .rating = 300,
    .features = CLOCK_EVT_FEAT_DYNIRQ | CLOCK_EVT_FEAT_PERIODIC |
    CLOCK_EVT_FEAT_ONESHOT,
    .set_state_shutdown = sprd_timer_shutdown,
    .set_state_periodic = sprd_timer_set_periodic,
    .set_next_event = sprd_timer_set_next_event,
    .cpumask = cpu_possible_mask,
    },
    .of_irq = {
    .handler = sprd_timer_interrupt,
    .flags = IRQF_TIMER | IRQF_IRQPOLL,
    },
    };
#[no_mangle]
unsafe extern "C" fn sprd_timer_init(np: *mut device_node) -> int __init {
    static int __init sprd_timer_init(struct device_node *np)
    {
    int ret;
    ret = timer_of_init(np, &to);
    if (ret)
    return ret;
    sprd_timer_enable_interrupt(timer_of_base(&to));
    clockevents_config_and_register(&to.clkevt, timer_of_rate(&to),
    1, UINT_MAX);
    return 0;
    }
    static struct timer_of suspend_to = {
    .flags = TIMER_OF_BASE | TIMER_OF_CLOCK,
    };
#[no_mangle]
unsafe extern "C" fn sprd_suspend_timer_read(cs: *mut clocksource) -> u64 {
    static u64 sprd_suspend_timer_read(struct clocksource *cs)
    {
    u32 lo, hi;
    do {
    hi = readl_relaxed(timer_of_base(&suspend_to) +
    TIMER_VALUE_SHDW_HI);
    lo = readl_relaxed(timer_of_base(&suspend_to) +
    TIMER_VALUE_SHDW_LO);
    } while (hi != readl_relaxed(timer_of_base(&suspend_to) + TIMER_VALUE_SHDW_HI));
    return ~(((u64)hi << 32) | lo);
    }
#[no_mangle]
unsafe extern "C" fn sprd_suspend_timer_enable(cs: *mut clocksource) -> c_int {
    static int sprd_suspend_timer_enable(struct clocksource *cs)
    {
    writel_relaxed(TIMER_VALUE_LO_MASK,
    timer_of_base(&suspend_to) + TIMER_LOAD_LO);
    writel_relaxed(TIMER_VALUE_HI_MASK,
    timer_of_base(&suspend_to) + TIMER_LOAD_HI);
    sprd_timer_enable(timer_of_base(&suspend_to),
    TIMER_CTL_PERIOD_MODE|TIMER_CTL_64BIT_WIDTH);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sprd_suspend_timer_disable(cs: *mut clocksource) {
    static void sprd_suspend_timer_disable(struct clocksource *cs)
    {
    sprd_timer_disable(timer_of_base(&suspend_to));
    }
    static struct clocksource suspend_clocksource = {
    .name	= "sprd_suspend_timer",
    .rating	= 200,
    .read	= sprd_suspend_timer_read,
    .enable = sprd_suspend_timer_enable,
    .disable = sprd_suspend_timer_disable,
    .mask	= CLOCKSOURCE_MASK(64),
    .flags	= CLOCK_SOURCE_IS_CONTINUOUS | CLOCK_SOURCE_SUSPEND_NONSTOP,
    };
#[no_mangle]
unsafe extern "C" fn sprd_suspend_timer_init(np: *mut device_node) -> int __init {
    static int __init sprd_suspend_timer_init(struct device_node *np)
    {
    int ret;
    ret = timer_of_init(np, &suspend_to);
    if (ret)
    return ret;
    clocksource_register_hz(&suspend_clocksource,
    timer_of_rate(&suspend_to));
    return 0;
    }
    TIMER_OF_DECLARE(sc9860_timer, "sprd,sc9860-timer", sprd_timer_init);
    TIMER_OF_DECLARE(sc9860_persistent_timer, "sprd,sc9860-suspend-timer",
    sprd_suspend_timer_init);
