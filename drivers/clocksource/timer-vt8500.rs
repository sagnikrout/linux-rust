//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-vt8500.c
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
// arch/arm/mach-vt8500/timer.c
//
// Copyright (C) 2012 Tony Prisk <linux@prisktech.co.nz>
// Copyright (C) 2010 Alexey Charkov <alchark@gmail.com>
//
// This file is copied and modified from the original timer.c provided by
// Alexey Charkov. Minor changes have been made for Device Tree Support.
//

pub const VT8500_TIMER_OFFSET: c_uint = 0x0100;
pub const VT8500_TIMER_HZ: c_int = 3000000;
pub const TIMER_MATCH_VAL: c_uint = 0x0000;
pub const TIMER_COUNT_VAL: c_uint = 0x0010;
pub const TIMER_STATUS_VAL: c_uint = 0x0014;
pub const TIMER_IER_VAL: c_uint = 0x001c		/* interrupt enable */;
pub const TIMER_CTRL_VAL: c_uint = 0x0020;
pub const TIMER_AS_VAL: c_uint = 0x0024		/* access status */;

pub const MIN_OSCR_DELTA: c_int = 16;
    static void __iomem *regbase;
#[no_mangle]
unsafe extern "C" fn vt8500_timer_read(cs: *mut clocksource) -> u64 {
    static u64 vt8500_timer_read(struct clocksource *cs)
    {
    let mut loops: c_int = msecs_to_loops(10);
    writel(3, regbase + TIMER_CTRL_VAL);
    while ((readl((regbase + TIMER_AS_VAL)) & TIMER_COUNT_R_ACTIVE)
    && --loops)
    cpu_relax();
    return readl(regbase + TIMER_COUNT_VAL);
    }
    static struct clocksource clocksource = {
    .name           = "vt8500_timer",
    .rating         = 200,
    .read           = vt8500_timer_read,
    .mask           = CLOCKSOURCE_MASK(32),
    .flags          = CLOCK_SOURCE_IS_CONTINUOUS,
    };
    static int vt8500_timer_set_next_event(unsigned long cycles,
    struct clock_event_device *evt)
    {
    let mut loops: c_int = msecs_to_loops(10);
    let mut alarm: u64 = clocksource.read(&clocksource) + cycles;
    while ((readl(regbase + TIMER_AS_VAL) & TIMER_MATCH_W_ACTIVE)
    && --loops)
    cpu_relax();
    writel((unsigned long)alarm, regbase + TIMER_MATCH_VAL);
    if ((signed)(alarm - clocksource.read(&clocksource)) <= MIN_OSCR_DELTA)
    return -ETIME;
    writel(1, regbase + TIMER_IER_VAL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vt8500_shutdown(evt: *mut clock_event_device) -> c_int {
    static int vt8500_shutdown(struct clock_event_device *evt)
    {
    writel(readl(regbase + TIMER_CTRL_VAL) | 1, regbase + TIMER_CTRL_VAL);
    writel(0, regbase + TIMER_IER_VAL);
    return 0;
    }
    static struct clock_event_device clockevent = {
    .name			= "vt8500_timer",
    .features		= CLOCK_EVT_FEAT_ONESHOT,
    .rating			= 200,
    .set_next_event		= vt8500_timer_set_next_event,
    .set_state_shutdown	= vt8500_shutdown,
    .set_state_oneshot	= vt8500_shutdown,
    };
#[no_mangle]
unsafe extern "C" fn vt8500_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t vt8500_timer_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *evt = dev_id;
    writel(0xf, regbase + TIMER_STATUS_VAL);
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn vt8500_timer_init(np: *mut device_node) -> int __init {
    static int __init vt8500_timer_init(struct device_node *np)
    {
    int timer_irq, ret;
    regbase = of_iomap(np, 0);
    if (!regbase) {
    pr_err("%s: Missing iobase description in Device Tree\n",
    __func__);
    return -ENXIO;
    }
    timer_irq = irq_of_parse_and_map(np, 0);
    if (!timer_irq) {
    pr_err("%s: Missing irq description in Device Tree\n",
    __func__);
    return -EINVAL;
    }
    writel(1, regbase + TIMER_CTRL_VAL);
    writel(0xf, regbase + TIMER_STATUS_VAL);
    writel(~0, regbase + TIMER_MATCH_VAL);
    ret = clocksource_register_hz(&clocksource, VT8500_TIMER_HZ);
    if (ret) {
    pr_err("%s: clocksource_register failed for %s\n",
    __func__, clocksource.name);
    return ret;
    }
    clockevent.cpumask = cpumask_of(0);
    ret = request_irq(timer_irq, vt8500_timer_interrupt,
    IRQF_TIMER | IRQF_IRQPOLL, "vt8500_timer",
    &clockevent);
    if (ret) {
    pr_err("%s: setup_irq failed for %s\n", __func__,
    clockevent.name);
    return ret;
    }
    clockevents_config_and_register(&clockevent, VT8500_TIMER_HZ,
    MIN_OSCR_DELTA * 2, 0xf0000000);
    return 0;
    }
    TIMER_OF_DECLARE(vt8500, "via,vt8500-timer", vt8500_timer_init);
