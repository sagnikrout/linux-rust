//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-meson6.c
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
// Amlogic Meson6 SoCs timer handling.
//
// Copyright (C) 2014 Carlo Caione <carlo@caione.org>
//
// Based on code from Amlogic, Inc
//

pub const MESON_ISA_TIMER_MUX: c_uint = 0x00;

pub const MESON_ISA_TIMER_MUX_TIMERE_INPUT_CLOCK_SYSTEM_CLOCK: c_uint = 0x0;
pub const MESON_ISA_TIMER_MUX_TIMERE_INPUT_CLOCK_1US: c_uint = 0x1;
pub const MESON_ISA_TIMER_MUX_TIMERE_INPUT_CLOCK_10US: c_uint = 0x2;
pub const MESON_ISA_TIMER_MUX_TIMERE_INPUT_CLOCK_100US: c_uint = 0x3;
pub const MESON_ISA_TIMER_MUX_TIMERE_INPUT_CLOCK_1MS: c_uint = 0x4;

pub const MESON_ISA_TIMER_MUX_TIMERABCD_INPUT_CLOCK_1US: c_uint = 0x0;
pub const MESON_ISA_TIMER_MUX_TIMERABCD_INPUT_CLOCK_10US: c_uint = 0x1;
pub const MESON_ISA_TIMER_MUX_TIMERABCD_INPUT_CLOCK_100US: c_uint = 0x0;
pub const MESON_ISA_TIMER_MUX_TIMERABCD_INPUT_CLOCK_1MS: c_uint = 0x3;
pub const MESON_ISA_TIMERA: c_uint = 0x04;
pub const MESON_ISA_TIMERB: c_uint = 0x08;
pub const MESON_ISA_TIMERC: c_uint = 0x0c;
pub const MESON_ISA_TIMERD: c_uint = 0x10;
pub const MESON_ISA_TIMERE: c_uint = 0x14;
    static void __iomem *timer_base;

