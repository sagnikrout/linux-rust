//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/clps711x-timer.c
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
// Cirrus Logic CLPS711X clocksource driver
//
// Copyright (C) 2014 Alexander Shiyan <shc_work@mail.ru>
//

    enum {
    CLPS711X_CLKSRC_CLOCKSOURCE,
    CLPS711X_CLKSRC_CLOCKEVENT,
    };
    static void __iomem *tcd;
#[no_mangle]
unsafe extern "C" fn clps711x_sched_clock_read() -> u64 notrace {
    static u64 notrace clps711x_sched_clock_read(void)
    {
    return ~readw(tcd);
    }
#[no_mangle]
unsafe extern "C" fn clps711x_clksrc_init(clock: *mut clk, base: *mut void __iomem) -> void __init {
    static void __init clps711x_clksrc_init(struct clk *clock, void __iomem *base)
    {
    let mut rate: c_ulong = clk_get_rate(clock);
    tcd = base;
    clocksource_mmio_init(tcd, "clps711x-clocksource", rate, 300, 16,
    clocksource_mmio_readw_down);
    sched_clock_register(clps711x_sched_clock_read, 16, rate);
    }
#[no_mangle]
unsafe extern "C" fn clps711x_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t clps711x_timer_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *evt = dev_id;
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
    static int __init _clps711x_clkevt_init(struct clk *clock, void __iomem *base,
    unsigned int irq)
    {
    struct clock_event_device *clkevt;
    unsigned long rate;
    clkevt = kzalloc_obj(*clkevt);
    if (!clkevt)
    return -ENOMEM;
    rate = clk_get_rate(clock);
// Set Timer prescaler
    writew(DIV_ROUND_CLOSEST(rate, HZ), base);
    clkevt.name = "clps711x-clockevent";
    clkevt.rating = 300;
    clkevt.features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_C3STOP;
    clkevt.cpumask = cpumask_of(0);
    clockevents_config_and_register(clkevt, HZ, 0, 0);
    return request_irq(irq, clps711x_timer_interrupt, IRQF_TIMER,
    "clps711x-timer", clkevt);
    }
#[no_mangle]
unsafe extern "C" fn clps711x_timer_init(np: *mut device_node) -> int __init {
    static int __init clps711x_timer_init(struct device_node *np)
    {
    let mut irq: c_uint = irq_of_parse_and_map(np, 0);
    struct clk *clock = of_clk_get(np, 0);
    void __iomem *base = of_iomap(np, 0);
    let mut ret: c_int = 0;
    if (!base)
    return -ENOMEM;
    if (!irq) {
    ret = -EINVAL;
    goto unmap_io;
    }
    if (IS_ERR(clock)) {
    ret = PTR_ERR(clock);
    goto unmap_io;
    }
    switch (of_alias_get_id(np, "timer")) {
    case CLPS711X_CLKSRC_CLOCKSOURCE:
    clps711x_clksrc_init(clock, base);
    return 0;
    case CLPS711X_CLKSRC_CLOCKEVENT:
    ret =  _clps711x_clkevt_init(clock, base, irq);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    unmap_io:
    iounmap(base);
    return ret;
    }
    TIMER_OF_DECLARE(clps711x, "cirrus,ep7209-timer", clps711x_timer_init);
