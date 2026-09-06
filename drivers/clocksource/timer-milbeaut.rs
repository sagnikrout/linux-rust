//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-milbeaut.c
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
// Copyright (C) 2018 Socionext Inc.
//

pub const MLB_TMR_TMCSR_OFS: c_uint = 0x0;
pub const MLB_TMR_TMR_OFS: c_uint = 0x4;
pub const MLB_TMR_TMRLR1_OFS: c_uint = 0x8;
pub const MLB_TMR_TMRLR2_OFS: c_uint = 0xc;
pub const MLB_TMR_REGSZPCH: c_uint = 0x10;

pub const MLB_TMR_TMCSR_CSL_DIV2: c_int = 0;
pub const MLB_TMR_DIV_CNT: c_int = 2;
pub const MLB_TMR_SRC_CH: c_int = 1;
pub const MLB_TMR_EVT_CH: c_int = 0;

pub const MLB_TIMER_RATING: c_int = 500;
pub const MLB_TIMER_ONESHOT: c_int = 0;
pub const MLB_TIMER_PERIODIC: c_int = 1;
#[no_mangle]
unsafe extern "C" fn mlb_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mlb_timer_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *clk = dev_id;
    struct timer_of *to = to_timer_of(clk);
    u32 val;
    val = readl_relaxed(timer_of_base(to) + MLB_TMR_EVT_TMCSR_OFS);
    val &= ~MLB_TMR_TMCSR_UF;
    writel_relaxed(val, timer_of_base(to) + MLB_TMR_EVT_TMCSR_OFS);
    clk.event_handler(clk);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mlb_evt_timer_start(to: *mut timer_of, periodic: bool) {
    static void mlb_evt_timer_start(struct timer_of *to, bool periodic)
    {
    let mut val: u32 = MLB_TMR_TMCSR_CSL_DIV2;
    val |= MLB_TMR_TMCSR_CNTE | MLB_TMR_TMCSR_TRG | MLB_TMR_TMCSR_INTE;
    if (periodic)
    val |= MLB_TMR_TMCSR_RELD;
    writel_relaxed(val, timer_of_base(to) + MLB_TMR_EVT_TMCSR_OFS);
    }
#[no_mangle]
unsafe extern "C" fn mlb_evt_timer_stop(to: *mut timer_of) {
    static void mlb_evt_timer_stop(struct timer_of *to)
    {
    let mut val: u32 = readl_relaxed(timer_of_base(to) + MLB_TMR_EVT_TMCSR_OFS);
    val &= ~MLB_TMR_TMCSR_CNTE;
    writel_relaxed(val, timer_of_base(to) + MLB_TMR_EVT_TMCSR_OFS);
    }
#[no_mangle]
unsafe extern "C" fn mlb_evt_timer_register_count(to: *mut timer_of, cnt: c_ulong) {
    static void mlb_evt_timer_register_count(struct timer_of *to, unsigned long cnt)
    {
    writel_relaxed(cnt, timer_of_base(to) + MLB_TMR_EVT_TMRLR1_OFS);
    }
#[no_mangle]
unsafe extern "C" fn mlb_set_state_periodic(clk: *mut clock_event_device) -> c_int {
    static int mlb_set_state_periodic(struct clock_event_device *clk)
    {
    struct timer_of *to = to_timer_of(clk);
    mlb_evt_timer_stop(to);
    mlb_evt_timer_register_count(to, to.of_clk.period);
    mlb_evt_timer_start(to, MLB_TIMER_PERIODIC);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mlb_set_state_oneshot(clk: *mut clock_event_device) -> c_int {
    static int mlb_set_state_oneshot(struct clock_event_device *clk)
    {
    struct timer_of *to = to_timer_of(clk);
    mlb_evt_timer_stop(to);
    mlb_evt_timer_start(to, MLB_TIMER_ONESHOT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mlb_set_state_shutdown(clk: *mut clock_event_device) -> c_int {
    static int mlb_set_state_shutdown(struct clock_event_device *clk)
    {
    struct timer_of *to = to_timer_of(clk);
    mlb_evt_timer_stop(to);
    return 0;
    }
    static int mlb_clkevt_next_event(unsigned long event,
    struct clock_event_device *clk)
    {
    struct timer_of *to = to_timer_of(clk);
    mlb_evt_timer_stop(to);
    mlb_evt_timer_register_count(to, event);
    mlb_evt_timer_start(to, MLB_TIMER_ONESHOT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mlb_config_clock_source(to: *mut timer_of) -> c_int {
    static int mlb_config_clock_source(struct timer_of *to)
    {
    let mut val: u32 = MLB_TMR_TMCSR_CSL_DIV2;
    writel_relaxed(val, timer_of_base(to) + MLB_TMR_SRC_TMCSR_OFS);
    writel_relaxed(~0, timer_of_base(to) + MLB_TMR_SRC_TMRLR1_OFS);
    writel_relaxed(~0, timer_of_base(to) + MLB_TMR_SRC_TMRLR2_OFS);
    val |= MLB_TMR_TMCSR_RELD | MLB_TMR_TMCSR_CNTE | MLB_TMR_TMCSR_TRG;
    writel_relaxed(val, timer_of_base(to) + MLB_TMR_SRC_TMCSR_OFS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mlb_config_clock_event(to: *mut timer_of) -> c_int {
    static int mlb_config_clock_event(struct timer_of *to)
    {
    writel_relaxed(0, timer_of_base(to) + MLB_TMR_EVT_TMCSR_OFS);
    return 0;
    }
    static struct timer_of to = {
    .flags = TIMER_OF_IRQ | TIMER_OF_BASE | TIMER_OF_CLOCK,
    .clkevt = {
    .name = "mlb-clkevt",
    .rating = MLB_TIMER_RATING,
    .cpumask = cpu_possible_mask,
    .features = CLOCK_EVT_FEAT_DYNIRQ | CLOCK_EVT_FEAT_ONESHOT,
    .set_state_oneshot = mlb_set_state_oneshot,
    .set_state_periodic = mlb_set_state_periodic,
    .set_state_shutdown = mlb_set_state_shutdown,
    .set_next_event = mlb_clkevt_next_event,
    },
    .of_irq = {
    .flags = IRQF_TIMER | IRQF_IRQPOLL,
    .handler = mlb_timer_interrupt,
    },
    };
#[no_mangle]
unsafe extern "C" fn mlb_timer_sched_read() -> u64 notrace {
    static u64 notrace mlb_timer_sched_read(void)
    {
    return ~readl_relaxed(timer_of_base(&to) + MLB_TMR_SRC_TMR_OFS);
    }
#[no_mangle]
unsafe extern "C" fn mlb_timer_init(node: *mut device_node) -> int __init {
    static int __init mlb_timer_init(struct device_node *node)
    {
    int ret;
    unsigned long rate;
    ret = timer_of_init(node, &to);
    if (ret)
    return ret;
    rate = timer_of_rate(&to) / MLB_TMR_DIV_CNT;
    mlb_config_clock_source(&to);
    clocksource_mmio_init(timer_of_base(&to) + MLB_TMR_SRC_TMR_OFS,
    node.name, rate, MLB_TIMER_RATING, 32,
    clocksource_mmio_readl_down);
    sched_clock_register(mlb_timer_sched_read, 32, rate);
    mlb_config_clock_event(&to);
    clockevents_config_and_register(&to.clkevt, timer_of_rate(&to), 15,
    0xffffffff);
    return 0;
    }
    TIMER_OF_DECLARE(mlb_peritimer, "socionext,milbeaut-timer",
    mlb_timer_init);
