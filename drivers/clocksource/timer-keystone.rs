//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-keystone.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Keystone broadcast clock-event
//
// Copyright 2013 Texas Instruments, Inc.
//
// Author: Ivan Khoronzhuk <ivan.khoronzhuk@ti.com>
//

// Timer register offsets
pub const TIM12: c_uint = 0x10;
pub const TIM34: c_uint = 0x14;
pub const PRD12: c_uint = 0x18;
pub const PRD34: c_uint = 0x1c;
pub const TCR: c_uint = 0x20;
pub const TGCR: c_uint = 0x24;
pub const INTCTLSTAT: c_uint = 0x44;
// Timer register bitfields
pub const TCR_ENAMODE_MASK: c_uint = 0xC0;
pub const TCR_ENAMODE_ONESHOT_MASK: c_uint = 0x40;
pub const TCR_ENAMODE_PERIODIC_MASK: c_uint = 0x80;
pub const TGCR_TIM_UNRESET_MASK: c_uint = 0x03;
pub const INTCTLSTAT_ENINT_MASK: c_uint = 0x01;
//
// struct keystone_timer: holds timer's data
// @base: timer memory base address
// @hz_period: cycles per HZ period
// @event_dev: event device based on timer
//
    static struct keystone_timer {
    void __iomem *base;
    unsigned long hz_period;
    struct clock_event_device event_dev;
    } timer;
#[no_mangle]
pub unsafe extern "C" fn keystone_timer_readl(rg: c_ulong) -> u32 {
    static inline u32 keystone_timer_readl(unsigned long rg)
    {
    return readl_relaxed(timer.base + rg);
    }
#[no_mangle]
pub unsafe extern "C" fn keystone_timer_writel(val: u32, rg: c_ulong) {
    static inline void keystone_timer_writel(u32 val, unsigned long rg)
    {
    writel_relaxed(val, timer.base + rg);
    }
//
// keystone_timer_barrier: write memory barrier
// use explicit barrier to avoid using readl/writel non relaxed function
// variants, because in our case non relaxed variants hide the true places
// where barrier is needed.
//
#[no_mangle]
pub unsafe extern "C" fn keystone_timer_barrier() {
    static inline void keystone_timer_barrier(void)
    {
    __iowmb();
    }
//
// keystone_timer_config: configures timer to work in oneshot/periodic modes.
// @ mask: mask of the mode to configure
// @ period: cycles number to configure for
//
#[no_mangle]
unsafe extern "C" fn keystone_timer_config(period: u64, mask: c_int) -> c_int {
    static int keystone_timer_config(u64 period, int mask)
    {
    u32 tcr;
    u32 off;
    tcr = keystone_timer_readl(TCR);
    off = tcr & ~(TCR_ENAMODE_MASK);
// set enable mode
    tcr |= mask;
// disable timer
    keystone_timer_writel(off, TCR);
// here we have to be sure the timer has been disabled
    keystone_timer_barrier();
// reset counter to zero, set new period
    keystone_timer_writel(0, TIM12);
    keystone_timer_writel(0, TIM34);
    keystone_timer_writel(period & 0xffffffff, PRD12);
    keystone_timer_writel(period >> 32, PRD34);
//
// enable timer
// here we have to be sure that CNTLO, CNTHI, PRDLO, PRDHI registers
// have been written.
//
    keystone_timer_barrier();
    keystone_timer_writel(tcr, TCR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn keystone_timer_disable() {
    static void keystone_timer_disable(void)
    {
    u32 tcr;
    tcr = keystone_timer_readl(TCR);
// disable timer
    tcr &= ~(TCR_ENAMODE_MASK);
    keystone_timer_writel(tcr, TCR);
    }
#[no_mangle]
unsafe extern "C" fn keystone_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t keystone_timer_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *evt = dev_id;
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
    static int keystone_set_next_event(unsigned long cycles,
    struct clock_event_device *evt)
    {
    return keystone_timer_config(cycles, TCR_ENAMODE_ONESHOT_MASK);
    }
#[no_mangle]
unsafe extern "C" fn keystone_shutdown(evt: *mut clock_event_device) -> c_int {
    static int keystone_shutdown(struct clock_event_device *evt)
    {
    keystone_timer_disable();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn keystone_set_periodic(evt: *mut clock_event_device) -> c_int {
    static int keystone_set_periodic(struct clock_event_device *evt)
    {
    keystone_timer_config(timer.hz_period, TCR_ENAMODE_PERIODIC_MASK);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn keystone_timer_init(np: *mut device_node) -> int __init {
    static int __init keystone_timer_init(struct device_node *np)
    {
    struct clock_event_device *event_dev = &timer.event_dev;
    unsigned long rate;
    struct clk *clk;
    int irq, error;
    irq  = irq_of_parse_and_map(np, 0);
    if (!irq) {
    pr_err("%s: failed to map interrupts\n", __func__);
    return -EINVAL;
    }
    timer.base = of_iomap(np, 0);
    if (!timer.base) {
    pr_err("%s: failed to map registers\n", __func__);
    return -ENXIO;
    }
    clk = of_clk_get(np, 0);
    if (IS_ERR(clk)) {
    pr_err("%s: failed to get clock\n", __func__);
    iounmap(timer.base);
    return PTR_ERR(clk);
    }
    error = clk_prepare_enable(clk);
    if (error) {
    pr_err("%s: failed to enable clock\n", __func__);
    goto err;
    }
    rate = clk_get_rate(clk);
// disable, use internal clock source
    keystone_timer_writel(0, TCR);
// here we have to be sure the timer has been disabled
    keystone_timer_barrier();
// reset timer as 64-bit, no pre-scaler, plus features are disabled
    keystone_timer_writel(0, TGCR);
// unreset timer
    keystone_timer_writel(TGCR_TIM_UNRESET_MASK, TGCR);
// init counter to zero
    keystone_timer_writel(0, TIM12);
    keystone_timer_writel(0, TIM34);
    timer.hz_period = DIV_ROUND_UP(rate, HZ);
// enable timer interrupts
    keystone_timer_writel(INTCTLSTAT_ENINT_MASK, INTCTLSTAT);
    error = request_irq(irq, keystone_timer_interrupt, IRQF_TIMER,
    TIMER_NAME, event_dev);
    if (error) {
    pr_err("%s: failed to setup irq\n", __func__);
    goto err;
    }
// setup clockevent
    event_dev.features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT;
    event_dev.set_next_event = keystone_set_next_event;
    event_dev.set_state_shutdown = keystone_shutdown;
    event_dev.set_state_periodic = keystone_set_periodic;
    event_dev.set_state_oneshot = keystone_shutdown;
    event_dev.cpumask = cpu_possible_mask;
    event_dev.owner = THIS_MODULE;
    event_dev.name = TIMER_NAME;
    event_dev.irq = irq;
    clockevents_config_and_register(event_dev, rate, 1, ULONG_MAX);
    pr_info("keystone timer clock @%lu Hz\n", rate);
    return 0;
    err:
    clk_put(clk);
    iounmap(timer.base);
    return error;
    }
    TIMER_OF_DECLARE(keystone_timer, "ti,keystone-timer",
    keystone_timer_init);
