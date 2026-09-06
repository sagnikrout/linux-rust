//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/bcm_kona_timer.c
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
// Copyright (C) 2012 Broadcom Corporation

pub const KONA_GPTIMER_STCS_OFFSET: c_uint = 0x00000000;
pub const KONA_GPTIMER_STCLO_OFFSET: c_uint = 0x00000004;
pub const KONA_GPTIMER_STCHI_OFFSET: c_uint = 0x00000008;
pub const KONA_GPTIMER_STCM0_OFFSET: c_uint = 0x0000000C;
pub const KONA_GPTIMER_STCS_TIMER_MATCH_SHIFT: c_int = 0;
pub const KONA_GPTIMER_STCS_COMPARE_ENABLE_SHIFT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kona_bcm_timers {
    pub tmr_irq: c_int,
    pub tmr_regs: *mut void __iomem,
}

    static struct kona_bcm_timers timers;
    static u32 arch_timer_rate;
//
// We use the peripheral timers for system tick, the cpu global timer for
// profile tick
//
#[no_mangle]
unsafe extern "C" fn kona_timer_disable_and_clear(base: *mut void __iomem) {
    static void kona_timer_disable_and_clear(void __iomem *base)
    {
    uint32_t reg;
//
// clear and disable interrupts
// We are using compare/match register 0 for our system interrupts
//
    reg = readl(base + KONA_GPTIMER_STCS_OFFSET);
// Clear compare (0) interrupt
    reg |= 1 << KONA_GPTIMER_STCS_TIMER_MATCH_SHIFT;
// disable compare
    reg &= ~(1 << KONA_GPTIMER_STCS_COMPARE_ENABLE_SHIFT);
    writel(reg, base + KONA_GPTIMER_STCS_OFFSET);
    }
    static int
    kona_timer_get_counter(void __iomem *timer_base, uint32_t *msw, uint32_t *lsw)
    {
    let mut loop_limit: c_int = 3;
//
// Read 64-bit free running counter
// 1. Read hi-word
// 2. Read low-word
// 3. Read hi-word again
// 4.1
// if new hi-word is not equal to previously read hi-word, then
// start from #1
// 4.2
// if new hi-word is equal to previously read hi-word then stop.
//
    do {
// msw = readl(timer_base + KONA_GPTIMER_STCHI_OFFSET);
// lsw = readl(timer_base + KONA_GPTIMER_STCLO_OFFSET);
    if (*msw == readl(timer_base + KONA_GPTIMER_STCHI_OFFSET))
    break;
    } while (--loop_limit);
    if (!loop_limit) {
    pr_err("bcm_kona_timer: getting counter failed.\n");
    pr_err(" Timer will be impacted\n");
    return -ETIMEDOUT;
    }
    return 0;
    }
    static int kona_timer_set_next_event(unsigned long clc,
    struct clock_event_device *unused)
    {
//
// timer (0) is disabled by the timer interrupt already
// so, here we reload the next event value and re-enable
// the timer.
//
// This way, we are potentially losing the time between
// timer-interrupt->set_next_event. CPU local timers, when
// they come in should get rid of skew.
//
    uint32_t lsw, msw;
    uint32_t reg;
    int ret;
    ret = kona_timer_get_counter(timers.tmr_regs, &msw, &lsw);
    if (ret)
    return ret;
// Load the "next" event tick value
    writel(lsw + clc, timers.tmr_regs + KONA_GPTIMER_STCM0_OFFSET);
// Enable compare
    reg = readl(timers.tmr_regs + KONA_GPTIMER_STCS_OFFSET);
    reg |= (1 << KONA_GPTIMER_STCS_COMPARE_ENABLE_SHIFT);
    writel(reg, timers.tmr_regs + KONA_GPTIMER_STCS_OFFSET);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kona_timer_shutdown(evt: *mut clock_event_device) -> c_int {
    static int kona_timer_shutdown(struct clock_event_device *evt)
    {
    kona_timer_disable_and_clear(timers.tmr_regs);
    return 0;
    }
    static struct clock_event_device kona_clockevent_timer = {
    .name = "timer 1",
    .features = CLOCK_EVT_FEAT_ONESHOT,
    .set_next_event = kona_timer_set_next_event,
    .set_state_shutdown = kona_timer_shutdown,
    .tick_resume = kona_timer_shutdown,
    };
#[no_mangle]
unsafe extern "C" fn kona_timer_clockevents_init() -> void __init {
    static void __init kona_timer_clockevents_init(void)
    {
    kona_clockevent_timer.cpumask = cpumask_of(0);
    clockevents_config_and_register(&kona_clockevent_timer,
    arch_timer_rate, 6, 0xffffffff);
    }
#[no_mangle]
unsafe extern "C" fn kona_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t kona_timer_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *evt = &kona_clockevent_timer;
    kona_timer_disable_and_clear(timers.tmr_regs);
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn kona_timer_init(node: *mut device_node) -> int __init {
    static int __init kona_timer_init(struct device_node *node)
    {
    u32 freq;
    struct clk *external_clk;
    external_clk = of_clk_get_by_name(node, core::ptr::null_mut());
    if (!IS_ERR(external_clk)) {
    arch_timer_rate = clk_get_rate(external_clk);
    clk_prepare_enable(external_clk);
    } else if (!of_property_read_u32(node, "clock-frequency", &freq)) {
    arch_timer_rate = freq;
    } else {
    pr_err("Kona Timer v1 unable to determine clock-frequency\n");
    return -EINVAL;
    }
// Setup IRQ numbers
    timers.tmr_irq = irq_of_parse_and_map(node, 0);
// Setup IO addresses
    timers.tmr_regs = of_iomap(node, 0);
    kona_timer_disable_and_clear(timers.tmr_regs);
    kona_timer_clockevents_init();
    if (request_irq(timers.tmr_irq, kona_timer_interrupt, IRQF_TIMER,
    "Kona Timer Tick", core::ptr::null_mut()))
    pr_err("%s: request_irq() failed\n", "Kona Timer Tick");
    kona_timer_set_next_event((arch_timer_rate / HZ), core::ptr::null_mut());
    return 0;
    }
    TIMER_OF_DECLARE(brcm_kona, "brcm,kona-timer", kona_timer_init);
//
// bcm,kona-timer is deprecated by brcm,kona-timer
// being kept here for driver compatibility
//
    TIMER_OF_DECLARE(bcm_kona, "bcm,kona-timer", kona_timer_init);
