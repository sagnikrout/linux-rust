//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-imx-sysctr.c
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
// Copyright 2017-2019 NXP

pub const CMP_OFFSET: c_uint = 0x10000;
pub const RD_OFFSET: c_uint = 0x20000;
pub const CNTCV_LO: c_uint = 0x8;
pub const CNTCV_HI: c_uint = 0xc;

pub const SYS_CTR_EN: c_uint = 0x1;
pub const SYS_CTR_IRQ_MASK: c_uint = 0x2;
pub const SYS_CTR_CLK_DIV: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysctr_private {
    pub cmpcr: u32,
    pub lo_off: u32,
    pub hi_off: u32,
}

#[no_mangle]
unsafe extern "C" fn sysctr_timer_enable(evt: *mut clock_event_device, enable: bool) {
    static void sysctr_timer_enable(struct clock_event_device *evt, bool enable)
    {
    struct timer_of *to = to_timer_of(evt);
    struct sysctr_private *priv = to.private_data;
    void __iomem *base = timer_of_base(to);
    writel(enable ? priv.cmpcr | SYS_CTR_EN : priv.cmpcr, base + CMPCR);
    }
#[no_mangle]
unsafe extern "C" fn sysctr_irq_acknowledge(evt: *mut clock_event_device) {
    static void sysctr_irq_acknowledge(struct clock_event_device *evt)
    {
//
// clear the enable bit(EN =0) will clear
// the status bit(ISTAT = 0), then the interrupt
// signal will be negated(acknowledged).
//
    sysctr_timer_enable(evt, false);
    }
#[no_mangle]
pub unsafe extern "C" fn sysctr_read_counter(evt: *mut clock_event_device) -> u64 {
    static inline u64 sysctr_read_counter(struct clock_event_device *evt)
    {
    struct timer_of *to = to_timer_of(evt);
    struct sysctr_private *priv = to.private_data;
    void __iomem *base = timer_of_base(to);
    u32 cnt_hi, tmp_hi, cnt_lo;
    do {
    cnt_hi = readl_relaxed(base + priv.hi_off);
    cnt_lo = readl_relaxed(base + priv.lo_off);
    tmp_hi = readl_relaxed(base + priv.hi_off);
    } while (tmp_hi != cnt_hi);
    return  ((u64) cnt_hi << 32) | cnt_lo;
    }
    static int sysctr_set_next_event(unsigned long delta,
    struct clock_event_device *evt)
    {
    struct timer_of *to = to_timer_of(evt);
    void __iomem *base = timer_of_base(to);
    u32 cmp_hi, cmp_lo;
    u64 next;
    sysctr_timer_enable(evt, false);
    next = sysctr_read_counter(evt);
    next += delta;
    cmp_hi = (next >> 32) & 0x00fffff;
    cmp_lo = next & 0xffffffff;
    writel_relaxed(cmp_hi, base + CMPCV_HI);
    writel_relaxed(cmp_lo, base + CMPCV_LO);
    sysctr_timer_enable(evt, true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sysctr_set_state_oneshot(evt: *mut clock_event_device) -> c_int {
    static int sysctr_set_state_oneshot(struct clock_event_device *evt)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sysctr_set_state_shutdown(evt: *mut clock_event_device) -> c_int {
    static int sysctr_set_state_shutdown(struct clock_event_device *evt)
    {
    sysctr_timer_enable(evt, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sysctr_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t sysctr_timer_interrupt(int irq, void *dev_id)
    {
    struct clock_event_device *evt = dev_id;
    sysctr_irq_acknowledge(evt);
    evt.event_handler(evt);
    return IRQ_HANDLED;
    }
    static struct timer_of to_sysctr = {
    .flags = TIMER_OF_IRQ | TIMER_OF_CLOCK | TIMER_OF_BASE,
    .clkevt = {
    .name			= "i.MX system counter timer",
    .features		= CLOCK_EVT_FEAT_ONESHOT |
    CLOCK_EVT_FEAT_DYNIRQ,
    .set_state_oneshot	= sysctr_set_state_oneshot,
    .set_next_event		= sysctr_set_next_event,
    .set_state_shutdown	= sysctr_set_state_shutdown,
    .rating			= 200,
    },
    .of_irq = {
    .handler		= sysctr_timer_interrupt,
    .flags			= IRQF_TIMER,
    },
    .of_clk = {
    .name = "per",
    },
    };
#[no_mangle]
unsafe extern "C" fn __sysctr_timer_init(np: *mut device_node) -> int __init {
    static int __init __sysctr_timer_init(struct device_node *np)
    {
    struct sysctr_private *priv;
    void __iomem *base;
    int ret;
    priv = kzalloc_obj(struct sysctr_private);
    if (!priv)
    return -ENOMEM;
    ret = timer_of_init(np, &to_sysctr);
    if (ret) {
    kfree(priv);
    return ret;
    }
    if (!of_property_read_bool(np, "nxp,no-divider")) {
// system counter clock is divided by 3 internally
    to_sysctr.of_clk.rate /= SYS_CTR_CLK_DIV;
    }
    to_sysctr.clkevt.cpumask = cpu_possible_mask;
    to_sysctr.private_data = priv;
    base = timer_of_base(&to_sysctr);
    priv.cmpcr = readl(base + CMPCR) & ~SYS_CTR_EN;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sysctr_timer_init(np: *mut device_node) -> int __init {
    static int __init sysctr_timer_init(struct device_node *np)
    {
    struct sysctr_private *priv;
    int ret;
    ret = __sysctr_timer_init(np);
    if (ret)
    return ret;
    priv = to_sysctr.private_data;
    priv.lo_off = CNTCV_LO;
    priv.hi_off = CNTCV_HI;
    clockevents_config_and_register(&to_sysctr.clkevt,
    timer_of_rate(&to_sysctr),
    0xff, 0x7fffffff);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sysctr_timer_imx95_init(np: *mut device_node) -> int __init {
    static int __init sysctr_timer_imx95_init(struct device_node *np)
    {
    struct sysctr_private *priv;
    int ret;
    ret = __sysctr_timer_init(np);
    if (ret)
    return ret;
    priv = to_sysctr.private_data;
    priv.lo_off = CNTCV_LO_IMX95;
    priv.hi_off = CNTCV_HI_IMX95;
    clockevents_config_and_register(&to_sysctr.clkevt,
    timer_of_rate(&to_sysctr),
    0xff, 0x7fffffff);
    return 0;
    }
    TIMER_OF_DECLARE(sysctr_timer, "nxp,sysctr-timer", sysctr_timer_init);
    TIMER_OF_DECLARE(sysctr_timer_imx95, "nxp,imx95-sysctr-timer", sysctr_timer_imx95_init);
