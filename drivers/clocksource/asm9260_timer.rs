//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/asm9260_timer.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2014 Oleksij Rempel <linux@rempel-privat.de>
//

//
// this device provide 4 offsets for each register:
// 0x0 - plain read write mode
// 0x4 - set mode, OR logic.
// 0x8 - clr mode, XOR logic.
// 0xc - togle mode.
//
pub const SET_REG: c_int = 4;
pub const CLR_REG: c_int = 8;
pub const HW_IR: c_uint = 0x0000 /* RW. Interrupt */;

pub const HW_TCR: c_uint = 0x0010 /* RW. Timer controller */;
// BM_C*_RST
// Timer Counter and the Prescale Counter are synchronously reset on the
// next positive edge of PCLK. The counters remain reset until TCR[1] is
// returned to zero.

// BM_C*_EN
// 1 - Timer Counter and Prescale Counter are enabled for counting
// 0 - counters are disabled

pub const HW_DIR: c_uint = 0x0020 /* RW. Direction? */;
// 00 - count up
// 01 - count down
// 10 - ?? 2^n/2
pub const BM_DIR_COUNT_UP: c_int = 0;
pub const BM_DIR_COUNT_DOWN: c_int = 1;
pub const BM_DIR0_SHIFT: c_int = 0;
pub const BM_DIR1_SHIFT: c_int = 4;
pub const BM_DIR2_SHIFT: c_int = 8;
pub const BM_DIR3_SHIFT: c_int = 12;

    BM_DIR_COUNT_UP << BM_DIR1_SHIFT | \
    BM_DIR_COUNT_UP << BM_DIR2_SHIFT | \
    BM_DIR_COUNT_UP << BM_DIR3_SHIFT)
pub const HW_TC0: c_uint = 0x0030 /* RO. Timer counter 0 */;
// HW_TC*. Timer counter owerflow (0xffff.ffff to 0x0000.0000) do not generate
// interrupt. This registers can be used to detect overflow
pub const HW_TC1: c_uint = 0x0040;
pub const HW_TC2: c_uint = 0x0050;
pub const HW_TC3: c_uint = 0x0060;
pub const HW_PR: c_uint = 0x0070 /* RW. prescaler */;
pub const BM_PR_DISABLE: c_int = 0;
pub const HW_PC: c_uint = 0x0080 /* RO. Prescaler counter */;
pub const HW_MCR: c_uint = 0x0090 /* RW. Match control */;
// enable interrupt on match

// enable TC reset on match

// enable stop TC on match

