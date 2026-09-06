//! Automatically rewritten from C to Rust
//! Source: drivers/clocksource/bcm2835_timer.c
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
// Copyright 2012 Simon Arlott
//

pub const REG_CONTROL: c_uint = 0x00;
pub const REG_COUNTER_LO: c_uint = 0x04;
pub const REG_COUNTER_HI: c_uint = 0x08;

pub const MAX_TIMER: c_int = 3;
pub const DEFAULT_TIMER: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm2835_timer {
    pub control: *mut void __iomem,
    pub compare: *mut void __iomem,
    pub match_mask: c_int,
    pub evt: clock_event_device,
}

    static void __iomem *system_clock __read_mostly;
#[no_mangle]
unsafe extern "C" fn bcm2835_sched_read() -> u64 notrace {
    static u64 notrace bcm2835_sched_read(void)
    {
    return readl_relaxed(system_clock);
    }
    static int bcm2835_time_set_next_event(unsigned long event,
    struct clock_event_device *evt_dev)
    {
    struct bcm2835_timer *timer = container_of(evt_dev,
    struct bcm2835_timer, evt);
    writel_relaxed(readl_relaxed(system_clock) + event,
    timer.compare);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_time_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t bcm2835_time_interrupt(int irq, void *dev_id)
    {
    struct bcm2835_timer *timer = dev_id;
    void (*event_handler)(struct clock_event_device *);
    if (readl_relaxed(timer.control) & timer.match_mask) {
    writel_relaxed(timer.match_mask, timer.control);
    event_handler = READ_ONCE(timer.evt.event_handler);
    if (event_handler)
    event_handler(&timer.evt);
    return IRQ_HANDLED;
    } else {
    return IRQ_NONE;
    }
    }
#[no_mangle]
unsafe extern "C" fn bcm2835_timer_init(node: *mut device_node) -> int __init {
    static int __init bcm2835_timer_init(struct device_node *node)
    {
    void __iomem *base;
    u32 freq;
    int irq, ret;
    struct bcm2835_timer *timer;
    base = of_iomap(node, 0);
    if (!base) {
    pr_err("Can't remap registers\n");
    return -ENXIO;
    }
    ret = of_property_read_u32(node, "clock-frequency", &freq);
    if (ret) {
    pr_err("Can't read clock-frequency\n");
    goto err_iounmap;
    }
    system_clock = base + REG_COUNTER_LO;
    sched_clock_register(bcm2835_sched_read, 32, freq);
    clocksource_mmio_init(base + REG_COUNTER_LO, node.name,
    freq, 300, 32, clocksource_mmio_readl_up);
    irq = irq_of_parse_and_map(node, DEFAULT_TIMER);
    if (irq <= 0) {
    pr_err("Can't parse IRQ\n");
    ret = -EINVAL;
    goto err_iounmap;
    }
    timer = kzalloc_obj(*timer);
    if (!timer) {
    ret = -ENOMEM;
    goto err_iounmap;
    }
    timer.control = base + REG_CONTROL;
    timer.compare = base + REG_COMPARE(DEFAULT_TIMER);
    timer.match_mask = BIT(DEFAULT_TIMER);
    timer.evt.name = node.name;
    timer.evt.rating = 300;
    timer.evt.features = CLOCK_EVT_FEAT_ONESHOT;
    timer.evt.set_next_event = bcm2835_time_set_next_event;
    timer.evt.cpumask = cpumask_of(0);
    ret = request_irq(irq, bcm2835_time_interrupt, IRQF_TIMER | IRQF_SHARED,
    node.name, timer);
    if (ret) {
    pr_err("Can't set up timer IRQ\n");
    goto err_timer_free;
    }
    clockevents_config_and_register(&timer.evt, freq, 0xf, 0xffffffff);
    pr_info("bcm2835: system timer (irq = %d)\n", irq);
    return 0;
    err_timer_free:
    kfree(timer);
    err_iounmap:
    iounmap(base);
    return ret;
    }
    TIMER_OF_DECLARE(bcm2835, "brcm,bcm2835-system-timer",
    bcm2835_timer_init);
