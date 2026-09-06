//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-msc313e.c
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
// MStar timer driver
//
// Copyright (C) 2021 Daniel Palmer
// Copyright (C) 2021 Romain Perier
//

pub const MSC313E_REG_CTRL: c_uint = 0x00;

pub const MSC313E_REG_TIMER_MAX_LOW: c_uint = 0x08;
pub const MSC313E_REG_TIMER_MAX_HIGH: c_uint = 0x0c;
pub const MSC313E_REG_COUNTER_LOW: c_uint = 0x10;
pub const MSC313E_REG_COUNTER_HIGH: c_uint = 0x14;
pub const MSC313E_REG_TIMER_DIVIDE: c_uint = 0x18;
pub const MSC313E_CLK_DIVIDER: c_int = 9;
pub const TIMER_SYNC_TICKS: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msc313e_delay {
    pub base: *mut void __iomem,
    pub delay: delay_timer,
}

    static struct msc313e_delay msc313e_delay;

    static void __iomem *msc313e_clksrc;
#[no_mangle]
unsafe extern "C" fn msc313e_timer_stop(base: *mut void __iomem) {
    static void msc313e_timer_stop(void __iomem *base)
    {
    writew(0, base + MSC313E_REG_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn msc313e_timer_start(base: *mut void __iomem, periodic: bool) {
    static void msc313e_timer_start(void __iomem *base, bool periodic)
    {
    u16 reg;
    reg = readw(base + MSC313E_REG_CTRL);
    if (periodic)
    reg |= MSC313E_REG_CTRL_TIMER_EN;
    else
    reg |= MSC313E_REG_CTRL_TIMER_TRIG;
    writew(reg | MSC313E_REG_CTRL_TIMER_INT_EN, base + MSC313E_REG_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn msc313e_timer_setup(base: *mut void __iomem, delay: c_ulong) {
    static void msc313e_timer_setup(void __iomem *base, unsigned long delay)
    {
    unsigned long flags;
    local_irq_save(flags);
    writew(delay >> 16, base + MSC313E_REG_TIMER_MAX_HIGH);
    writew(delay & 0xffff, base + MSC313E_REG_TIMER_MAX_LOW);
    local_irq_restore(flags);
    }
#[no_mangle]
unsafe extern "C" fn msc313e_timer_current_value(base: *mut void __iomem) -> c_ulong {
    static unsigned long msc313e_timer_current_value(void __iomem *base)
    {
    unsigned long flags;
    u16 l, h;
    local_irq_save(flags);
    l = readw(base + MSC313E_REG_COUNTER_LOW);
    h = readw(base + MSC313E_REG_COUNTER_HIGH);
    local_irq_restore(flags);
    return (((u32)h) << 16 | l);
    }
#[no_mangle]
unsafe extern "C" fn msc313e_timer_clkevt_shutdown(evt: *mut clock_event_device) -> c_int {
    static int msc313e_timer_clkevt_shutdown(struct clock_event_device *evt)
    {
    struct timer_of *timer = to_timer_of(evt);
    msc313e_timer_stop(timer_of_base(timer));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msc313e_timer_clkevt_set_oneshot(evt: *mut clock_event_device) -> c_int {
    static int msc313e_timer_clkevt_set_oneshot(struct clock_event_device *evt)
    {
    struct timer_of *timer = to_timer_of(evt);
    msc313e_timer_stop(timer_of_base(timer));
    msc313e_timer_start(timer_of_base(timer), false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msc313e_timer_clkevt_set_periodic(evt: *mut clock_event_device) -> c_int {
    static int msc313e_timer_clkevt_set_periodic(struct clock_event_device *evt)
    {
    struct timer_of *timer = to_timer_of(evt);
    msc313e_timer_stop(timer_of_base(timer));
    msc313e_timer_setup(timer_of_base(timer), timer_of_period(timer));
    msc313e_timer_start(timer_of_base(timer), true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msc313e_timer_clkevt_next_event(evt: c_ulong, clkevt: *mut clock_event_device) -> c_int {
    static int msc313e_timer_clkevt_next_event(unsigned long evt, struct clock_event_device *clkevt)
    {
    struct timer_of *timer = to_timer_of(clkevt);
    msc313e_timer_stop(timer_of_base(timer));
    msc313e_timer_setup(timer_of_base(timer), evt);
    msc313e_timer_start(timer_of_base(timer), false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msc313e_timer_clkevt_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t msc313e_timer_clkevt_irq(int irq, void *dev_id)
    {
    struct clock_event_device *evt = dev_id;
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn msc313e_timer_clksrc_read(cs: *mut clocksource) -> u64 {
    static u64 msc313e_timer_clksrc_read(struct clocksource *cs)
    {
    return msc313e_timer_current_value(msc313e_clksrc) & cs.mask;
    }

#[no_mangle]
unsafe extern "C" fn msc313e_read_delay_timer_read() -> c_ulong {
    static unsigned long msc313e_read_delay_timer_read(void)
    {
    return msc313e_timer_current_value(msc313e_delay.base);
    }

#[no_mangle]
unsafe extern "C" fn msc313e_timer_sched_clock_read() -> u64 {
    static u64 msc313e_timer_sched_clock_read(void)
    {
    return msc313e_timer_current_value(msc313e_clksrc);
    }
    static struct clock_event_device msc313e_clkevt = {
    .name = TIMER_NAME,
    .rating = 300,
    .features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT,
    .set_state_shutdown = msc313e_timer_clkevt_shutdown,
    .set_state_periodic = msc313e_timer_clkevt_set_periodic,
    .set_state_oneshot = msc313e_timer_clkevt_set_oneshot,
    .tick_resume = msc313e_timer_clkevt_shutdown,
    .set_next_event = msc313e_timer_clkevt_next_event,
    };
#[no_mangle]
unsafe extern "C" fn msc313e_clkevt_init(np: *mut device_node) -> int __init {
    static int __init msc313e_clkevt_init(struct device_node *np)
    {
    int ret;
    struct timer_of *to;
    to = kzalloc_obj(*to);
    if (!to)
    return -ENOMEM;
    to.flags = TIMER_OF_IRQ | TIMER_OF_CLOCK | TIMER_OF_BASE;
    to.of_irq.handler = msc313e_timer_clkevt_irq;
    ret = timer_of_init(np, to);
    if (ret)
    return ret;
    if (of_device_is_compatible(np, "sstar,ssd20xd-timer")) {
    to.of_clk.rate = clk_get_rate(to.of_clk.clk) / MSC313E_CLK_DIVIDER;
    to.of_clk.period = DIV_ROUND_UP(to.of_clk.rate, HZ);
    writew(MSC313E_CLK_DIVIDER - 1, timer_of_base(to) + MSC313E_REG_TIMER_DIVIDE);
    }
    msc313e_clkevt.cpumask = cpu_possible_mask;
    msc313e_clkevt.irq = to.of_irq.irq;
    to.clkevt = msc313e_clkevt;
    clockevents_config_and_register(&to.clkevt, timer_of_rate(to),
    TIMER_SYNC_TICKS, 0xffffffff);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msc313e_clksrc_init(np: *mut device_node) -> int __init {
    static int __init msc313e_clksrc_init(struct device_node *np)
    {
    let mut to: timer_of = { 0 };
    int ret;
    u16 reg;
    to.flags = TIMER_OF_BASE | TIMER_OF_CLOCK;
    ret = timer_of_init(np, &to);
    if (ret)
    return ret;
    msc313e_clksrc = timer_of_base(&to);
    reg = readw(msc313e_clksrc + MSC313E_REG_CTRL);
    reg |= MSC313E_REG_CTRL_TIMER_EN;
    writew(reg, msc313e_clksrc + MSC313E_REG_CTRL);

    msc313e_delay.base = timer_of_base(&to);
    msc313e_delay.delay.read_current_timer = msc313e_read_delay_timer_read;
    msc313e_delay.delay.freq = timer_of_rate(&to);
    register_current_timer_delay(&msc313e_delay.delay);

    sched_clock_register(msc313e_timer_sched_clock_read, 32, timer_of_rate(&to));
    return clocksource_mmio_init(timer_of_base(&to), TIMER_NAME, timer_of_rate(&to), 300, 32,
    msc313e_timer_clksrc_read);
    }
#[no_mangle]
unsafe extern "C" fn msc313e_timer_init(np: *mut device_node) -> int __init {
    static int __init msc313e_timer_init(struct device_node *np)
    {
    let mut ret: c_int = 0;
    static int num_called;
    switch (num_called) {
    case 0:
    ret = msc313e_clksrc_init(np);
    if (ret)
    return ret;
    break;
    default:
    ret = msc313e_clkevt_init(np);
    if (ret)
    return ret;
    break;
    }
    num_called++;
    return 0;
    }
    TIMER_OF_DECLARE(msc313, "mstar,msc313e-timer", msc313e_timer_init);
    TIMER_OF_DECLARE(ssd20xd, "sstar,ssd20xd-timer", msc313e_timer_init);
