//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-microchip-pit64b.c
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
// 64-bit Periodic Interval Timer driver
//
// Copyright (C) 2019 Microchip Technology Inc. and its subsidiaries
//
// Author: Claudiu Beznea <claudiu.beznea@microchip.com>
//

pub const MCHP_PIT64B_CR: c_uint = 0x00	/* Control Register */;

pub const MCHP_PIT64B_MR: c_uint = 0x04	/* Mode Register */;

pub const MCHP_PIT64B_LSB_PR: c_uint = 0x08	/* LSB Period Register */;
pub const MCHP_PIT64B_MSB_PR: c_uint = 0x0C	/* MSB Period Register */;
pub const MCHP_PIT64B_IER: c_uint = 0x10	/* Interrupt Enable Register */;

pub const MCHP_PIT64B_ISR: c_uint = 0x1C	/* Interrupt Status Register */;
pub const MCHP_PIT64B_TLSBR: c_uint = 0x20	/* Timer LSB Register */;
pub const MCHP_PIT64B_TMSBR: c_uint = 0x24	/* Timer MSB Register */;
pub const MCHP_PIT64B_PRES_MAX: c_uint = 0x10;

//
// struct mchp_pit64b_timer - PIT64B timer data structure
// @base: base address of PIT64B hardware block
// @pclk: PIT64B's peripheral clock
// @gclk: PIT64B's generic clock
// @mode: precomputed value for mode register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mchp_pit64b_timer {
    pub base: *mut void __iomem,
    pub pclk: *mut clk,
    pub gclk: *mut clk,
    pub mode: u32,
}

//
// struct mchp_pit64b_clkevt - PIT64B clockevent data structure
// @timer: PIT64B timer
// @clkevt: clockevent
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mchp_pit64b_clkevt {
    pub timer: mchp_pit64b_timer,
    pub clkevt: clock_event_device,
}

    ((struct mchp_pit64b_timer *)container_of(x,\
    struct mchp_pit64b_clkevt, clkevt))
//
// struct mchp_pit64b_clksrc - PIT64B clocksource data structure
// @timer: PIT64B timer
// @clksrc: clocksource
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mchp_pit64b_clksrc {
    pub timer: mchp_pit64b_timer,
    pub clksrc: clocksource,
}

    ((struct mchp_pit64b_timer *)container_of(x,\
    struct mchp_pit64b_clksrc, clksrc))
// Base address for clocksource timer.
    static void __iomem *mchp_pit64b_cs_base;
// Default cycles for clockevent timer.
    static u64 mchp_pit64b_ce_cycles;
// Delay timer.
    static struct delay_timer mchp_pit64b_dt;
