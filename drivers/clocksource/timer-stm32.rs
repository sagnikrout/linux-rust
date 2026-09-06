//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-stm32.c
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
// Copyright (C) Maxime Coquelin 2015
// Author:  Maxime Coquelin <mcoquelin.stm32@gmail.com>
//
// Inspired by time-efm32.c from Uwe Kleine-Koenig
//

pub const TIM_CR1: c_uint = 0x00;
pub const TIM_DIER: c_uint = 0x0c;
pub const TIM_SR: c_uint = 0x10;
pub const TIM_EGR: c_uint = 0x14;
pub const TIM_CNT: c_uint = 0x24;
pub const TIM_PSC: c_uint = 0x28;
pub const TIM_ARR: c_uint = 0x2c;
pub const TIM_CCR1: c_uint = 0x34;

pub const TIM_PSC_CLKRATE: c_int = 10000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm32_timer_private {
    pub bits: c_int,
}

//
// stm32_timer_of_bits_set - set accessor helper
// @to: a timer_of structure pointer
// @bits: the number of bits (16 or 32)
//
// Accessor helper to set the number of bits in the timer-of private
// structure.
//
#[no_mangle]
unsafe extern "C" fn stm32_timer_of_bits_set(to: *mut timer_of, bits: c_int) {
    static void stm32_timer_of_bits_set(struct timer_of *to, int bits)
    {
    struct stm32_timer_private *pd = to.private_data;
    pd.bits = bits;
    }
//
// stm32_timer_of_bits_get - get accessor helper
// @to: a timer_of structure pointer
//
// Accessor helper to get the number of bits in the timer-of private
// structure.
//
// Returns: an integer corresponding to the number of bits.
//
#[no_mangle]
unsafe extern "C" fn stm32_timer_of_bits_get(to: *mut timer_of) -> c_int {
    static int stm32_timer_of_bits_get(struct timer_of *to)
    {
    struct stm32_timer_private *pd = to.private_data;
    return pd.bits;
    }
    static void __iomem *stm32_timer_cnt __read_mostly;
#[no_mangle]
unsafe extern "C" fn stm32_read_sched_clock() -> u64 notrace {
    static u64 notrace stm32_read_sched_clock(void)
    {
    return readl_relaxed(stm32_timer_cnt);
    }
    static struct delay_timer stm32_timer_delay;
#[no_mangle]
unsafe extern "C" fn stm32_read_delay() -> c_ulong {
    static unsigned long stm32_read_delay(void)
    {
    return readl_relaxed(stm32_timer_cnt);
    }
#[no_mangle]
unsafe extern "C" fn stm32_clock_event_disable(to: *mut timer_of) {
    static void stm32_clock_event_disable(struct timer_of *to)
    {
    writel_relaxed(0, timer_of_base(to) + TIM_DIER);
    }
//
// stm32_timer_start - Start the counter without event
// @to: a timer_of structure pointer
//
// Start the timer in order to have the counter reset and start
// incrementing but disable interrupt event when there is a counter
// overflow. By default, the counter direction is used as upcounter.
//
#[no_mangle]
unsafe extern "C" fn stm32_timer_start(to: *mut timer_of) {
    static void stm32_timer_start(struct timer_of *to)
    {
    writel_relaxed(TIM_CR1_UDIS | TIM_CR1_CEN, timer_of_base(to) + TIM_CR1);
    }
#[no_mangle]
unsafe extern "C" fn stm32_clock_event_shutdown(clkevt: *mut clock_event_device) -> c_int {
    static int stm32_clock_event_shutdown(struct clock_event_device *clkevt)
    {
    struct timer_of *to = to_timer_of(clkevt);
    stm32_clock_event_disable(to);
    return 0;
    }
    static int stm32_clock_event_set_next_event(unsigned long evt,
    struct clock_event_device *clkevt)
    {
    struct timer_of *to = to_timer_of(clkevt);
    unsigned long now, next;
    next = readl_relaxed(timer_of_base(to) + TIM_CNT) + evt;
    writel_relaxed(next, timer_of_base(to) + TIM_CCR1);
    now = readl_relaxed(timer_of_base(to) + TIM_CNT);
    if ((next - now) > evt)
    return -ETIME;
    writel_relaxed(TIM_DIER_CC1IE, timer_of_base(to) + TIM_DIER);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_clock_event_set_periodic(clkevt: *mut clock_event_device) -> c_int {
    static int stm32_clock_event_set_periodic(struct clock_event_device *clkevt)
    {
    struct timer_of *to = to_timer_of(clkevt);
    stm32_timer_start(to);
    return stm32_clock_event_set_next_event(timer_of_period(to), clkevt);
    }
#[no_mangle]
unsafe extern "C" fn stm32_clock_event_set_oneshot(clkevt: *mut clock_event_device) -> c_int {
    static int stm32_clock_event_set_oneshot(struct clock_event_device *clkevt)
    {
    struct timer_of *to = to_timer_of(clkevt);
    stm32_timer_start(to);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm32_clock_event_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t stm32_clock_event_handler(int irq, void *dev_id)
    {
    struct clock_event_device *clkevt = (struct clock_event_device *)dev_id;
    struct timer_of *to = to_timer_of(clkevt);
    writel_relaxed(0, timer_of_base(to) + TIM_SR);
    if (clockevent_state_periodic(clkevt))
    stm32_clock_event_set_periodic(clkevt);
    else
    stm32_clock_event_shutdown(clkevt);
    clkevt.event_handler(clkevt);
    return IRQ_HANDLED;
    }
//
// stm32_timer_set_width - Sort out the timer width (32/16)
// @to: a pointer to a timer-of structure
//
// Write the 32-bit max value and read/return the result. If the timer
// is 32 bits wide, the result will be UINT_MAX, otherwise it will
// be truncated by the 16-bit register to USHRT_MAX.
//
#[no_mangle]
unsafe extern "C" fn stm32_timer_set_width(to: *mut timer_of) -> void __init {
    static void __init stm32_timer_set_width(struct timer_of *to)
    {
    u32 width;
    writel_relaxed(UINT_MAX, timer_of_base(to) + TIM_ARR);
    width = readl_relaxed(timer_of_base(to) + TIM_ARR);
    stm32_timer_of_bits_set(to, width == UINT_MAX ? 32 : 16);
    }
//
// stm32_timer_set_prescaler - Compute and set the prescaler register
// @to: a pointer to a timer-of structure
//
// Depending on the timer width, compute the prescaler to always
// target a 10MHz timer rate for 16 bits. 32-bit timers are
// considered precise and long enough to not use the prescaler.
//
#[no_mangle]
unsafe extern "C" fn stm32_timer_set_prescaler(to: *mut timer_of) -> void __init {
    static void __init stm32_timer_set_prescaler(struct timer_of *to)
    {
    let mut prescaler: c_int = 1;
    if (stm32_timer_of_bits_get(to) != 32) {
    prescaler = DIV_ROUND_CLOSEST(timer_of_rate(to),
    TIM_PSC_CLKRATE);
//
// The prescaler register is an u16, the variable
// can't be greater than TIM_PSC_MAX, let's cap it in
// this case.
//
    prescaler = prescaler < TIM_PSC_MAX ? prescaler : TIM_PSC_MAX;
    }
    writel_relaxed(prescaler - 1, timer_of_base(to) + TIM_PSC);
    writel_relaxed(TIM_EGR_UG, timer_of_base(to) + TIM_EGR);
    writel_relaxed(0, timer_of_base(to) + TIM_SR);
// Adjust rate and period given the prescaler value
    to.of_clk.rate = DIV_ROUND_CLOSEST(to.of_clk.rate, prescaler);
    to.of_clk.period = DIV_ROUND_UP(to.of_clk.rate, HZ);
    }
#[no_mangle]
unsafe extern "C" fn stm32_clocksource_init(to: *mut timer_of) -> int __init {
    static int __init stm32_clocksource_init(struct timer_of *to)
    {
    let mut bits: u32 = stm32_timer_of_bits_get(to);
    const char *name = to.np.full_name;
//
// This driver allows to register several timers and relies on
// the generic time framework to select the right one.
// However, nothing allows to do the same for the
// sched_clock. We are not interested in a sched_clock for the
// 16-bit timers but only for the 32-bit one, so if no 32-bit
// timer is registered yet, we select this 32-bit timer as a
// sched_clock.
//
    if (bits == 32 && !stm32_timer_cnt) {
//
// Start immediately the counter as we will be using
// it right after.
//
    stm32_timer_start(to);
    stm32_timer_cnt = timer_of_base(to) + TIM_CNT;
    sched_clock_register(stm32_read_sched_clock, bits, timer_of_rate(to));
    pr_info("%s: STM32 sched_clock registered\n", name);
    stm32_timer_delay.read_current_timer = stm32_read_delay;
    stm32_timer_delay.freq = timer_of_rate(to);
    register_current_timer_delay(&stm32_timer_delay);
    pr_info("%s: STM32 delay timer registered\n", name);
    }
    return clocksource_mmio_init(timer_of_base(to) + TIM_CNT, name,
    timer_of_rate(to), bits == 32 ? 250 : 100,
    bits, clocksource_mmio_readl_up);
    }
#[no_mangle]
unsafe extern "C" fn stm32_clockevent_init(to: *mut timer_of) -> void __init {
    static void __init stm32_clockevent_init(struct timer_of *to)
    {
    let mut bits: u32 = stm32_timer_of_bits_get(to);
    to.clkevt.name = to.np.full_name;
    to.clkevt.features = CLOCK_EVT_FEAT_PERIODIC | CLOCK_EVT_FEAT_ONESHOT;
    to.clkevt.set_state_shutdown = stm32_clock_event_shutdown;
    to.clkevt.set_state_periodic = stm32_clock_event_set_periodic;
    to.clkevt.set_state_oneshot = stm32_clock_event_set_oneshot;
    to.clkevt.tick_resume = stm32_clock_event_shutdown;
    to.clkevt.set_next_event = stm32_clock_event_set_next_event;
    to.clkevt.rating = bits == 32 ? 250 : 100;
    clockevents_config_and_register(&to.clkevt, timer_of_rate(to), 0x1,
    (1 <<  bits) - 1);
    pr_info("%pOF: STM32 clockevent driver initialized (%d bits)\n",
    to.np, bits);
    }
#[no_mangle]
unsafe extern "C" fn stm32_timer_init(node: *mut device_node) -> int __init {
    static int __init stm32_timer_init(struct device_node *node)
    {
    struct reset_control *rstc;
    struct timer_of *to;
    int ret;
    to = kzalloc_obj(*to);
    if (!to)
    return -ENOMEM;
    to.flags = TIMER_OF_IRQ | TIMER_OF_CLOCK | TIMER_OF_BASE;
    to.of_irq.handler = stm32_clock_event_handler;
    ret = timer_of_init(node, to);
    if (ret)
    goto err;
    to.private_data = kzalloc_obj(struct stm32_timer_private);
    if (!to.private_data) {
    ret = -ENOMEM;
    goto deinit;
    }
    rstc = of_reset_control_get(node, core::ptr::null_mut());
    if (!IS_ERR(rstc)) {
    reset_control_assert(rstc);
    reset_control_deassert(rstc);
    }
    stm32_timer_set_width(to);
    stm32_timer_set_prescaler(to);
    ret = stm32_clocksource_init(to);
    if (ret)
    goto deinit;
    stm32_clockevent_init(to);
    return 0;
    deinit:
    timer_of_cleanup(to);
    err:
    kfree(to);
    return ret;
    }
    TIMER_OF_DECLARE(stm32, "st,stm32-timer", stm32_timer_init);
