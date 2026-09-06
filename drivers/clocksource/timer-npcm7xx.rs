//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-npcm7xx.c
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
// Copyright (C) 2014-2018 Nuvoton Technologies tomer.maimon@nuvoton.com
// All rights reserved.
//
// Copyright 2017 Google, Inc.
//

// Timers registers
pub const NPCM7XX_REG_TCSR0: c_uint = 0x0 /* Timer 0 Control and Status Register */;
pub const NPCM7XX_REG_TICR0: c_uint = 0x8 /* Timer 0 Initial Count Register */;
pub const NPCM7XX_REG_TCSR1: c_uint = 0x4 /* Timer 1 Control and Status Register */;
pub const NPCM7XX_REG_TICR1: c_uint = 0xc /* Timer 1 Initial Count Register */;
pub const NPCM7XX_REG_TDR1: c_uint = 0x14 /* Timer 1 Data Register */;
pub const NPCM7XX_REG_TISR: c_uint = 0x18 /* Timer Interrupt Status Register */;
// Timers control
pub const NPCM7XX_Tx_RESETINT: c_uint = 0x1f;

pub const NPCM7XX_Tx_ONESHOT: c_uint = 0x0;

pub const NPCM7XX_Tx_MIN_PRESCALE: c_uint = 0x1;
pub const NPCM7XX_Tx_TDR_MASK_BITS: c_int = 24;
pub const NPCM7XX_Tx_MAX_CNT: c_uint = 0xFFFFFF;
pub const NPCM7XX_T0_CLR_INT: c_uint = 0x1;
pub const NPCM7XX_Tx_CLR_CSR: c_uint = 0x0;
// Timers operating mode

    NPCM7XX_Tx_INTEN | \
    NPCM7XX_Tx_MIN_PRESCALE)

    NPCM7XX_Tx_INTEN | \
    NPCM7XX_Tx_MIN_PRESCALE)

    NPCM7XX_Tx_MIN_PRESCALE)