pub const HW_MR0: c_uint = 0x00a0 /* RW. Match reg */;
pub const HW_MR1: c_uint = 0x00b0;
pub const HW_MR2: c_uint = 0x00C0;
pub const HW_MR3: c_uint = 0x00D0;
pub const HW_CTCR: c_uint = 0x0180 /* Counter control */;
pub const BM_CTCR0_SHIFT: c_int = 0;
pub const BM_CTCR1_SHIFT: c_int = 2;
pub const BM_CTCR2_SHIFT: c_int = 4;
pub const BM_CTCR3_SHIFT: c_int = 6;

    BM_CTCR_TM << BM_CTCR1_SHIFT | \
    BM_CTCR_TM << BM_CTCR2_SHIFT | \
    BM_CTCR_TM << BM_CTCR3_SHIFT)
    static struct asm9260_timer_priv {
    void __iomem *base;
    unsigned long ticks_per_jiffy;
    } priv;
    static int asm9260_timer_set_next_event(unsigned long delta,
    struct clock_event_device *evt)
    {
// configure match count for TC0
    writel_relaxed(delta, priv.base + HW_MR0);
// enable TC0
    writel_relaxed(BM_C0_EN, priv.base + HW_TCR + SET_REG);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __asm9260_timer_shutdown(evt: *mut clock_event_device) {
    static inline void __asm9260_timer_shutdown(struct clock_event_device *evt)
    {
// stop timer0
    writel_relaxed(BM_C0_EN, priv.base + HW_TCR + CLR_REG);
    }
#[no_mangle]
unsafe extern "C" fn asm9260_timer_shutdown(evt: *mut clock_event_device) -> c_int {
    static int asm9260_timer_shutdown(struct clock_event_device *evt)
    {
    __asm9260_timer_shutdown(evt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn asm9260_timer_set_oneshot(evt: *mut clock_event_device) -> c_int {
    static int asm9260_timer_set_oneshot(struct clock_event_device *evt)
    {
    __asm9260_timer_shutdown(evt);
// enable reset and stop on match
    writel_relaxed(BM_MCR_RES_EN(0) | BM_MCR_STOP_EN(0),
    priv.base + HW_MCR + SET_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn asm9260_timer_set_periodic(evt: *mut clock_event_device) -> c_int {
    static int asm9260_timer_set_periodic(struct clock_event_device *evt)
    {
    __asm9260_timer_shutdown(evt);
// disable reset and stop on match
    writel_relaxed(BM_MCR_RES_EN(0) | BM_MCR_STOP_EN(0),
    priv.base + HW_MCR + CLR_REG);
// configure match count for TC0
    writel_relaxed(priv.ticks_per_jiffy, priv.base + HW_MR0);
// enable TC0
    writel_relaxed(BM_C0_EN, priv.base + HW_TCR + SET_REG);
    return 0;
    }
    static struct clock_event_device event_dev = {
    .name			= DRIVER_NAME,
    .rating			= 200,
    .features		= CLOCK_EVT_FEAT_PERIODIC |
    CLOCK_EVT_FEAT_ONESHOT,
    .set_next_event		= asm9260_timer_set_next_event,
    .set_state_shutdown	= asm9260_timer_shutdown,
    .set_state_periodic	= asm9260_timer_set_periodic,
    .set_state_oneshot	= asm9260_timer_set_oneshot,
    .tick_resume		= asm9260_timer_shutdown,
    };
#[no_mangle]
unsafe extern "C" fn asm9260_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t asm9260_timer_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *evt = dev_id;
    evt.event_handler(evt);
    writel_relaxed(BM_IR_MR0, priv.base + HW_IR);
    return IRQ_HANDLED;
    }
//
// ---------------------------------------------------------------------------
// Timer initialization
// ---------------------------------------------------------------------------
//
#[no_mangle]
unsafe extern "C" fn asm9260_timer_init(np: *mut device_node) -> int __init {
    static int __init asm9260_timer_init(struct device_node *np)
    {
    int irq;
    struct clk *clk;
    int ret;
    unsigned long rate;
    priv.base = of_io_request_and_map(np, 0, np.name);
    if (IS_ERR(priv.base)) {
    pr_err("%pOFn: unable to map resource\n", np);
    return PTR_ERR(priv.base);
    }
    clk = of_clk_get(np, 0);
    if (IS_ERR(clk)) {
    pr_err("Failed to get clk!\n");
    return PTR_ERR(clk);
    }
    ret = clk_prepare_enable(clk);
    if (ret) {
    pr_err("Failed to enable clk!\n");
    return ret;
    }
    irq = irq_of_parse_and_map(np, 0);
    ret = request_irq(irq, asm9260_timer_interrupt, IRQF_TIMER,
    DRIVER_NAME, &event_dev);
    if (ret) {
    pr_err("Failed to setup irq!\n");
    clk_disable_unprepare(clk);
    return ret;
    }
// set all timers for count-up
    writel_relaxed(BM_DIR_DEFAULT, priv.base + HW_DIR);
// disable divider
    writel_relaxed(BM_PR_DISABLE, priv.base + HW_PR);
// make sure all timers use every rising PCLK edge.
    writel_relaxed(BM_CTCR_DEFAULT, priv.base + HW_CTCR);
// enable interrupt for TC0 and clean setting for all other lines
    writel_relaxed(BM_MCR_INT_EN(0) , priv.base + HW_MCR);
    rate = clk_get_rate(clk);
    clocksource_mmio_init(priv.base + HW_TC1, DRIVER_NAME, rate,
    200, 32, clocksource_mmio_readl_up);
// Seems like we can't use counter without match register even if
// actions for MR are disabled. So, set MR to max value.
    writel_relaxed(0xffffffff, priv.base + HW_MR1);
// enable TC1
    writel_relaxed(BM_C1_EN, priv.base + HW_TCR + SET_REG);
    priv.ticks_per_jiffy = DIV_ROUND_CLOSEST(rate, HZ);
    event_dev.cpumask = cpumask_of(0);
    clockevents_config_and_register(&event_dev, rate, 0x2c00, 0xfffffffe);
    return 0;
    }
    TIMER_OF_DECLARE(asm9260_timer, "alphascale,asm9260-timer",
    asm9260_timer_init);
