//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-sun4i.c
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
// Allwinner A1X SoCs timer handling.
//
// Copyright (C) 2012 Maxime Ripard
//
// Maxime Ripard <maxime.ripard@free-electrons.com>
//
// Based on code from
// Allwinner Technology Co., Ltd. <www.allwinnertech.com>
// Benn Huang <benn@allwinnertech.com>
//

pub const TIMER_IRQ_EN_REG: c_uint = 0x00;

pub const TIMER_IRQ_ST_REG: c_uint = 0x04;

pub const TIMER_SYNC_TICKS: c_int = 3;
//
// When we disable a timer, we need to wait at least for 2 cycles of
// the timer source clock. We will use for that the clocksource timer
// that is already setup and runs at the same frequency than the other
// timers, and we never will be disabled.
//
#[no_mangle]
unsafe extern "C" fn sun4i_clkevt_sync(base: *mut void __iomem) {
    static void sun4i_clkevt_sync(void __iomem *base)
    {
    let mut old: u32 = readl(base + TIMER_CNTVAL_REG(1));
    while ((old - readl(base + TIMER_CNTVAL_REG(1))) < TIMER_SYNC_TICKS)
    cpu_relax();
    }
#[no_mangle]
unsafe extern "C" fn sun4i_clkevt_time_stop(base: *mut void __iomem, timer: u8) {
    static void sun4i_clkevt_time_stop(void __iomem *base, u8 timer)
    {
    let mut val: u32 = readl(base + TIMER_CTL_REG(timer));
    writel(val & ~TIMER_CTL_ENABLE, base + TIMER_CTL_REG(timer));
    sun4i_clkevt_sync(base);
    }
    static void sun4i_clkevt_time_setup(void __iomem *base, u8 timer,
    unsigned long delay)
    {
    writel(delay, base + TIMER_INTVAL_REG(timer));
    }
    static void sun4i_clkevt_time_start(void __iomem *base, u8 timer,
    bool periodic)
    {
    let mut val: u32 = readl(base + TIMER_CTL_REG(timer));
    if (periodic)
    val &= ~TIMER_CTL_ONESHOT;
    else
    val |= TIMER_CTL_ONESHOT;
    writel(val | TIMER_CTL_ENABLE | TIMER_CTL_RELOAD,
    base + TIMER_CTL_REG(timer));
    }
#[no_mangle]
unsafe extern "C" fn sun4i_clkevt_shutdown(evt: *mut clock_event_device) -> c_int {
    static int sun4i_clkevt_shutdown(struct clock_event_device *evt)
    {
    struct timer_of *to = to_timer_of(evt);
    sun4i_clkevt_time_stop(timer_of_base(to), 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_clkevt_set_oneshot(evt: *mut clock_event_device) -> c_int {
    static int sun4i_clkevt_set_oneshot(struct clock_event_device *evt)
    {
    struct timer_of *to = to_timer_of(evt);
    sun4i_clkevt_time_stop(timer_of_base(to), 0);
    sun4i_clkevt_time_start(timer_of_base(to), 0, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_clkevt_set_periodic(evt: *mut clock_event_device) -> c_int {
    static int sun4i_clkevt_set_periodic(struct clock_event_device *evt)
    {
    struct timer_of *to = to_timer_of(evt);
    sun4i_clkevt_time_stop(timer_of_base(to), 0);
    sun4i_clkevt_time_setup(timer_of_base(to), 0, timer_of_period(to));
    sun4i_clkevt_time_start(timer_of_base(to), 0, true);
    return 0;
    }
    static int sun4i_clkevt_next_event(unsigned long evt,
    struct clock_event_device *clkevt)
    {
    struct timer_of *to = to_timer_of(clkevt);
    sun4i_clkevt_time_stop(timer_of_base(to), 0);
    sun4i_clkevt_time_setup(timer_of_base(to), 0, evt - TIMER_SYNC_TICKS);
    sun4i_clkevt_time_start(timer_of_base(to), 0, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_timer_clear_interrupt(base: *mut void __iomem) {
    static void sun4i_timer_clear_interrupt(void __iomem *base)
    {
    writel(TIMER_IRQ_CLEAR(0), base + TIMER_IRQ_ST_REG);
    }
#[no_mangle]
unsafe extern "C" fn sun4i_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sun4i_timer_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *evt = dev_id;
    struct timer_of *to = to_timer_of(evt);
    sun4i_timer_clear_interrupt(timer_of_base(to));
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
    static struct timer_of to = {
    .flags = TIMER_OF_IRQ | TIMER_OF_CLOCK | TIMER_OF_BASE,
    .clkevt = {
    .name = "sun4i_tick",
    .rating = 350,
    .features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT |
    CLOCK_EVT_FEAT_DYNIRQ,
    .set_state_shutdown = sun4i_clkevt_shutdown,
    .set_state_periodic = sun4i_clkevt_set_periodic,
    .set_state_oneshot = sun4i_clkevt_set_oneshot,
    .tick_resume = sun4i_clkevt_shutdown,
    .set_next_event = sun4i_clkevt_next_event,
    .cpumask = cpu_possible_mask,
    },
    .of_irq = {
    .handler = sun4i_timer_interrupt,
    .flags = IRQF_TIMER | IRQF_IRQPOLL,
    },
    };
#[no_mangle]
unsafe extern "C" fn sun4i_timer_sched_read() -> u64 notrace {
    static u64 notrace sun4i_timer_sched_read(void)
    {
    return ~readl(timer_of_base(&to) + TIMER_CNTVAL_REG(1));
    }
#[no_mangle]
unsafe extern "C" fn sun4i_timer_init(node: *mut device_node) -> int __init {
    static int __init sun4i_timer_init(struct device_node *node)
    {
    int ret;
    u32 val;
    ret = timer_of_init(node, &to);
    if (ret)
    return ret;
    writel(~0, timer_of_base(&to) + TIMER_INTVAL_REG(1));
    writel(TIMER_CTL_ENABLE | TIMER_CTL_RELOAD |
    TIMER_CTL_CLK_SRC(TIMER_CTL_CLK_SRC_OSC24M),
    timer_of_base(&to) + TIMER_CTL_REG(1));
//
// sched_clock_register does not have priorities, and on sun6i and
// later there is a better sched_clock registered by arm_arch_timer.c
//
    if (of_machine_is_compatible("allwinner,sun4i-a10") ||
    of_machine_is_compatible("allwinner,sun5i-a13") ||
    of_machine_is_compatible("allwinner,sun5i-a10s") ||
    of_machine_is_compatible("allwinner,suniv-f1c100s"))
    sched_clock_register(sun4i_timer_sched_read, 32,
    timer_of_rate(&to));
    ret = clocksource_mmio_init(timer_of_base(&to) + TIMER_CNTVAL_REG(1),
    node.name, timer_of_rate(&to), 350, 32,
    clocksource_mmio_readl_down);
    if (ret) {
    pr_err("Failed to register clocksource\n");
    return ret;
    }
    writel(TIMER_CTL_CLK_SRC(TIMER_CTL_CLK_SRC_OSC24M),
    timer_of_base(&to) + TIMER_CTL_REG(0));
// Make sure timer is stopped before playing with interrupts
    sun4i_clkevt_time_stop(timer_of_base(&to), 0);
// clear timer0 interrupt
    sun4i_timer_clear_interrupt(timer_of_base(&to));
    clockevents_config_and_register(&to.clkevt, timer_of_rate(&to),
    TIMER_SYNC_TICKS + 1, 0xffffffff);
// Enable timer0 interrupt
    val = readl(timer_of_base(&to) + TIMER_IRQ_EN_REG);
    writel(val | TIMER_IRQ_EN(0), timer_of_base(&to) + TIMER_IRQ_EN_REG);
    return ret;
    }
    TIMER_OF_DECLARE(sun4i, "allwinner,sun4i-a10-timer",
    sun4i_timer_init);
    TIMER_OF_DECLARE(sun8i_a23, "allwinner,sun8i-a23-timer",
    sun4i_timer_init);
    TIMER_OF_DECLARE(sun8i_v3s, "allwinner,sun8i-v3s-timer",
    sun4i_timer_init);
    TIMER_OF_DECLARE(suniv, "allwinner,suniv-f1c100s-timer",
    sun4i_timer_init);
