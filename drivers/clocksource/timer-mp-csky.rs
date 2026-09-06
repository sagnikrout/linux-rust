//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-mp-csky.c
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
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

    static int csky_mptimer_irq;
    static int csky_mptimer_set_next_event(unsigned long delta,
    struct clock_event_device *ce)
    {
    mtcr(PTIM_LVR, delta);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn csky_mptimer_shutdown(ce: *mut clock_event_device) -> c_int {
    static int csky_mptimer_shutdown(struct clock_event_device *ce)
    {
    mtcr(PTIM_CTLR, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn csky_mptimer_oneshot(ce: *mut clock_event_device) -> c_int {
    static int csky_mptimer_oneshot(struct clock_event_device *ce)
    {
    mtcr(PTIM_CTLR, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn csky_mptimer_oneshot_stopped(ce: *mut clock_event_device) -> c_int {
    static int csky_mptimer_oneshot_stopped(struct clock_event_device *ce)
    {
    mtcr(PTIM_CTLR, 0);
    return 0;
    }
    static DEFINE_PER_CPU(struct timer_of, csky_to) = {
    .flags					= TIMER_OF_CLOCK,
    .clkevt = {
    .rating				= 300,
    .features			= CLOCK_EVT_FEAT_PERCPU |
    CLOCK_EVT_FEAT_ONESHOT,
    .set_state_shutdown		= csky_mptimer_shutdown,
    .set_state_oneshot		= csky_mptimer_oneshot,
    .set_state_oneshot_stopped	= csky_mptimer_oneshot_stopped,
    .set_next_event			= csky_mptimer_set_next_event,
    },
    };
#[no_mangle]
unsafe extern "C" fn csky_timer_interrupt(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t csky_timer_interrupt(int irq, void *dev)
    {
    struct timer_of *to = this_cpu_ptr(&csky_to);
    mtcr(PTIM_TSR, 0);
    to.clkevt.event_handler(&to.clkevt);
    return IRQ_HANDLED;
    }
//
// clock event for percpu
//
#[no_mangle]
unsafe extern "C" fn csky_mptimer_starting_cpu(cpu: c_uint) -> c_int {
    static int csky_mptimer_starting_cpu(unsigned int cpu)
    {
    struct timer_of *to = per_cpu_ptr(&csky_to, cpu);
    to.clkevt.cpumask = cpumask_of(cpu);
    enable_percpu_irq(csky_mptimer_irq, 0);
    clockevents_config_and_register(&to.clkevt, timer_of_rate(to),
    2, ULONG_MAX);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn csky_mptimer_dying_cpu(cpu: c_uint) -> c_int {
    static int csky_mptimer_dying_cpu(unsigned int cpu)
    {
    disable_percpu_irq(csky_mptimer_irq);
    return 0;
    }
//
// clock source
//
#[no_mangle]
unsafe extern "C" fn sched_clock_read() -> u64 notrace {
    static u64 notrace sched_clock_read(void)
    {
    return (u64)mfcr(PTIM_CCVR);
    }
#[no_mangle]
unsafe extern "C" fn clksrc_read(c: *mut clocksource) -> u64 {
    static u64 clksrc_read(struct clocksource *c)
    {
    return (u64)mfcr(PTIM_CCVR);
    }
    struct clocksource csky_clocksource = {
    .name	= "csky",
    .rating	= 400,
    .mask	= CLOCKSOURCE_MASK(32),
    .flags	= CLOCK_SOURCE_IS_CONTINUOUS,
    .read	= clksrc_read,
    };
#[no_mangle]
unsafe extern "C" fn csky_mptimer_init(np: *mut device_node) -> int __init {
    static int __init csky_mptimer_init(struct device_node *np)
    {
    int ret, cpu, cpu_rollback;
    struct timer_of *to = core::ptr::null_mut();
//
// Csky_mptimer is designed for C-SKY SMP multi-processors and
// every core has it's own private irq and regs for clkevt and
// clksrc.
//
// The regs is accessed by cpu instruction: mfcr/mtcr instead of
// mmio map style. So we needn't mmio-address in dts, but we still
// need to give clk and irq number.
//
// We use private irq for the mptimer and irq number is the same
// for every core. So we use request_percpu_irq() in timer_of_init.
//
    csky_mptimer_irq = irq_of_parse_and_map(np, 0);
    if (csky_mptimer_irq <= 0)
    return -EINVAL;
    ret = request_percpu_irq(csky_mptimer_irq, csky_timer_interrupt,
    "csky_mp_timer", &csky_to);
    if (ret)
    return -EINVAL;
    for_each_possible_cpu(cpu) {
    to = per_cpu_ptr(&csky_to, cpu);
    ret = timer_of_init(np, to);
    if (ret)
    goto rollback;
    }
    clocksource_register_hz(&csky_clocksource, timer_of_rate(to));
    sched_clock_register(sched_clock_read, 32, timer_of_rate(to));
    ret = cpuhp_setup_state(CPUHP_AP_CSKY_TIMER_STARTING,
    "clockevents/csky/timer:starting",
    csky_mptimer_starting_cpu,
    csky_mptimer_dying_cpu);
    if (ret)
    return -EINVAL;
    return 0;
    rollback:
    for_each_possible_cpu(cpu_rollback) {
    if (cpu_rollback == cpu)
    break;
    to = per_cpu_ptr(&csky_to, cpu_rollback);
    timer_of_cleanup(to);
    }
    return -EINVAL;
    }
    TIMER_OF_DECLARE(csky_mptimer, "csky,mptimer", csky_mptimer_init);
