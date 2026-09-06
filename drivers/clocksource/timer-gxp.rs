//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/timer-gxp.c
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
// Copyright (C) 2022 Hewlett-Packard Enterprise Development Company, L.P.

pub const TIMER0_FREQ: c_int = 1000000;
pub const GXP_TIMER_CNT_OFS: c_uint = 0x00;
pub const GXP_TIMESTAMP_OFS: c_uint = 0x08;
pub const GXP_TIMER_CTRL_OFS: c_uint = 0x14;
// TCS Stands for Timer Control/Status: these are masks to be used in
// the Timer Count Registers
pub const MASK_TCS_ENABLE: c_uint = 0x01;
pub const MASK_TCS_PERIOD: c_uint = 0x02;
pub const MASK_TCS_RELOAD: c_uint = 0x04;
pub const MASK_TCS_TC: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gxp_timer {
    pub counter: *mut void __iomem,
    pub control: *mut void __iomem,
    pub evt: clock_event_device,
}

    static struct gxp_timer *gxp_timer;
    static void __iomem *system_clock __ro_after_init;
    static inline struct gxp_timer *to_gxp_timer(struct clock_event_device *evt_dev)
    {
    return container_of(evt_dev, struct gxp_timer, evt);
    }
#[no_mangle]
unsafe extern "C" fn gxp_sched_read() -> u64 notrace {
    static u64 notrace gxp_sched_read(void)
    {
    return readl_relaxed(system_clock);
    }
#[no_mangle]
unsafe extern "C" fn gxp_time_set_next_event(event: c_ulong, evt_dev: *mut clock_event_device) -> c_int {
    static int gxp_time_set_next_event(unsigned long event, struct clock_event_device *evt_dev)
    {
    struct gxp_timer *timer = to_gxp_timer(evt_dev);
// Stop counting and disable interrupt before updating
    writeb_relaxed(MASK_TCS_TC, timer.control);
    writel_relaxed(event, timer.counter);
    writeb_relaxed(MASK_TCS_TC | MASK_TCS_ENABLE, timer.control);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gxp_timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t gxp_timer_interrupt(int irq, void *dev_id)
    {
    struct gxp_timer *timer = (struct gxp_timer *)dev_id;
    if (!(readb_relaxed(timer.control) & MASK_TCS_TC))
    return IRQ_NONE;
    writeb_relaxed(MASK_TCS_TC, timer.control);
    timer.evt.event_handler(&timer.evt);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn gxp_timer_init(node: *mut device_node) -> int __init {
    static int __init gxp_timer_init(struct device_node *node)
    {
    void __iomem *base;
    struct clk *clk;
    u32 freq;
    int ret, irq;
    gxp_timer = kzalloc_obj(*gxp_timer);
    if (!gxp_timer) {
    ret = -ENOMEM;
    pr_err("Can't allocate gxp_timer");
    return ret;
    }
    clk = of_clk_get(node, 0);
    if (IS_ERR(clk)) {
    ret = PTR_ERR(clk);
    pr_err("%pOFn clock not found: %d\n", node, ret);
    goto err_free;
    }
    ret = clk_prepare_enable(clk);
    if (ret) {
    pr_err("%pOFn clock enable failed: %d\n", node, ret);
    goto err_clk_enable;
    }
    base = of_iomap(node, 0);
    if (!base) {
    ret = -ENXIO;
    pr_err("Can't map timer base registers");
    goto err_iomap;
    }
// Set the offsets to the clock register and timer registers
    gxp_timer.counter = base + GXP_TIMER_CNT_OFS;
    gxp_timer.control = base + GXP_TIMER_CTRL_OFS;
    system_clock = base + GXP_TIMESTAMP_OFS;
    gxp_timer.evt.name = node.name;
    gxp_timer.evt.rating = 300;
    gxp_timer.evt.features = CLOCK_EVT_FEAT_ONESHOT;
    gxp_timer.evt.set_next_event = gxp_time_set_next_event;
    gxp_timer.evt.cpumask = cpumask_of(0);
    irq = irq_of_parse_and_map(node, 0);
    if (irq <= 0) {
    ret = -EINVAL;
    pr_err("GXP Timer Can't parse IRQ %d", irq);
    goto err_exit;
    }
    freq = clk_get_rate(clk);
    ret = clocksource_mmio_init(system_clock, node.name, freq,
    300, 32, clocksource_mmio_readl_up);
    if (ret) {
    pr_err("%pOFn init clocksource failed: %d", node, ret);
    goto err_exit;
    }
    sched_clock_register(gxp_sched_read, 32, freq);
    irq = irq_of_parse_and_map(node, 0);
    if (irq <= 0) {
    ret = -EINVAL;
    pr_err("%pOFn Can't parse IRQ %d", node, irq);
    goto err_exit;
    }
    clockevents_config_and_register(&gxp_timer.evt, TIMER0_FREQ,
    0xf, 0xffffffff);
    ret = request_irq(irq, gxp_timer_interrupt, IRQF_TIMER | IRQF_SHARED,
    node.name, gxp_timer);
    if (ret) {
    pr_err("%pOFn request_irq() failed: %d", node, ret);
    goto err_exit;
    }
    pr_debug("gxp: system timer (irq = %d)\n", irq);
    return 0;
    err_exit:
    iounmap(base);
    err_iomap:
    clk_disable_unprepare(clk);
    err_clk_enable:
    clk_put(clk);
    err_free:
    kfree(gxp_timer);
    return ret;
    }
//
// This probe gets called after the timer is already up and running. This will create
// the watchdog device as a child since the registers are shared.
//
#[no_mangle]
unsafe extern "C" fn gxp_timer_probe(pdev: *mut platform_device) -> c_int {
    static int gxp_timer_probe(struct platform_device *pdev)
    {
    struct platform_device *gxp_watchdog_device;
    struct device *dev = &pdev.dev;
    int ret;
    if (!gxp_timer) {
    pr_err("Gxp Timer not initialized, cannot create watchdog");
    return -ENOMEM;
    }
    gxp_watchdog_device = platform_device_alloc("gxp-wdt", -1);
    if (!gxp_watchdog_device) {
    pr_err("Timer failed to allocate gxp-wdt");
    return -ENOMEM;
    }
// Pass the base address (counter) as platform data and nothing else
    gxp_watchdog_device.dev.platform_data = gxp_timer.counter;
    gxp_watchdog_device.dev.parent = dev;
    ret = platform_device_add(gxp_watchdog_device);
    if (ret)
    platform_device_put(gxp_watchdog_device);
    return ret;
    }
    static const struct of_device_id gxp_timer_of_match[] = {
    { .compatible = "hpe,gxp-timer", },
    {},
    };
    static struct platform_driver gxp_timer_driver = {
    .probe  = gxp_timer_probe,
    .driver = {
    .name = "gxp-timer",
    .of_match_table = gxp_timer_of_match,
    .suppress_bind_attrs = true,
    },
    };
    builtin_platform_driver(gxp_timer_driver);
    TIMER_OF_DECLARE(gxp, "hpe,gxp-timer", gxp_timer_init);