#[no_mangle]
unsafe extern "C" fn meson6_read_current_timer() -> c_ulong {
    static unsigned long meson6_read_current_timer(void)
    {
    return readl_relaxed(timer_base + MESON_ISA_TIMERE);
    }
    static struct delay_timer meson6_delay_timer = {
    .read_current_timer = meson6_read_current_timer,
    .freq = 1000 * 1000,
    };

#[no_mangle]
unsafe extern "C" fn meson6_timer_sched_read() -> u64 notrace {
    static u64 notrace meson6_timer_sched_read(void)
    {
    return (u64)readl(timer_base + MESON_ISA_TIMERE);
    }
#[no_mangle]
unsafe extern "C" fn meson6_clkevt_time_stop() {
    static void meson6_clkevt_time_stop(void)
    {
    let mut val: u32 = readl(timer_base + MESON_ISA_TIMER_MUX);
    writel(val & ~MESON_ISA_TIMER_MUX_TIMERA_EN,
    timer_base + MESON_ISA_TIMER_MUX);
    }
#[no_mangle]
unsafe extern "C" fn meson6_clkevt_time_setup(delay: c_ulong) {
    static void meson6_clkevt_time_setup(unsigned long delay)
    {
    writel(delay, timer_base + MESON_ISA_TIMERA);
    }
#[no_mangle]
unsafe extern "C" fn meson6_clkevt_time_start(periodic: bool) {
    static void meson6_clkevt_time_start(bool periodic)
    {
    let mut val: u32 = readl(timer_base + MESON_ISA_TIMER_MUX);
    if (periodic)
    val |= MESON_ISA_TIMER_MUX_TIMERA_MODE;
    else
    val &= ~MESON_ISA_TIMER_MUX_TIMERA_MODE;
    writel(val | MESON_ISA_TIMER_MUX_TIMERA_EN,
    timer_base + MESON_ISA_TIMER_MUX);
    }
#[no_mangle]
unsafe extern "C" fn meson6_shutdown(evt: *mut clock_event_device) -> c_int {
    static int meson6_shutdown(struct clock_event_device *evt)
    {
    meson6_clkevt_time_stop();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson6_set_oneshot(evt: *mut clock_event_device) -> c_int {
    static int meson6_set_oneshot(struct clock_event_device *evt)
    {
    meson6_clkevt_time_stop();
    meson6_clkevt_time_start(false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn meson6_set_periodic(evt: *mut clock_event_device) -> c_int {
    static int meson6_set_periodic(struct clock_event_device *evt)
    {
    meson6_clkevt_time_stop();
    meson6_clkevt_time_setup(USEC_PER_SEC / HZ - 1);
    meson6_clkevt_time_start(true);
    return 0;
    }
    static int meson6_clkevt_next_event(unsigned long evt,
    struct clock_event_device *unused)
    {
    meson6_clkevt_time_stop();
    meson6_clkevt_time_setup(evt);
    meson6_clkevt_time_start(false);
    return 0;
    }
    static struct clock_event_device meson6_clockevent = {
    .name			= "meson6_tick",
    .rating			= 400,
    .features		= CLOCK_EVT_FEAT_PERIODIC |
    CLOCK_EVT_FEAT_ONESHOT,
    .set_state_shutdown	= meson6_shutdown,
    .set_state_periodic	= meson6_set_periodic,
    .set_state_oneshot	= meson6_set_oneshot,
    .tick_resume		= meson6_shutdown,
    .set_next_event		= meson6_clkevt_next_event,
    };
#[no_mangle]
unsafe extern "C" fn meson6_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t meson6_timer_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *evt = (struct clock_event_device *)dev_id;
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn meson6_timer_init(node: *mut device_node) -> int __init {
    static int __init meson6_timer_init(struct device_node *node)
    {
    u32 val;
    int ret, irq;
    timer_base = of_io_request_and_map(node, 0, "meson6-timer");
    if (IS_ERR(timer_base)) {
    pr_err("Can't map registers\n");
    return -ENXIO;
    }
    irq = irq_of_parse_and_map(node, 0);
    if (irq <= 0) {
    pr_err("Can't parse IRQ\n");
    return -EINVAL;
    }
// Set 1us for timer E
    val = readl(timer_base + MESON_ISA_TIMER_MUX);
    val &= ~MESON_ISA_TIMER_MUX_TIMERE_INPUT_CLOCK_MASK;
    val |= FIELD_PREP(MESON_ISA_TIMER_MUX_TIMERE_INPUT_CLOCK_MASK,
    MESON_ISA_TIMER_MUX_TIMERE_INPUT_CLOCK_1US);
    writel(val, timer_base + MESON_ISA_TIMER_MUX);
    sched_clock_register(meson6_timer_sched_read, 32, USEC_PER_SEC);
    clocksource_mmio_init(timer_base + MESON_ISA_TIMERE, node.name,
    1000 * 1000, 300, 32, clocksource_mmio_readl_up);
// Timer A base 1us
    val &= ~MESON_ISA_TIMER_MUX_TIMERA_INPUT_CLOCK_MASK;
    val |= FIELD_PREP(MESON_ISA_TIMER_MUX_TIMERA_INPUT_CLOCK_MASK,
    MESON_ISA_TIMER_MUX_TIMERABCD_INPUT_CLOCK_1US);
    writel(val, timer_base + MESON_ISA_TIMER_MUX);
// Stop the timer A
    meson6_clkevt_time_stop();
    ret = request_irq(irq, meson6_timer_interrupt,
    IRQF_TIMER | IRQF_IRQPOLL, "meson6_timer",
    &meson6_clockevent);
    if (ret) {
    pr_warn("failed to setup irq %d\n", irq);
    return ret;
    }
    meson6_clockevent.cpumask = cpu_possible_mask;
    meson6_clockevent.irq = irq;
    clockevents_config_and_register(&meson6_clockevent, USEC_PER_SEC,
    1, 0xfffe);

// Also use MESON_ISA_TIMERE for delays
    register_current_timer_delay(&meson6_delay_timer);

    return 0;
    }
    TIMER_OF_DECLARE(meson6, "amlogic,meson6-timer",
    meson6_timer_init);