#[no_mangle]
pub unsafe extern "C" fn mchp_pit64b_cnt_read(base: *mut void __iomem) -> u64 {
    static inline u64 mchp_pit64b_cnt_read(void __iomem *base)
    {
    unsigned long	flags;
    u32		low, high;
    raw_local_irq_save(flags);
//
// When using a 64 bit period TLSB must be read first, followed by the
// read of TMSB. This sequence generates an atomic read of the 64 bit
// timer value whatever the lapse of time between the accesses.
//
    low = readl_relaxed(base + MCHP_PIT64B_TLSBR);
    high = readl_relaxed(base + MCHP_PIT64B_TMSBR);
    raw_local_irq_restore(flags);
    return (((u64)high << 32) | low);
    }
    static inline void mchp_pit64b_reset(struct mchp_pit64b_timer *timer,
    u64 cycles, u32 mode, u32 irqs)
    {
    u32 low, high;
    low = cycles & MCHP_PIT64B_LSBMASK;
    high = cycles >> 32;
    writel_relaxed(MCHP_PIT64B_CR_SWRST, timer.base + MCHP_PIT64B_CR);
    writel_relaxed(mode | timer.mode, timer.base + MCHP_PIT64B_MR);
    writel_relaxed(high, timer.base + MCHP_PIT64B_MSB_PR);
    writel_relaxed(low, timer.base + MCHP_PIT64B_LSB_PR);
    writel_relaxed(irqs, timer.base + MCHP_PIT64B_IER);
    writel_relaxed(MCHP_PIT64B_CR_START, timer.base + MCHP_PIT64B_CR);
    }
#[no_mangle]
unsafe extern "C" fn mchp_pit64b_suspend(timer: *mut mchp_pit64b_timer) {
    static void mchp_pit64b_suspend(struct mchp_pit64b_timer *timer)
    {
    writel_relaxed(MCHP_PIT64B_CR_SWRST, timer.base + MCHP_PIT64B_CR);
    if (timer.mode & MCHP_PIT64B_MR_SGCLK)
    clk_disable_unprepare(timer.gclk);
    clk_disable_unprepare(timer.pclk);
    }
#[no_mangle]
unsafe extern "C" fn mchp_pit64b_resume(timer: *mut mchp_pit64b_timer) {
    static void mchp_pit64b_resume(struct mchp_pit64b_timer *timer)
    {
    clk_prepare_enable(timer.pclk);
    if (timer.mode & MCHP_PIT64B_MR_SGCLK)
    clk_prepare_enable(timer.gclk);
    }
#[no_mangle]
unsafe extern "C" fn mchp_pit64b_clksrc_suspend(cs: *mut clocksource) {
    static void mchp_pit64b_clksrc_suspend(struct clocksource *cs)
    {
    struct mchp_pit64b_timer *timer = clksrc_to_mchp_pit64b_timer(cs);
    mchp_pit64b_suspend(timer);
    }
#[no_mangle]
unsafe extern "C" fn mchp_pit64b_clksrc_resume(cs: *mut clocksource) {
    static void mchp_pit64b_clksrc_resume(struct clocksource *cs)
    {
    struct mchp_pit64b_timer *timer = clksrc_to_mchp_pit64b_timer(cs);
    mchp_pit64b_resume(timer);
    mchp_pit64b_reset(timer, ULLONG_MAX, MCHP_PIT64B_MR_CONT, 0);
    }
#[no_mangle]
unsafe extern "C" fn mchp_pit64b_clksrc_read(cs: *mut clocksource) -> u64 {
    static u64 mchp_pit64b_clksrc_read(struct clocksource *cs)
    {
    return mchp_pit64b_cnt_read(mchp_pit64b_cs_base);
    }
#[no_mangle]
unsafe extern "C" fn mchp_pit64b_sched_read_clk() -> u64 notrace {
    static u64 notrace mchp_pit64b_sched_read_clk(void)
    {
    return mchp_pit64b_cnt_read(mchp_pit64b_cs_base);
    }
#[no_mangle]
unsafe extern "C" fn mchp_pit64b_dt_read() -> unsigned long notrace {
    static unsigned long notrace mchp_pit64b_dt_read(void)
    {
    return mchp_pit64b_cnt_read(mchp_pit64b_cs_base);
    }
#[no_mangle]
unsafe extern "C" fn mchp_pit64b_clkevt_shutdown(cedev: *mut clock_event_device) -> c_int {
    static int mchp_pit64b_clkevt_shutdown(struct clock_event_device *cedev)
    {
    struct mchp_pit64b_timer *timer = clkevt_to_mchp_pit64b_timer(cedev);
    if (!clockevent_state_detached(cedev))
    mchp_pit64b_suspend(timer);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mchp_pit64b_clkevt_set_periodic(cedev: *mut clock_event_device) -> c_int {
    static int mchp_pit64b_clkevt_set_periodic(struct clock_event_device *cedev)
    {
    struct mchp_pit64b_timer *timer = clkevt_to_mchp_pit64b_timer(cedev);
    if (clockevent_state_shutdown(cedev))
    mchp_pit64b_resume(timer);
    mchp_pit64b_reset(timer, mchp_pit64b_ce_cycles, MCHP_PIT64B_MR_CONT,
    MCHP_PIT64B_IER_PERIOD);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mchp_pit64b_clkevt_set_oneshot(cedev: *mut clock_event_device) -> c_int {
    static int mchp_pit64b_clkevt_set_oneshot(struct clock_event_device *cedev)
    {
    struct mchp_pit64b_timer *timer = clkevt_to_mchp_pit64b_timer(cedev);
    if (clockevent_state_shutdown(cedev))
    mchp_pit64b_resume(timer);
    mchp_pit64b_reset(timer, mchp_pit64b_ce_cycles, MCHP_PIT64B_MR_ONE_SHOT,
    MCHP_PIT64B_IER_PERIOD);
    return 0;
    }
    static int mchp_pit64b_clkevt_set_next_event(unsigned long evt,
    struct clock_event_device *cedev)
    {
    struct mchp_pit64b_timer *timer = clkevt_to_mchp_pit64b_timer(cedev);
    mchp_pit64b_reset(timer, evt, MCHP_PIT64B_MR_ONE_SHOT,
    MCHP_PIT64B_IER_PERIOD);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mchp_pit64b_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mchp_pit64b_interrupt(int irq, void *dev_id)
    {
    struct mchp_pit64b_clkevt *irq_data = dev_id;
// Need to clear the interrupt.
    readl_relaxed(irq_data.timer.base + MCHP_PIT64B_ISR);
    irq_data.clkevt.event_handler(&irq_data.clkevt);
    return IRQ_HANDLED;
    }
    static void __init mchp_pit64b_pres_compute(u32 *pres, u32 clk_rate,
    u32 max_rate)
    {
    u32 tmp;
    for (*pres = 0; *pres < MCHP_PIT64B_PRES_MAX; (*pres)++) {
    tmp = clk_rate / (*pres + 1);
    if (tmp <= max_rate)
    break;
    }
// Use the biggest prescaler if we didn't match one.
    if (*pres == MCHP_PIT64B_PRES_MAX)
// pres = MCHP_PIT64B_PRES_MAX - 1;
    }
//
// mchp_pit64b_init_mode() - prepare PIT64B mode register value to be used at
// runtime; this includes prescaler and SGCLK bit
// @timer: pointer to pit64b timer to init
// @max_rate: maximum rate that timer's clock could use
//
// PIT64B timer may be fed by gclk or pclk. When gclk is used its rate has to
// be at least 3 times lower that pclk's rate. pclk rate is fixed, gclk rate
// could be changed via clock APIs. The chosen clock (pclk or gclk) could be
// divided by the internal PIT64B's divider.
//
// This function, first tries to use GCLK by requesting the desired rate from
// PMC and then using the internal PIT64B prescaler, if any, to reach the
// requested rate. If PCLK/GCLK < 3 (condition requested by PIT64B hardware)
// then the function falls back on using PCLK as clock source for PIT64B timer
// choosing the highest prescaler in case it doesn't locate one to match the
// requested frequency.
//
// Below is presented the PIT64B block in relation with PMC:
//
// PIT64B
// PMC             +------------------------------------+
// +----+           |   +-----+                          |
// |    |-->gclk -->|-->|     |    +---------+  +-----+  |
// |    |           |   | MUX |--->| Divider |->|timer|  |
// |    |-->pclk -->|-->|     |    +---------+  +-----+  |
// +----+           |   +-----+                          |
// |      ^                             |
// |     sel                            |
// +------------------------------------+
//
// Where:
// - gclk rate <= pclk rate/3
// - gclk rate could be requested from PMC
// - pclk rate is fixed (cannot be requested from PMC)
//
    static int __init mchp_pit64b_init_mode(struct mchp_pit64b_timer *timer,
    unsigned long max_rate)
    {
    unsigned long pclk_rate, diff = 0, best_diff = ULONG_MAX;
    let mut gclk_round: c_long = 0;
    u32 pres, best_pres = 0;
    pclk_rate = clk_get_rate(timer.pclk);
    if (!pclk_rate)
    return -EINVAL;
    timer.mode = 0;
// Try using GCLK.
    gclk_round = clk_round_rate(timer.gclk, max_rate);
    if (gclk_round < 0)
    goto pclk;
    if (pclk_rate / gclk_round < 3)
    goto pclk;
    mchp_pit64b_pres_compute(&pres, gclk_round, max_rate);
    best_diff = abs(gclk_round / (pres + 1) - max_rate);
    best_pres = pres;
    if (!best_diff) {
    timer.mode |= MCHP_PIT64B_MR_SGCLK;
    clk_set_rate(timer.gclk, gclk_round);
    goto done;
    }
    pclk:
// Check if requested rate could be obtained using PCLK.
    mchp_pit64b_pres_compute(&pres, pclk_rate, max_rate);
    diff = abs(pclk_rate / (pres + 1) - max_rate);
    if (best_diff > diff) {
// Use PCLK.
    best_pres = pres;
    } else {
// Use GCLK.
    timer.mode |= MCHP_PIT64B_MR_SGCLK;
    clk_set_rate(timer.gclk, gclk_round);
    }
    done:
    timer.mode |= MCHP_PIT64B_PRES_TO_MODE(best_pres);
    pr_info("PIT64B: using clk=%s with prescaler %u, freq=%lu [Hz]\n",
    timer.mode & MCHP_PIT64B_MR_SGCLK ? "gclk" : "pclk", best_pres,
    timer.mode & MCHP_PIT64B_MR_SGCLK ?
    gclk_round / (best_pres + 1) : pclk_rate / (best_pres + 1));
    return 0;
    }
    static int __init mchp_pit64b_init_clksrc(struct mchp_pit64b_timer *timer,
    u32 clk_rate)
    {
    struct mchp_pit64b_clksrc *cs;
    int ret;
    cs = kzalloc_obj(*cs);
    if (!cs)
    return -ENOMEM;
    mchp_pit64b_resume(timer);
    mchp_pit64b_reset(timer, ULLONG_MAX, MCHP_PIT64B_MR_CONT, 0);
    mchp_pit64b_cs_base = timer.base;
    cs.timer.base = timer.base;
    cs.timer.pclk = timer.pclk;
    cs.timer.gclk = timer.gclk;
    cs.timer.mode = timer.mode;
    cs.clksrc.name = MCHP_PIT64B_NAME;
    cs.clksrc.mask = CLOCKSOURCE_MASK(64);
    cs.clksrc.flags = CLOCK_SOURCE_IS_CONTINUOUS;
    cs.clksrc.rating = 210;
    cs.clksrc.read = mchp_pit64b_clksrc_read;
    cs.clksrc.suspend = mchp_pit64b_clksrc_suspend;
    cs.clksrc.resume = mchp_pit64b_clksrc_resume;
    ret = clocksource_register_hz(&cs.clksrc, clk_rate);
    if (ret) {
    pr_debug("clksrc: Failed to register PIT64B clocksource!\n");
// Stop timer.
    mchp_pit64b_suspend(timer);
    kfree(cs);
    return ret;
    }
    sched_clock_register(mchp_pit64b_sched_read_clk, 64, clk_rate);
    mchp_pit64b_dt.read_current_timer = mchp_pit64b_dt_read;
    mchp_pit64b_dt.freq = clk_rate;
    register_current_timer_delay(&mchp_pit64b_dt);
    return 0;
    }
    static int __init mchp_pit64b_init_clkevt(struct mchp_pit64b_timer *timer,
    u32 clk_rate, u32 irq)
    {
    struct mchp_pit64b_clkevt *ce;
    int ret;
    ce = kzalloc_obj(*ce);
    if (!ce)
    return -ENOMEM;
    mchp_pit64b_ce_cycles = DIV_ROUND_CLOSEST(clk_rate, HZ);
    ce.timer.base = timer.base;
    ce.timer.pclk = timer.pclk;
    ce.timer.gclk = timer.gclk;
    ce.timer.mode = timer.mode;
    ce.clkevt.name = MCHP_PIT64B_NAME;
    ce.clkevt.features = CLOCK_EVT_FEAT_ONESHOT | CLOCK_EVT_FEAT_PERIODIC;
    ce.clkevt.rating = 150;
    ce.clkevt.set_state_shutdown = mchp_pit64b_clkevt_shutdown;
    ce.clkevt.set_state_periodic = mchp_pit64b_clkevt_set_periodic;
    ce.clkevt.set_state_oneshot = mchp_pit64b_clkevt_set_oneshot;
    ce.clkevt.set_next_event = mchp_pit64b_clkevt_set_next_event;
    ce.clkevt.cpumask = cpumask_of(0);
    ce.clkevt.irq = irq;
    ret = request_irq(irq, mchp_pit64b_interrupt, IRQF_TIMER,
    "pit64b_tick", ce);
    if (ret) {
    pr_debug("clkevt: Failed to setup PIT64B IRQ\n");
    kfree(ce);
    return ret;
    }
    clockevents_config_and_register(&ce.clkevt, clk_rate, 1, ULONG_MAX);
    return 0;
    }
    static int __init mchp_pit64b_dt_init_timer(struct device_node *node,
    bool clkevt)
    {
    struct mchp_pit64b_timer timer;
    unsigned long clk_rate;
    let mut irq: u32 = 0;
    int ret;
// Parse DT node.
    timer.pclk = of_clk_get_by_name(node, "pclk");
    if (IS_ERR(timer.pclk))
    return PTR_ERR(timer.pclk);
    timer.gclk = of_clk_get_by_name(node, "gclk");
    if (IS_ERR(timer.gclk))
    return PTR_ERR(timer.gclk);
    timer.base = of_iomap(node, 0);
    if (!timer.base)
    return -ENXIO;
    if (clkevt) {
    irq = irq_of_parse_and_map(node, 0);
    if (!irq) {
    ret = -ENODEV;
    goto io_unmap;
    }
    }
// Initialize mode (prescaler + SGCK bit). To be used at runtime.
    ret = mchp_pit64b_init_mode(&timer, MCHP_PIT64B_DEF_FREQ);
    if (ret)
    goto irq_unmap;
    if (timer.mode & MCHP_PIT64B_MR_SGCLK)
    clk_rate = clk_get_rate(timer.gclk);
    else
    clk_rate = clk_get_rate(timer.pclk);
    clk_rate = clk_rate / (MCHP_PIT64B_MODE_TO_PRES(timer.mode) + 1);
    if (clkevt)
    ret = mchp_pit64b_init_clkevt(&timer, clk_rate, irq);
    else
    ret = mchp_pit64b_init_clksrc(&timer, clk_rate);
    if (ret)
    goto irq_unmap;
    return 0;
    irq_unmap:
    irq_dispose_mapping(irq);
    io_unmap:
    iounmap(timer.base);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mchp_pit64b_dt_init(node: *mut device_node) -> int __init {
    static int __init mchp_pit64b_dt_init(struct device_node *node)
    {
    static int inits;
    switch (inits++) {
    case 0:
// 1st request, register clockevent.
    return mchp_pit64b_dt_init_timer(node, true);
    case 1:
// 2nd request, register clocksource.
    return mchp_pit64b_dt_init_timer(node, false);
    }
// The rest, don't care.
    return -EINVAL;
    }
    TIMER_OF_DECLARE(mchp_pit64b, "microchip,sam9x60-pit64b", mchp_pit64b_dt_init);
