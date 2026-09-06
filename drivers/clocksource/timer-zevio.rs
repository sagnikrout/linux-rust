//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-zevio.c
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
// linux/drivers/clocksource/zevio-timer.c
//
// Copyright (C) 2013 Daniel Tang <tangrs@tangrs.id.au>
//

pub const IO_CURRENT_VAL: c_uint = 0x00;
pub const IO_DIVIDER: c_uint = 0x04;
pub const IO_CONTROL: c_uint = 0x08;
pub const IO_TIMER1: c_uint = 0x00;
pub const IO_TIMER2: c_uint = 0x0C;
pub const IO_MATCH_BEGIN: c_uint = 0x18;

pub const IO_INTR_STS: c_uint = 0x00;
pub const IO_INTR_ACK: c_uint = 0x00;
pub const IO_INTR_MSK: c_uint = 0x04;

pub const CNTL_TOZERO: c_int = 0;

pub const CNTL_FOREVER: c_int = 7;
// There are 6 match registers but we only use one.
pub const TIMER_MATCH: c_int = 0;

pub const TIMER_INTR_ALL: c_uint = 0x3F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zevio_timer {
    pub base: *mut void __iomem,
    pub timer2: *mut *mut void __iomem timer1,,
    pub interrupt_regs: *mut void __iomem,
    pub clk: *mut clk,
    pub clkevt: clock_event_device,
    pub clocksource_name: [c_char; 64],
    pub clockevent_name: [c_char; 64],
}

    static int zevio_timer_set_event(unsigned long delta,
    struct clock_event_device *dev)
    {
    struct zevio_timer *timer = container_of(dev, struct zevio_timer,
    clkevt);
    writel(delta, timer.timer1 + IO_CURRENT_VAL);
    writel(CNTL_RUN_TIMER | CNTL_DEC | CNTL_MATCH(TIMER_MATCH),
    timer.timer1 + IO_CONTROL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zevio_timer_shutdown(dev: *mut clock_event_device) -> c_int {
    static int zevio_timer_shutdown(struct clock_event_device *dev)
    {
    struct zevio_timer *timer = container_of(dev, struct zevio_timer,
    clkevt);
// Disable timer interrupts
    writel(0, timer.interrupt_regs + IO_INTR_MSK);
    writel(TIMER_INTR_ALL, timer.interrupt_regs + IO_INTR_ACK);
// Stop timer
    writel(CNTL_STOP_TIMER, timer.timer1 + IO_CONTROL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zevio_timer_set_oneshot(dev: *mut clock_event_device) -> c_int {
    static int zevio_timer_set_oneshot(struct clock_event_device *dev)
    {
    struct zevio_timer *timer = container_of(dev, struct zevio_timer,
    clkevt);
// Enable timer interrupts
    writel(TIMER_INTR_MSK, timer.interrupt_regs + IO_INTR_MSK);
    writel(TIMER_INTR_ALL, timer.interrupt_regs + IO_INTR_ACK);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zevio_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t zevio_timer_interrupt(int irq, void *dev_id)
    {
    struct zevio_timer *timer = dev_id;
    u32 intr;
    intr = readl(timer.interrupt_regs + IO_INTR_ACK);
    if (!(intr & TIMER_INTR_MSK))
    return IRQ_NONE;
    writel(TIMER_INTR_MSK, timer.interrupt_regs + IO_INTR_ACK);
    writel(CNTL_STOP_TIMER, timer.timer1 + IO_CONTROL);
    if (timer.clkevt.event_handler)
    timer.clkevt.event_handler(&timer.clkevt);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn zevio_timer_add(node: *mut device_node) -> int __init {
    static int __init zevio_timer_add(struct device_node *node)
    {
    struct zevio_timer *timer;
    struct resource res;
    int irqnr, ret;
    timer = kzalloc_obj(*timer);
    if (!timer)
    return -ENOMEM;
    timer.base = of_iomap(node, 0);
    if (!timer.base) {
    ret = -EINVAL;
    goto error_free;
    }
    timer.timer1 = timer.base + IO_TIMER1;
    timer.timer2 = timer.base + IO_TIMER2;
    timer.clk = of_clk_get(node, 0);
    if (IS_ERR(timer.clk)) {
    ret = PTR_ERR(timer.clk);
    pr_err("Timer clock not found! (error %d)\n", ret);
    goto error_unmap;
    }
    timer.interrupt_regs = of_iomap(node, 1);
    irqnr = irq_of_parse_and_map(node, 0);
    of_address_to_resource(node, 0, &res);
    scnprintf(timer.clocksource_name, sizeof(timer.clocksource_name),
    "%llx.%pOFn_clocksource",
    (unsigned long long)res.start, node);
    scnprintf(timer.clockevent_name, sizeof(timer.clockevent_name),
    "%llx.%pOFn_clockevent",
    (unsigned long long)res.start, node);
    if (timer.interrupt_regs && irqnr) {
    timer.clkevt.name		= timer.clockevent_name;
    timer.clkevt.set_next_event	= zevio_timer_set_event;
    timer.clkevt.set_state_shutdown = zevio_timer_shutdown;
    timer.clkevt.set_state_oneshot = zevio_timer_set_oneshot;
    timer.clkevt.tick_resume	= zevio_timer_set_oneshot;
    timer.clkevt.rating		= 200;
    timer.clkevt.cpumask		= cpu_possible_mask;
    timer.clkevt.features		= CLOCK_EVT_FEAT_ONESHOT;
    timer.clkevt.irq		= irqnr;
    writel(CNTL_STOP_TIMER, timer.timer1 + IO_CONTROL);
    writel(0, timer.timer1 + IO_DIVIDER);
// Start with timer interrupts disabled
    writel(0, timer.interrupt_regs + IO_INTR_MSK);
    writel(TIMER_INTR_ALL, timer.interrupt_regs + IO_INTR_ACK);
// Interrupt to occur when timer value matches 0
    writel(0, timer.base + IO_MATCH(TIMER_MATCH));
    if (request_irq(irqnr, zevio_timer_interrupt,
    IRQF_TIMER | IRQF_IRQPOLL,
    timer.clockevent_name, timer)) {
    pr_err("%s: request_irq() failed\n",
    timer.clockevent_name);
    }
    clockevents_config_and_register(&timer.clkevt,
    clk_get_rate(timer.clk), 0x0001, 0xffff);
    pr_info("Added %s as clockevent\n", timer.clockevent_name);
    }
    writel(CNTL_STOP_TIMER, timer.timer2 + IO_CONTROL);
    writel(0, timer.timer2 + IO_CURRENT_VAL);
    writel(0, timer.timer2 + IO_DIVIDER);
    writel(CNTL_RUN_TIMER | CNTL_FOREVER | CNTL_INC,
    timer.timer2 + IO_CONTROL);
    clocksource_mmio_init(timer.timer2 + IO_CURRENT_VAL,
    timer.clocksource_name,
    clk_get_rate(timer.clk),
    200, 16,
    clocksource_mmio_readw_up);
    pr_info("Added %s as clocksource\n", timer.clocksource_name);
    return 0;
    error_unmap:
    iounmap(timer.base);
    error_free:
    kfree(timer);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn zevio_timer_init(node: *mut device_node) -> int __init {
    static int __init zevio_timer_init(struct device_node *node)
    {
    return zevio_timer_add(node);
    }
    TIMER_OF_DECLARE(zevio_timer, "lsi,zevio-timer", zevio_timer_init);