#[no_mangle]
unsafe extern "C" fn npcm7xx_timer_resume(evt: *mut clock_event_device) -> c_int {
    static int npcm7xx_timer_resume(struct clock_event_device *evt)
    {
    struct timer_of *to = to_timer_of(evt);
    u32 val;
    val = readl(timer_of_base(to) + NPCM7XX_REG_TCSR0);
    val |= NPCM7XX_Tx_COUNTEN;
    writel(val, timer_of_base(to) + NPCM7XX_REG_TCSR0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn npcm7xx_timer_shutdown(evt: *mut clock_event_device) -> c_int {
    static int npcm7xx_timer_shutdown(struct clock_event_device *evt)
    {
    struct timer_of *to = to_timer_of(evt);
    u32 val;
    val = readl(timer_of_base(to) + NPCM7XX_REG_TCSR0);
    val &= ~NPCM7XX_Tx_COUNTEN;
    writel(val, timer_of_base(to) + NPCM7XX_REG_TCSR0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn npcm7xx_timer_oneshot(evt: *mut clock_event_device) -> c_int {
    static int npcm7xx_timer_oneshot(struct clock_event_device *evt)
    {
    struct timer_of *to = to_timer_of(evt);
    u32 val;
    val = readl(timer_of_base(to) + NPCM7XX_REG_TCSR0);
    val &= ~NPCM7XX_Tx_OPER;
    val |= NPCM7XX_START_ONESHOT_Tx;
    writel(val, timer_of_base(to) + NPCM7XX_REG_TCSR0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn npcm7xx_timer_periodic(evt: *mut clock_event_device) -> c_int {
    static int npcm7xx_timer_periodic(struct clock_event_device *evt)
    {
    struct timer_of *to = to_timer_of(evt);
    u32 val;
    writel(timer_of_period(to), timer_of_base(to) + NPCM7XX_REG_TICR0);
    val = readl(timer_of_base(to) + NPCM7XX_REG_TCSR0);
    val &= ~NPCM7XX_Tx_OPER;
    val |= NPCM7XX_START_PERIODIC_Tx;
    writel(val, timer_of_base(to) + NPCM7XX_REG_TCSR0);
    return 0;
    }
    static int npcm7xx_clockevent_set_next_event(unsigned long evt,
    struct clock_event_device *clk)
    {
    struct timer_of *to = to_timer_of(clk);
    u32 val;
    writel(evt, timer_of_base(to) + NPCM7XX_REG_TICR0);
    val = readl(timer_of_base(to) + NPCM7XX_REG_TCSR0);
    val |= NPCM7XX_START_Tx;
    writel(val, timer_of_base(to) + NPCM7XX_REG_TCSR0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn npcm7xx_timer0_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t npcm7xx_timer0_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *evt = (struct clock_event_device *)dev_id;
    struct timer_of *to = to_timer_of(evt);
    writel(NPCM7XX_T0_CLR_INT, timer_of_base(to) + NPCM7XX_REG_TISR);
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
    static struct timer_of npcm7xx_to = {
    .flags = TIMER_OF_IRQ | TIMER_OF_BASE | TIMER_OF_CLOCK,
    .clkevt = {
    .name		    = "npcm7xx-timer0",
    .features	    = CLOCK_EVT_FEAT_PERIODIC |
    CLOCK_EVT_FEAT_ONESHOT,
    .set_next_event	    = npcm7xx_clockevent_set_next_event,
    .set_state_shutdown = npcm7xx_timer_shutdown,
    .set_state_periodic = npcm7xx_timer_periodic,
    .set_state_oneshot  = npcm7xx_timer_oneshot,
    .tick_resume	    = npcm7xx_timer_resume,
    .rating		    = 300,
    },
    .of_irq = {
    .handler = npcm7xx_timer0_interrupt,
    .flags = IRQF_TIMER | IRQF_IRQPOLL,
    },
    };
#[no_mangle]
unsafe extern "C" fn npcm7xx_clockevents_init() -> void __init {
    static void __init npcm7xx_clockevents_init(void)
    {
    writel(NPCM7XX_DEFAULT_CSR,
    timer_of_base(&npcm7xx_to) + NPCM7XX_REG_TCSR0);
    writel(NPCM7XX_Tx_RESETINT,
    timer_of_base(&npcm7xx_to) + NPCM7XX_REG_TISR);
    npcm7xx_to.clkevt.cpumask = cpumask_of(0);
    clockevents_config_and_register(&npcm7xx_to.clkevt,
    timer_of_rate(&npcm7xx_to),
    0x1, NPCM7XX_Tx_MAX_CNT);
    }
#[no_mangle]
unsafe extern "C" fn npcm7xx_clocksource_init() -> void __init {
    static void __init npcm7xx_clocksource_init(void)
    {
    u32 val;
    writel(NPCM7XX_DEFAULT_CSR,
    timer_of_base(&npcm7xx_to) + NPCM7XX_REG_TCSR1);
    writel(NPCM7XX_Tx_MAX_CNT,
    timer_of_base(&npcm7xx_to) + NPCM7XX_REG_TICR1);
    val = readl(timer_of_base(&npcm7xx_to) + NPCM7XX_REG_TCSR1);
    val |= NPCM7XX_START_Tx;
    writel(val, timer_of_base(&npcm7xx_to) + NPCM7XX_REG_TCSR1);
    clocksource_mmio_init(timer_of_base(&npcm7xx_to) +
    NPCM7XX_REG_TDR1,
    "npcm7xx-timer1", timer_of_rate(&npcm7xx_to),
    200, (unsigned int)NPCM7XX_Tx_TDR_MASK_BITS,
    clocksource_mmio_readl_down);
    }
#[no_mangle]
unsafe extern "C" fn npcm7xx_timer_init(np: *mut device_node) -> int __init {
    static int __init npcm7xx_timer_init(struct device_node *np)
    {
    struct clk *clk;
    int ret;
    ret = timer_of_init(np, &npcm7xx_to);
    if (ret)
    return ret;
// Clock input is divided by PRESCALE + 1 before it is fed
// to the counter
    npcm7xx_to.of_clk.rate = npcm7xx_to.of_clk.rate /
    (NPCM7XX_Tx_MIN_PRESCALE + 1);
// Enable the clock for timer1, if it exists
    clk = of_clk_get(np, 1);
    if (clk) {
    if (!IS_ERR(clk))
    clk_prepare_enable(clk);
    else
    pr_warn("%pOF: Failed to get clock for timer1: %pe", np, clk);
    }
    npcm7xx_clocksource_init();
    npcm7xx_clockevents_init();
    pr_info("Enabling NPCM7xx clocksource timer base: %px, IRQ: %d ",
    timer_of_base(&npcm7xx_to), timer_of_irq(&npcm7xx_to));
    return 0;
    }
    TIMER_OF_DECLARE(wpcm450, "nuvoton,wpcm450-timer", npcm7xx_timer_init);
    TIMER_OF_DECLARE(npcm7xx, "nuvoton,npcm750-timer", npcm7xx_timer_init);
