//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-ep93xx.c
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
// Cirrus Logic EP93xx timer driver.
// Copyright (C) 2021 Nikita Shubin <nikita.shubin@maquefel.me>
//
// Based on a rewrite of arch/arm/mach-ep93xx/timer.c:
//

//
// Timer handling for EP93xx
//
// The ep93xx has four internal timers.  Timers 1, 2 (both 16 bit) and
// 3 (32 bit) count down at 508 kHz, are self-reloading, and can generate
// an interrupt on underflow.  Timer 4 (40 bit) counts down at 983.04 kHz,
// is free-running, and can't generate interrupts.
//
// The 508 kHz timers are ideal for use for the timer interrupt, as the
// most common values of HZ divide 508 kHz nicely.  We pick the 32 bit
// timer (timer 3) to get as long sleep intervals as possible when using
// CONFIG_NO_HZ.
//
// The higher clock rate of timer 4 makes it a better choice than the
// other timers for use as clock source and for sched_clock(), providing
// a stable 40 bit time base.
//
pub const EP93XX_TIMER1_LOAD: c_uint = 0x00;
pub const EP93XX_TIMER1_VALUE: c_uint = 0x04;
pub const EP93XX_TIMER1_CONTROL: c_uint = 0x08;

pub const EP93XX_TIMER1_CLEAR: c_uint = 0x0c;
pub const EP93XX_TIMER2_LOAD: c_uint = 0x20;
pub const EP93XX_TIMER2_VALUE: c_uint = 0x24;
pub const EP93XX_TIMER2_CONTROL: c_uint = 0x28;
pub const EP93XX_TIMER2_CLEAR: c_uint = 0x2c;
//
// This read-only register contains the low word of the time stamp debug timer
// ( Timer4). When this register is read, the high byte of the Timer4 counter is
// saved in the Timer4ValueHigh register.
//
pub const EP93XX_TIMER4_VALUE_LOW: c_uint = 0x60;
pub const EP93XX_TIMER4_VALUE_HIGH: c_uint = 0x64;

pub const EP93XX_TIMER3_LOAD: c_uint = 0x80;
pub const EP93XX_TIMER3_VALUE: c_uint = 0x84;
pub const EP93XX_TIMER3_CONTROL: c_uint = 0x88;
pub const EP93XX_TIMER3_CLEAR: c_uint = 0x8c;
pub const EP93XX_TIMER123_RATE: c_int = 508469;
pub const EP93XX_TIMER4_RATE: c_int = 983040;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep93xx_tcu {
    pub base: *mut void __iomem,
}

    static struct ep93xx_tcu *ep93xx_tcu;
#[no_mangle]
unsafe extern "C" fn ep93xx_clocksource_read(c: *mut clocksource) -> u64 {
    static u64 ep93xx_clocksource_read(struct clocksource *c)
    {
    struct ep93xx_tcu *tcu = ep93xx_tcu;
    return lo_hi_readq(tcu.base + EP93XX_TIMER4_VALUE_LOW) & GENMASK_ULL(39, 0);
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_read_sched_clock() -> u64 notrace {
    static u64 notrace ep93xx_read_sched_clock(void)
    {
    return ep93xx_clocksource_read(core::ptr::null_mut());
    }
    static int ep93xx_clkevt_set_next_event(unsigned long next,
    struct clock_event_device *evt)
    {
    struct ep93xx_tcu *tcu = ep93xx_tcu;
// Default mode: periodic, off, 508 kHz
    u32 tmode = EP93XX_TIMER123_CONTROL_MODE |
    EP93XX_TIMER123_CONTROL_CLKSEL;
// Clear timer
    writel(tmode, tcu.base + EP93XX_TIMER3_CONTROL);
// Set next event
    writel(next, tcu.base + EP93XX_TIMER3_LOAD);
    writel(tmode | EP93XX_TIMER123_CONTROL_ENABLE,
    tcu.base + EP93XX_TIMER3_CONTROL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_clkevt_shutdown(evt: *mut clock_event_device) -> c_int {
    static int ep93xx_clkevt_shutdown(struct clock_event_device *evt)
    {
    struct ep93xx_tcu *tcu = ep93xx_tcu;
// Disable timer
    writel(0, tcu.base + EP93XX_TIMER3_CONTROL);
    return 0;
    }
    static struct clock_event_device ep93xx_clockevent = {
    .name			= "timer1",
    .features		= CLOCK_EVT_FEAT_ONESHOT,
    .set_state_shutdown	= ep93xx_clkevt_shutdown,
    .set_state_oneshot	= ep93xx_clkevt_shutdown,
    .tick_resume		= ep93xx_clkevt_shutdown,
    .set_next_event		= ep93xx_clkevt_set_next_event,
    .rating			= 300,
    };
#[no_mangle]
unsafe extern "C" fn ep93xx_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ep93xx_timer_interrupt(int irq, void *dev_id)
    {
    struct ep93xx_tcu *tcu = ep93xx_tcu;
    struct clock_event_device *evt = dev_id;
// Writing any value clears the timer interrupt
    writel(1, tcu.base + EP93XX_TIMER3_CLEAR);
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ep93xx_timer_of_init(np: *mut device_node) -> int __init {
    static int __init ep93xx_timer_of_init(struct device_node *np)
    {
    int irq;
    let mut flags: c_ulong = IRQF_TIMER | IRQF_IRQPOLL;
    struct ep93xx_tcu *tcu;
    int ret;
    tcu = kzalloc_obj(*tcu);
    if (!tcu)
    return -ENOMEM;
    tcu.base = of_iomap(np, 0);
    if (!tcu.base) {
    pr_err("Can't remap registers\n");
    ret = -ENXIO;
    goto out_free;
    }
    ep93xx_tcu = tcu;
    irq = irq_of_parse_and_map(np, 0);
    if (!irq) {
    ret = -EINVAL;
    pr_err("EP93XX Timer Can't parse IRQ %d", irq);
    goto out_free;
    }
// Enable and register clocksource and sched_clock on timer 4
    writel(EP93XX_TIMER4_VALUE_HIGH_ENABLE,
    tcu.base + EP93XX_TIMER4_VALUE_HIGH);
    clocksource_mmio_init(core::ptr::null_mut(), "timer4",
    EP93XX_TIMER4_RATE, 200, 40,
    ep93xx_clocksource_read);
    sched_clock_register(ep93xx_read_sched_clock, 40,
    EP93XX_TIMER4_RATE);
// Set up clockevent on timer 3
    if (request_irq(irq, ep93xx_timer_interrupt, flags, "ep93xx timer",
    &ep93xx_clockevent))
    pr_err("Failed to request irq %d (ep93xx timer)\n", irq);
    clockevents_config_and_register(&ep93xx_clockevent,
    EP93XX_TIMER123_RATE,
    1,
    UINT_MAX);
    return 0;
    out_free:
    kfree(tcu);
    return ret;
    }
    TIMER_OF_DECLARE(ep93xx_timer, "cirrus,ep9301-timer", ep93xx_timer_of_init);
