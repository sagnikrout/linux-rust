//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-imx-tpm.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2016 Freescale Semiconductor, Inc.
// Copyright 2017 NXP

pub const TPM_PARAM: c_uint = 0x4;
pub const TPM_PARAM_WIDTH_SHIFT: c_int = 16;

pub const TPM_SC: c_uint = 0x10;

pub const TPM_SC_CMOD_DIV_DEFAULT: c_uint = 0x3;
pub const TPM_SC_CMOD_DIV_MAX: c_uint = 0x7;

pub const TPM_CNT: c_uint = 0x14;
pub const TPM_MOD: c_uint = 0x18;
pub const TPM_STATUS: c_uint = 0x1c;

pub const TPM_C0SC: c_uint = 0x20;

pub const TPM_C0SC_MODE_SHIFT: c_int = 2;
pub const TPM_C0SC_MODE_MASK: c_uint = 0x3c;
pub const TPM_C0SC_MODE_SW_COMPARE: c_uint = 0x4;

pub const TPM_C0V: c_uint = 0x24;
    static int counter_width __ro_after_init;
    static void __iomem *timer_base __ro_after_init;
#[no_mangle]
pub unsafe extern "C" fn tpm_timer_disable() {
    static inline void tpm_timer_disable(void)
    {
    unsigned int val;
// channel disable
    val = readl(timer_base + TPM_C0SC);
    val &= ~(TPM_C0SC_MODE_MASK | TPM_C0SC_CHIE);
    writel(val, timer_base + TPM_C0SC);
    }
#[no_mangle]
pub unsafe extern "C" fn tpm_timer_enable() {
    static inline void tpm_timer_enable(void)
    {
    unsigned int val;
// channel enabled in sw compare mode
    val = readl(timer_base + TPM_C0SC);
    val |= (TPM_C0SC_MODE_SW_COMPARE << TPM_C0SC_MODE_SHIFT) |
    TPM_C0SC_CHIE;
    writel(val, timer_base + TPM_C0SC);
    }
#[no_mangle]
pub unsafe extern "C" fn tpm_irq_acknowledge() {
    static inline void tpm_irq_acknowledge(void)
    {
    writel(TPM_STATUS_CH0F, timer_base + TPM_STATUS);
    }
#[no_mangle]
pub unsafe extern "C" fn tpm_read_counter() -> c_ulong {
    static inline unsigned long tpm_read_counter(void)
    {
    return readl(timer_base + TPM_CNT);
    }

    static struct delay_timer tpm_delay_timer;
#[no_mangle]
unsafe extern "C" fn tpm_read_current_timer() -> c_ulong {
    static unsigned long tpm_read_current_timer(void)
    {
    return tpm_read_counter();
    }
#[no_mangle]
unsafe extern "C" fn tpm_read_sched_clock() -> u64 notrace {
    static u64 notrace tpm_read_sched_clock(void)
    {
    return tpm_read_counter();
    }

    static int tpm_set_next_event(unsigned long delta,
    struct clock_event_device *evt)
    {
    unsigned long next, prev, now;
    prev = tpm_read_counter();
    next = prev + delta;
    writel(next, timer_base + TPM_C0V);
    now = tpm_read_counter();
//
// Need to wait CNT increase at least 1 cycle to make sure
// the C0V has been updated into HW.
//
    if ((next & 0xffffffff) != readl(timer_base + TPM_C0V))
    while (now == tpm_read_counter())
    ;
//
// NOTE: We observed in a very small probability, the bus fabric
// contention between GPU and A7 may results a few cycles delay
// of writing CNT registers which may cause the min_delta event got
// missed, so we need add a ETIME check here in case it happened.
//
    return (now - prev) >= delta ? -ETIME : 0;
    }
#[no_mangle]
unsafe extern "C" fn tpm_set_state_oneshot(evt: *mut clock_event_device) -> c_int {
    static int tpm_set_state_oneshot(struct clock_event_device *evt)
    {
    tpm_timer_enable();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tpm_set_state_shutdown(evt: *mut clock_event_device) -> c_int {
    static int tpm_set_state_shutdown(struct clock_event_device *evt)
    {
    tpm_timer_disable();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tpm_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t tpm_timer_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *evt = dev_id;
    tpm_irq_acknowledge();
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
    static struct timer_of to_tpm = {
    .flags = TIMER_OF_IRQ | TIMER_OF_BASE | TIMER_OF_CLOCK,
    .clkevt = {
    .name			= "i.MX TPM Timer",
    .rating			= 200,
    .features		= CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_DYNIRQ,
    .set_state_shutdown	= tpm_set_state_shutdown,
    .set_state_oneshot	= tpm_set_state_oneshot,
    .set_next_event		= tpm_set_next_event,
    .cpumask		= cpu_possible_mask,
    },
    .of_irq = {
    .handler		= tpm_timer_interrupt,
    .flags			= IRQF_TIMER,
    },
    .of_clk = {
    .name = "per",
    },
    };
#[no_mangle]
unsafe extern "C" fn tpm_clocksource_init() -> int __init {
    static int __init tpm_clocksource_init(void)
    {

    tpm_delay_timer.read_current_timer = &tpm_read_current_timer;
    tpm_delay_timer.freq = timer_of_rate(&to_tpm) >> 3;
    register_current_timer_delay(&tpm_delay_timer);
    sched_clock_register(tpm_read_sched_clock, counter_width,
    timer_of_rate(&to_tpm) >> 3);

    return clocksource_mmio_init(timer_base + TPM_CNT,
    "imx-tpm",
    timer_of_rate(&to_tpm) >> 3,
    to_tpm.clkevt.rating,
    counter_width,
    clocksource_mmio_readl_up);
    }
#[no_mangle]
unsafe extern "C" fn tpm_clockevent_init() -> void __init {
    static void __init tpm_clockevent_init(void)
    {
    clockevents_config_and_register(&to_tpm.clkevt,
    timer_of_rate(&to_tpm) >> 3,
    300,
    GENMASK(counter_width - 1,
    1));
    }
#[no_mangle]
unsafe extern "C" fn tpm_timer_init(np: *mut device_node) -> int __init {
    static int __init tpm_timer_init(struct device_node *np)
    {
    struct clk *ipg;
    int ret;
    ipg = of_clk_get_by_name(np, "ipg");
    if (IS_ERR(ipg)) {
    pr_err("tpm: failed to get ipg clk\n");
    return -ENODEV;
    }
// enable clk before accessing registers
    ret = clk_prepare_enable(ipg);
    if (ret) {
    pr_err("tpm: ipg clock enable failed (%d)\n", ret);
    clk_put(ipg);
    return ret;
    }
    ret = timer_of_init(np, &to_tpm);
    if (ret)
    return ret;
    timer_base = timer_of_base(&to_tpm);
    counter_width = (readl(timer_base + TPM_PARAM)
    & TPM_PARAM_WIDTH_MASK) >> TPM_PARAM_WIDTH_SHIFT;
// use rating 200 for 32-bit counter and 150 for 16-bit counter
    to_tpm.clkevt.rating = counter_width == 0x20 ? 200 : 150;
//
// Initialize tpm module to a known state
// 1) Counter disabled
// 2) TPM counter operates in up counting mode
// 3) Timer Overflow Interrupt disabled
// 4) Channel0 disabled
// 5) DMA transfers disabled
//
// make sure counter is disabled
    writel(0, timer_base + TPM_SC);
// TOF is W1C
    writel(TPM_SC_TOF_MASK, timer_base + TPM_SC);
    writel(0, timer_base + TPM_CNT);
// CHF is W1C
    writel(TPM_C0SC_CHF_MASK, timer_base + TPM_C0SC);
//
// increase per cnt,
// div 8 for 32-bit counter and div 128 for 16-bit counter
//
    writel(TPM_SC_CMOD_INC_PER_CNT |
    (counter_width == 0x20 ?
    TPM_SC_CMOD_DIV_DEFAULT : TPM_SC_CMOD_DIV_MAX),
    timer_base + TPM_SC);
// set MOD register to maximum for free running mode
    writel(GENMASK(counter_width - 1, 0), timer_base + TPM_MOD);
    tpm_clockevent_init();
    return tpm_clocksource_init();
    }
    TIMER_OF_DECLARE(imx7ulp, "fsl,imx7ulp-tpm", tpm_timer_init);
